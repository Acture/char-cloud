use crate::core::error::GlyphWeaveError;
use crate::core::model::Rotation;
use crate::layout::common::{
	Rect, available_positions, create_progress_bar, descending_font_sizes, finish_progress,
	intersects, occupy_area, pick_color, pick_weighted_word, placement, random_index, total_area,
	update_progress,
};
use crate::layout::{LayoutRequest, LayoutResult, LayoutStrategy};
use crate::mask::calculate_text_size;
use ndarray::Array2;
use rand::RngCore;

const CANDIDATE_TRIALS: usize = 48;
const INTEGRAL_REBUILD_INTERVAL: usize = 64;
const POOL_REFILL_THRESHOLD: usize = 512;

pub struct FastGridStrategy;

impl LayoutStrategy for FastGridStrategy {
	fn place(
		&self,
		request: &LayoutRequest<'_>,
		rng: &mut dyn RngCore,
	) -> Result<LayoutResult, GlyphWeaveError> {
		let mut mask = request.mask.clone();
		let total_usable_area = total_area(&mask);
		if total_usable_area == 0 {
			return Err(GlyphWeaveError::Generation(
				"shape mask has no usable area".to_string(),
			));
		}

		let mut integral = build_integral(&mask);
		let mut pending_rects: Vec<Rect> = Vec::new();
		let mut positions = available_positions(&mask);
		let mut placements = Vec::new();
		let mut attempts = 0usize;
		let mut used_area = 0usize;
		let progress = create_progress_bar(request.show_progress);

		while attempts < request.max_try_count {
			let fill_ratio = used_area as f32 / total_usable_area as f32;
			if fill_ratio >= request.ratio_threshold {
				break;
			}

			attempts += 1;
			let Some(word_entry) = pick_weighted_word(request.words, rng) else {
				break;
			};

			let mut placed = false;
			for _ in 0..CANDIDATE_TRIALS {
				if positions.is_empty() {
					break;
				}

				let idx = random_index(rng, positions.len());
				let (y, x) = positions[idx];
				if !mask[[y, x]] {
					positions.swap_remove(idx);
					continue;
				}

				let context = FastFitContext {
					integral: &integral,
					pending_rects: &pending_rects,
					mask: &mask,
					style: request.style,
					font: request.font,
				};
				if let Some((font_size, rotation, rect)) =
					find_fit_with_integral(&context, x, y, &word_entry.text)
				{
					used_area += occupy_area(&mut mask, rect);
					pending_rects.push(rect);
					let color = pick_color(&request.style.colors, rng);
					placements.push(placement(
						&word_entry.text,
						rect,
						font_size,
						color,
						rotation,
					));
					placed = true;

					if pending_rects.len() >= INTEGRAL_REBUILD_INTERVAL {
						integral = build_integral(&mask);
						pending_rects.clear();
					}

					break;
				}
			}

			if !placed && positions.len() < POOL_REFILL_THRESHOLD {
				positions = available_positions(&mask);
			}

			let ratio_progress = (used_area * 100) / total_usable_area;
			let try_progress = (attempts * 100) / request.max_try_count;
			update_progress(&progress, ratio_progress.max(try_progress));
		}

		finish_progress(&progress);

		Ok(LayoutResult {
			placements,
			attempts,
			used_area,
		})
	}
}

struct FastFitContext<'a> {
	integral: &'a Array2<u32>,
	pending_rects: &'a [Rect],
	mask: &'a Array2<bool>,
	style: &'a crate::core::model::StyleConfig,
	font: &'a fontdue::Font,
}

fn find_fit_with_integral(
	context: &FastFitContext<'_>,
	x: usize,
	y: usize,
	word: &str,
) -> Option<(usize, Rotation, Rect)> {
	for size in descending_font_sizes(context.style) {
		for rotation in &context.style.rotations {
			let (w, h) =
				calculate_text_size(word, context.font, size, context.style.padding, *rotation);
			let rect = Rect { x, y, w, h };
			if is_area_available_fast(context.integral, context.pending_rects, context.mask, rect) {
				return Some((size, *rotation, rect));
			}
		}
	}

	None
}

fn is_area_available_fast(
	integral: &Array2<u32>,
	pending_rects: &[Rect],
	mask: &Array2<bool>,
	rect: Rect,
) -> bool {
	if rect.w == 0 || rect.h == 0 {
		return false;
	}

	if rect.x + rect.w > mask.ncols() || rect.y + rect.h > mask.nrows() {
		return false;
	}

	let area = rect.area() as u32;
	if rect_sum(integral, rect) != area {
		return false;
	}

	!pending_rects
		.iter()
		.any(|pending| intersects(*pending, rect))
}

fn build_integral(mask: &Array2<bool>) -> Array2<u32> {
	let rows = mask.nrows();
	let cols = mask.ncols();
	let mut integral = Array2::<u32>::zeros((rows + 1, cols + 1));

	for y in 0..rows {
		for x in 0..cols {
			let value = if mask[[y, x]] { 1 } else { 0 };
			integral[[y + 1, x + 1]] =
				value + integral[[y, x + 1]] + integral[[y + 1, x]] - integral[[y, x]];
		}
	}

	integral
}

fn rect_sum(integral: &Array2<u32>, rect: Rect) -> u32 {
	let x1 = rect.x;
	let y1 = rect.y;
	let x2 = rect.x + rect.w;
	let y2 = rect.y + rect.h;

	integral[[y2, x2]] + integral[[y1, x1]] - integral[[y1, x2]] - integral[[y2, x1]]
}
