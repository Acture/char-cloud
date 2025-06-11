mod mask;
mod draw;
mod utils;
mod args;
mod embedded_fonts;

use crate::args::CliArgs;
use crate::draw::{draw, DrawConfigBuilder, FillConfigBuilder};
use crate::mask::{calculate_auto_font_size, CanvasConfigBuilder, FontSize, ShapeConfigBuilder};
use clap::Parser;
use env_logger::Builder;
use fontdue::Font;
use log::{debug, error};

fn main() {
	let cli_args = CliArgs::parse();

	let log_level = if cfg!(debug_assertions) {
		match cli_args.verbose {
			true => log::LevelFilter::Debug,
			false => log::LevelFilter::Info,
		}
	} else {
		match cli_args.verbose {
			true => log::LevelFilter::Debug,
			false => log::LevelFilter::Warn,
		}
	};

	Builder::from_default_env()
		.filter_level(log_level)
		.format_level(true)
		.format_timestamp_secs()
		.format_module_path(true)
		.format_line_number(true)
		.format_target(true)
		.format_indent(Some(4))
		.init();

	let font = match cli_args.font_path {
		Some(path) => {
			debug!("Loading font from path: {:?}", path);
			utils::load_font_from_file(path)
				.expect("Failed to load font from specified path")
		}
		None => {
			if cfg!(feature = "embedded_fonts") {
				debug!("No font path provided, using default embedded font for debug mode");
				Font::from_bytes(crate::embedded_fonts::NOTO_SANS_SC_REGULAR, fontdue::FontSettings::default())
					.expect("Failed to load default font")
			} else {
				error!("No font path provided and embedded fonts feature is not enabled. Please provide a valid font file path or enable the embedded fonts feature.");
				panic!("No font available");
			}
		}
	};


	let canvas_config = CanvasConfigBuilder::default()
		.width(cli_args.canva_size.0)
		.height(cli_args.canva_size.1)
		.margin(cli_args.canva_margin)
		.build()
		.expect("Failed to create <canvas> configuration");

	let shape_config = match cli_args.shape_size {
		FontSize::Fixed(size) => {
			ShapeConfigBuilder::default()
				.text(cli_args.shape_text)
				.font(&font)
				.font_size(FontSize::Fixed(size))
				.build()
		}
		FontSize::AutoFit => {
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
		.ratio_threshold(cli_args.threshold)
		.max_try_count(cli_args.max_tries)
		.build()
		.expect("Failed to create <draw> configuration");


	let svg = draw(&draw_config);

	if svg.to_string().is_empty() {
		log::error!("Generated SVG is empty, please check your configuration.");
		return;
	}

	log::info!("SVG generated successfully, saving to file...");

	let output_path = cli_args.output;

	svg::save(
		output_path,
		&svg,
	).expect("Failed to save SVG file");
}
