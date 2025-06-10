use crate::mask::FontSize;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
	#[arg(long="canva-size", default_value = "1920,1080", value_parser = parse_tuple)]
	pub canva_size: (usize, usize),

	#[arg(long = "canva-margin", default_value = "10", help = "Margin around the canvas in pixels. Default is 10.")]
	pub canva_margin: usize,

	#[arg(long = "words", required = true,value_delimiter = ',', help = "List of words to be used in the word cloud. This is a required argument.")]
	pub words: Vec<String>,
	#[arg(long = "word_size_range", default_value = "10,30", value_parser = parse_tuple, help = "Range of font sizes for the words, specified as MIN,MAX. Default is '10,30'.")]
	pub word_size_range: (usize, usize),

	#[arg(long = "colors", default_value = "black, red, green, blue", value_delimiter = ',', help = "Comma-separated list of colors for the words. Default is 'black, red, green, blue'.")]
	pub word_colors: Vec<String>,

	#[arg(short = 't', long = "text", required = true, help = "Text to be used as the shape. This text will be rendered in the specified font and size.")]
	pub shape_text: String,

	#[arg(long="text-size", default_value = "AutoFit", value_parser = parse_shape_size, help = "Size of the shape text. Use 'AutoFit' for automatic sizing or specify a fixed size.")]
	pub shape_size: FontSize,

	#[arg(short = 'v', long = "verbose", default_value = "false", help = "Enable verbose logging")]
	pub verbose: bool,

	#[arg(long = "font", help = "Path to the font file. If not provided, a default font will be used.", required = false)]
	pub font_path: Option<PathBuf>,

	#[arg(short = 'm', long = "max-tries", default_value = "10000", help = "Maximum number of attempts to place words in the canvas. Default is 1000.")]
	pub max_tries: usize,

	#[arg(short = 'r', long = "ratio", default_value = "0.9", help = "Threshold ratio for word placement. Default is 0.5.")]
	pub threshold: f32,

	#[arg(short = 'o', long = "output", help = "Output SVG file path.")]
	pub output: PathBuf,
}

fn parse_tuple(s: &str) -> Result<(usize, usize), String> {
	let parts: Vec<&str> = s.split(',').collect();
	if parts.len() != 2 {
		return Err("Size must be in the format WIDTH,HEIGHT".to_string());
	}
	let width = parts[0].parse::<usize>().map_err(|_| "Invalid width".to_string())?;
	let height = parts[1].parse::<usize>().map_err(|_| "Invalid height".to_string())?;
	Ok((width, height))
}

fn parse_shape_size(s: &str) -> Result<FontSize, String> {
	match s {
		"AutoFit" => Ok(FontSize::AutoFit),
		_ => {
			let size = s.parse::<usize>().map_err(|_| "Invalid font size".to_string())?;
			Ok(FontSize::Fixed(size))
		}
	}
}

