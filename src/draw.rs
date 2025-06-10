use crate::mask::{calculate_auto_font_size, calculate_mask, calculate_text_size, CanvasConfig, ShapeConfig};
use crate::{mask, utils};
use derive_builder::Builder;
use fontdue::Font;
use log::{debug, info, trace};
use rand::seq::IteratorRandom;
use std::path::PathBuf;
use ndarray::Array2;
use svg::node::element::SVG;
use svg::Document;


#[derive(Debug, Clone, Builder)]
pub struct FillConfig {
	pub words: Vec<String>,
	pub font: Font,
	pub font_size_range: (usize, usize),
	pub padding: usize,
}

#[derive(Debug, Clone, Builder)]
pub struct DrawConfig {
	pub canva_config: CanvasConfig,
	pub shape_config: ShapeConfig,
	pub fill_config: FillConfig,
	pub ratio_threshold: f32,
	pub try_count: usize,
}

fn init_canvas(config: &DrawConfig) -> Document {
	Document::new()
		.set("width", config.canva_config.width)
		.set("height", config.canva_config.height)
		.set("viewBox", (0, 0, config.canva_config.width, config.canva_config.height))
		.set("xmlns", "http://www.w3.org/2000/svg")
		.set("xmlns:xlink", "http://www.w3.org/1999/xlink")
}

fn get_available_positions(mask_tensor: &Array2<bool>) -> Vec<(usize, usize)> {
	mask_tensor
		.indexed_iter()
		.filter(|&(_, &value)| value)
		.map(|((y, x), _)| (y, x))
		.collect()
}

fn create_text_element(x: usize, y: usize, word: &str, font: &Font, font_size: usize) -> svg::node::element::Text {
	svg::node::element::Text::new(word)
		.set("x", x)
		.set("y", y)
		.set("font-family", utils::get_font_family_name(font))
		.set("font-size", font_size)
		.set("fill", "black")
}

fn update_mask(
	mask_tensor: &mut Array2<bool>,
	position: (usize, usize),
	text_size: (usize, usize),
) {
	let (y, x) = position;
	let (text_width, text_height) = text_size;

	for dy in 0..text_height {
		for dx in 0..text_width {
			if let Some(value) = mask_tensor.get_mut((y + dy, x + dx)) {
				*value = false;
			}
		}
	}
}
fn calculate_fill_ratio(current_area: usize, total_area: usize) -> f32 {
	1f32 - current_area as f32 / total_area as f32
}
pub fn draw(config: &DrawConfig) -> SVG {
	let mut canvas = init_canvas(config); 
	let mut mask_tensor = calculate_mask(&config.canva_config, &config.shape_config);
	let image = mask::mask_to_image(&mask_tensor);
	image.save("mask_image.png").expect("Failed to save mask image");
	let total_usable_area = mask_tensor.iter().filter(|&&value| value).count();
	let mut current_usable_area = total_usable_area;
	info!("Total Usable area: {}", total_usable_area);
	let mut ratio = calculate_fill_ratio(current_usable_area, total_usable_area);
	let mut try_count = 0;
	let mut rng = rand::rng();

	while ratio < config.ratio_threshold && try_count < config.try_count {
		debug!("Current ratio: {:.2}/{:.2}, Try count: {}/{}", ratio, config.ratio_threshold, try_count, config.try_count);
		let available_positions = get_available_positions(&mask_tensor); 
		if available_positions.is_empty() {
			break;
		}

		if let Some(&(y, x)) = available_positions.iter().choose(&mut rng) {
			let word = config.fill_config.words.iter().choose(&mut rng).expect("No words available");
			let font_size = 20;
			let (text_width, text_height) = calculate_text_size(word, &config.fill_config.font, font_size.into());
			canvas = canvas.add(create_text_element(x, y, word, &config.fill_config.font, font_size));
			update_mask(&mut mask_tensor, (y, x), (text_width, text_height));
			current_usable_area = mask_tensor.iter().filter(|&&value| value).count();
			ratio = calculate_fill_ratio(current_usable_area, total_usable_area);
		} else {
			trace!("No available position found for filling text");
		}

		try_count += 1;
	}

	canvas
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::mask::{FontSize, ShapeConfig};

	#[test]
	fn test_draw() {
		let font = utils::load_font_from_file(PathBuf::from("fonts/Roboto-Regular.ttf"))
			.expect("Failed to load font");
		let mut shape_config = ShapeConfig {
			text: "测试".to_string(),
			font: font.clone(),
			font_size: FontSize::AutoFit,
		};
		let canvas_config = CanvasConfig {
			width: 1920,
			height: 1080,
			padding: 10,
		};

		let font_size = calculate_auto_font_size(&shape_config, &canvas_config);
		shape_config.font_size = FontSize::Fixed(font_size);


		let fill_config = FillConfig {
			words: vec!["测试".to_string(), "绘图".to_string()],
			font,
			font_size_range: (10, 30),
			padding: 0,
		};
		let config = DrawConfig {
			canva_config: canvas_config,
			shape_config,
			fill_config,
			ratio_threshold: 0.5,
			try_count: 1000,
		};

		let svg = draw(&config);
		assert!(!svg.to_string().is_empty());

		svg::save(
			"test_output.svg",
			&svg,
		).expect("Failed to save SVG file");
	}
}