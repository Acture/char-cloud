use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlyphWeaveError {
	#[error("invalid configuration: {0}")]
	InvalidConfig(String),

	#[error("font loading failed: {0}")]
	FontLoad(String),

	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error),

	#[error("image error: {0}")]
	Image(#[from] image::ImageError),

	#[error("generation failed: {0}")]
	Generation(String),
}
