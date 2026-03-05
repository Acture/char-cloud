use crate::core::error::CharCloudError;
use crate::layout::common::{
	apply_candidate, available_positions, candidate_quality, create_progress_bar, finish_progress,
	pick_color, random_unit_f32, sample_candidate, total_area, update_progress,
};
use crate::layout::{LayoutRequest, LayoutResult, LayoutStrategy};
use rand::RngCore;

const CANDIDATE_TRIALS: usize = 48;
const INITIAL_TEMPERATURE: f32 = 1.0;
const MIN_TEMPERATURE: f32 = 0.02;
const COOLING_RATE: f32 = 0.996;
const POOL_REFILL_THRESHOLD: usize = 256;

pub struct SimulatedAnnealingStrategy;

impl LayoutStrategy for SimulatedAnnealingStrategy {
	fn place(
		&self,
		request: &LayoutRequest<'_>,
		rng: &mut dyn RngCore,
	) -> Result<LayoutResult, CharCloudError> {
		let mut mask = request.mask.clone();
		let total_usable_area = total_area(&mask);
		if total_usable_area == 0 {
			return Err(CharCloudError::Generation(
				"shape mask has no usable area".to_string(),
			));
		}

		let mut positions = available_positions(&mask);
		let mut placements = Vec::new();
		let mut attempts = 0usize;
		let mut used_area = 0usize;
		let mut current_score = 0.0f32;
		let mut temperature = INITIAL_TEMPERATURE;
		let progress = create_progress_bar(request.show_progress);

		while attempts < request.max_try_count {
			let fill_ratio = used_area as f32 / total_usable_area as f32;
			if fill_ratio >= request.ratio_threshold {
				break;
			}

			attempts += 1;

			if positions.len() < POOL_REFILL_THRESHOLD {
				positions = available_positions(&mask);
			}
			if positions.is_empty() {
				break;
			}

			let Some(candidate) =
				sample_candidate(&mask, &mut positions, request, rng, CANDIDATE_TRIALS)
			else {
				temperature = (temperature * COOLING_RATE).max(MIN_TEMPERATURE);
				continue;
			};

			let candidate_score = candidate_quality(&candidate, total_usable_area);
			let delta = candidate_score - current_score;
			let accepted = if delta >= 0.0 {
				true
			} else {
				let acceptance = (delta / temperature.max(1e-6)).exp();
				random_unit_f32(rng) < acceptance
			};

			if accepted {
				let color = pick_color(&request.style.colors, rng);
				let (placed, consumed) = apply_candidate(&mut mask, &candidate, color);
				used_area += consumed;
				current_score = candidate_score;
				placements.push(placed);
			}

			temperature = (temperature * COOLING_RATE).max(MIN_TEMPERATURE);

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
