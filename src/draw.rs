use crate::mask::{calculate_mask, calculate_text_size, CanvasConfig, ShapeConfig};
use crate::{mask, utils};
use derive_builder::Builder;
use fontdue::Font;
use log::{debug, info, trace};
use ndarray::Array2;
use rand::seq::IteratorRandom;
use std::ops::RangeInclusive;
use svg::node::element::SVG;
use svg::Document;


#[derive(Debug, Clone, Builder)]
pub struct FillConfig {
	pub words: Vec<String>,
	pub font: Font,
	pub font_size_range: RangeInclusive<usize>,
	pub padding: usize,
	pub colors: Vec<String>,
}

#[derive(Debug, Clone, Builder)]
pub struct DrawConfig {
	pub canva_config: CanvasConfig,
	pub shape_config: ShapeConfig,
	pub fill_config: FillConfig,
	pub ratio_threshold: f32,
	pub max_try_count: usize,
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

fn create_text_element(x: usize, y: usize, word: &str, font: &Font, font_size: usize, color: &str) -> svg::node::element::Text {
	svg::node::element::Text::new(word)
		.set("x", x)
		.set("y", y)
		.set("font-family", utils::get_font_family_name(font))
		.set("font-size", font_size)
		.set("fill", color)
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

fn is_area_available(
	mask_tensor: &Array2<bool>,
	x: usize,
	y: usize,
	text_width: usize,
	text_height: usize,
) -> bool {
	for dy in 0..text_height {
		for dx in 0..text_width {
			if !mask_tensor[[y + dy, x + dx]] {
				return false;
			}
		}
	}
	true
}

fn find_possible_font_size<I: Iterator<Item = usize>>(
	word: &String,
	x: usize,
	y: usize,
	mask_tensor: &mut Array2<bool>,
	font: &Font,
	font_size_range: I,
	padding: usize,
) -> Option<(usize, usize, usize)> {
	for font_size in font_size_range {
		// 计算文本尺寸
		let (text_width, text_height) = calculate_text_size(word, font, font_size.into(), padding);

		// 检查是否超出边界
		if x + text_width > mask_tensor.ncols() || y + text_height > mask_tensor.nrows() {
			continue;
		}

		// 检查区域是否可用
		if is_area_available(mask_tensor, x, y, text_width, text_height) {
			return Some((font_size, text_width, text_height));
		}
	}

	trace!("未找到合适的位置或字体大小");
	None
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

	while ratio < config.ratio_threshold && try_count < config.max_try_count {
		debug!("Current ratio: {:.2}/{:.2}, Try count: {}/{}", ratio, config.ratio_threshold, try_count, config.max_try_count);
		let available_positions = get_available_positions(&mask_tensor);
		if available_positions.is_empty() {
			info!("No available positions left to fill text");
			break;
		}

		// 随机选择一个可用位置
		let chosen_position = available_positions.iter().choose(&mut rng);

		if chosen_position.is_none() {
			info!("No available positions found for filling text");
		}

		let &(y, x) = chosen_position.expect("No available positions found");
		let word = config.fill_config.words.iter().choose(&mut rng).expect("No words available");

		let chosen_font = find_possible_font_size(	word, x, y, &mut mask_tensor,
													&config.fill_config.font,
													config.fill_config.font_size_range.clone().rev(),
													config.fill_config.padding,
		);

		if chosen_font.is_none() {
			trace!("No suitable font size found for word '{}'", word);
			try_count += 1;
			continue;
		}

		let (font_size, text_width, text_height) = chosen_font.expect("Failed to find font size");
		debug!("Filling word '{}' at position ({}, {}) with font size {}", word, x, y, font_size);
		let color = config.fill_config.colors.iter().choose(&mut rng).expect("No colors available");
		canvas = canvas.add(create_text_element(x, y, word, &config.fill_config.font, font_size, color));
		// 更新可用区域
		update_mask(&mut mask_tensor, (y, x), (text_width, text_height));


		// 计算当前可用区域的比例
		current_usable_area = mask_tensor.iter().filter(|&&value| value).count();
		ratio = calculate_fill_ratio(current_usable_area, total_usable_area);
		try_count += 1;
	}

	canvas
}


#[cfg(test)]
mod tests {
	use super::*;
use crate::mask::{calculate_auto_font_size, FontSize, ShapeConfig};

	#[test]
	fn test_draw() {
		let font = utils::load_font_from_file("fonts/Roboto-Regular.ttf")
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
			font_size_range: 10usize..=30usize,
			padding: 0,
			colors: vec![
				"#FF5733".to_string(), // 红色
				"#33FF57".to_string(), // 绿色
				"#3357FF".to_string(), // 蓝色
				"#FFFF33".to_string(), // 黄色
				"#FF33FF".to_string(), // 品红
				"#33FFFF".to_string(), // 青色
			],
		};
		let config = DrawConfig {
			canva_config: canvas_config,
			shape_config,
			fill_config,
			ratio_threshold: 0.5,
			max_try_count: 1000,
		};

		let svg = draw(&config);
		assert!(!svg.to_string().is_empty());

		svg::save(
			"test_output.svg",
			&svg,
		).expect("Failed to save SVG file");
	}
}