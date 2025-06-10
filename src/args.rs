use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs{
	#[arg(short, long, default_value = "1920,1080", value_parser = parse_tuple)]
	pub canva_size: (usize, usize),
	
	#[arg(short, long, default_value = "10")]
	pub canva_padding: usize,
	
	#[arg(short, long)]
	pub words: Vec<String>,
	#[arg(short, long, default_value = "10,30", value_parser = parse_tuple)]
	pub word_size_range: (usize, usize),
	
	#[arg(short, long, default_value = "black, red, green, blue")]
	pub word_colors: Vec<String>,
	
	
	#[arg(short, long)]
	pub shape_text: String,
	
	#[arg(short, long, default_value = "None")]
	pub shape_size: Option<usize>,
	
	#[arg(short, long, default_value = "False")]
	pub verbose: bool,
	
	#[arg(short, long, default_value = "None")]
	pub font_path: Option<PathBuf>,
	
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