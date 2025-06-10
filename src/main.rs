mod mask;
mod draw;
mod utils;
mod args;

use crate::args::CliArgs;
use crate::draw::{draw, DrawConfigBuilder, FillConfigBuilder};
use crate::mask::{calculate_auto_font_size, CanvasConfigBuilder, FontSize, ShapeConfigBuilder};
use clap::Parser;
use env_logger::Builder;
use fontdue::Font;

static DEFAULT_FONT_DATA: &[u8] = include_bytes!("../fonts/NotoSansSC-Regular.ttf");

fn main() {
	let cli_args = CliArgs::parse();
	
	let log_level = if cli_args.verbose {
		log::LevelFilter::Debug
	} else {
		log::LevelFilter::Info
	};

	Builder::from_default_env()
		.filter_level(log_level)
		.format_level( true)
		.format_timestamp_secs()
		.format_module_path(true)
		.format_line_number(true)
		.format_target(true)
		.format_indent(Some(4))
		.init();

	let font = match cli_args.font_path {
		Some(path) => {
			utils::load_font_from_file(path)
				.expect("Failed to load font from specified path")
		}
		None => {

			Font::from_bytes(DEFAULT_FONT_DATA.to_vec(), fontdue::FontSettings::default())
				.expect("Failed to load default font")
		}
	};
		


	let canvas_config = CanvasConfigBuilder::default()
		.width(cli_args.canva_size.0)
		.height(cli_args.canva_size.1)
		.padding(cli_args.canva_padding)
		.build()
		.expect("Failed to create <canvas> configuration");

	let shape_config = match cli_args.shape_size {
		Some(size) => {
			ShapeConfigBuilder::default()
				.text(cli_args.shape_text)
				.font(&font)
				.font_size(FontSize::Fixed(size))
				.build()
		}
		None => {
			let font_size = calculate_auto_font_size(&canvas_config, &cli_args.shape_text, &font);
			
			ShapeConfigBuilder::default()
				.text(&cli_args.shape_text)
				.font(&font)
				.font_size(font_size)
				.build()
		}
	}.expect("Failed to create <shape> configuration");


	let fill_config = FillConfigBuilder::default()
		.words(cli_args.words)
		.font(&font)
		.font_size_range(cli_args.word_size_range.0..=cli_args.word_size_range.1)
		.padding(0)
		.colors(cli_args.word_colors)
		.build()
		.expect("Failed to create <fill> configuration");
	
	let draw_config = DrawConfigBuilder::default()
		.canva_config(canvas_config)
		.shape_config(shape_config)
		.fill_config(fill_config)
		.ratio_threshold(0.5)
		.max_try_count(1000)
		.build()
		.expect("Failed to create <draw> configuration");
		

	let svg = draw(&draw_config);
	
	if svg.to_string().is_empty() {
		log::error!("Generated SVG is empty, please check your configuration.");
		return;
	}

	svg::save(
		"test_output.svg",
		&svg,
	).expect("Failed to save SVG file");
}
