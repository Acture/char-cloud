mod mask;
mod draw;
mod utils;

use std::path::PathBuf;
use env_logger::Builder;
use crate::draw::{draw, DrawConfig, FillConfig};
use crate::mask::{calculate_auto_font_size, CanvasConfig, FontSize, ShapeConfig};

fn main() {
	Builder::from_default_env()
		.filter_level(log::LevelFilter::Debug)
		.format_level( true)
		.format_timestamp_secs()
		.format_module_path(true)
		.format_line_number(true)
		.format_target(true)
		.format_indent(Some(4))
		.init();

	let font = utils::load_font_from_file(PathBuf::from("fonts/Roboto-Regular.ttf"))
		.expect("Failed to load font");
	let mut shape_config = ShapeConfig {
		text:"ASBSBSB".to_string(), 
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
