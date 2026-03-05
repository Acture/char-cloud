use crate::core::error::CharCloudError;
use fontdue::{Font, FontSettings};
use std::path::Path;

pub fn font_family_name(font: &Font) -> String {
    font.name().unwrap_or("Unknown").to_string()
}

pub fn load_font_from_file<P: AsRef<Path>>(path: P) -> Result<Font, CharCloudError> {
    let path_ref = path.as_ref();
    let font_data = std::fs::read(path_ref).map_err(|err| {
        CharCloudError::FontLoad(format!(
            "failed to read font '{}': {err}",
            path_ref.display()
        ))
    })?;

    Font::from_bytes(font_data, FontSettings::default()).map_err(|err| {
        CharCloudError::FontLoad(format!(
            "failed to parse font '{}': {err}",
            path_ref.display()
        ))
    })
}

#[cfg(feature = "embedded_fonts")]
pub fn load_default_embedded_font() -> Result<Font, CharCloudError> {
    Font::from_bytes(
        crate::embedded_fonts::NOTO_SANS_SC_REGULAR,
        FontSettings::default(),
    )
    .map_err(|err| CharCloudError::FontLoad(format!("failed to parse embedded font: {err}")))
}

#[cfg(not(feature = "embedded_fonts"))]
pub fn load_default_embedded_font() -> Result<Font, CharCloudError> {
    Err(CharCloudError::FontLoad(
        "no font provided and embedded_fonts feature is disabled".to_string(),
    ))
}
