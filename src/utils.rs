use fontdue::{Font, FontSettings};
use std::path::Path;

pub fn get_font_family_name(font: &Font) -> String {
	// 直接获取字体的第一个名称作为字体族名称
	font.name()
		.unwrap_or("Unknown")
		.to_string()
}

pub fn load_font_from_file<P: AsRef<Path>>(path: P) -> Result<Font, String> {
	let font_data = std::fs::read(path.as_ref())
		.expect("Failed to load font file");

	Ok(Font::from_bytes(font_data, FontSettings::default())
		.expect("Failed to parse font"))
}