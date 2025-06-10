use derive_builder::Builder;
use fontdue::Font;
use image::{ImageBuffer, Rgba};
use ndarray::Array2;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct CanvasConfig {
	#[builder(default = "1920")]
	pub width: usize,
	#[builder(default = "1080")]
	pub height: usize,
	#[builder(default = "10")]
	pub padding: usize,
}

#[derive(Debug, Clone)]
pub enum FontSize {
	Fixed(usize),
	AutoFit,
}

impl From<usize> for FontSize {
	fn from(size: usize) -> Self {
		FontSize::Fixed(size)
	}
}

impl From<FontSize> for usize {
	fn from(size: FontSize) -> Self {
		match size {
			FontSize::Fixed(size) => size,
			FontSize::AutoFit => panic!("Cannot convert AutoFit to usize"),
		}
	}
}




#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct ShapeConfig<'a> {
	pub text: String,
	pub font: &'a Font,
	#[builder(default = "FontSize::AutoFit")]
	pub font_size: FontSize,
}



impl ShapeConfig<'_> {
	pub fn get_font_size(&self) -> usize {
		match self.font_size {
			FontSize::Fixed(size) => size,
			FontSize::AutoFit => {
				panic!("Font size is set to AutoFit but no size has been calculated yet. Call calculate_auto_fit_size first.");
			}
		}
	}



}

pub fn calculate_text_size<S: AsRef<str>>(string: &S, font: &Font, font_size: FontSize, padding: usize) -> (usize, usize) {
	let metrics_list: Vec<_> = string.as_ref().chars()
		.map(|c| font.metrics(c,usize::from(font_size.clone()) as f32))
		.collect();

	let total_width = metrics_list.iter().map(|m| m.advance_width).sum::<f32>() as usize + 2 * padding;
	let max_height = metrics_list.iter().map(|m| m.height).max().unwrap_or(0) as usize+ 2 * padding;

	(total_width, max_height)
}

pub fn calculate_auto_font_size<S: AsRef<str>>(canvas_config: &CanvasConfig, text:S, font: &Font  ) -> usize {

	let available_width = canvas_config.width.saturating_sub(2 * canvas_config.padding);
	let available_height = canvas_config.height.saturating_sub(2 * canvas_config.padding);

	let mut low = 1;
	let mut high = available_height;
	let mut best_size = low;
	while high > low {
		let mid = (low + high) / 2;
		let metrics = text.as_ref().chars()
			.map(|c| font.metrics(c, mid as f32))
			.collect::<Vec<_>>();
		let total_width = metrics.iter().map(|m| m.advance_width).sum::<f32>() as usize;
		let max_height = metrics.iter().map(|m| m.height).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0) as usize;
		if total_width <= available_width && max_height <= available_height {
			best_size = mid;
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}
	best_size
}


pub fn calculate_mask(canvas: &CanvasConfig, shape: &ShapeConfig) -> Array2<bool> {
	let height = canvas.height;
	let width = canvas.width;
	// 创建结果数组
	let mut result = Array2::from_elem((height, width), false); // 初始化为 false
	
	// 获取字体大小
	let metrics_list: Vec<_> = shape.text.chars()
		.map(|c| shape.font.metrics(c, shape.get_font_size() as f32))
		.collect();
	// 计算总宽度和最大高度
	let total_width = metrics_list.iter().map(|m| m.advance_width).sum::<f32>() as usize;
	let max_height = metrics_list.iter().map(|m| m.height).max().unwrap_or(0) as usize;

	let offset_x = canvas.padding + (width.saturating_sub(2 * canvas.padding).saturating_sub(total_width)) / 2;
	let offset_y = canvas.padding + (height.saturating_sub(2 * canvas.padding).saturating_sub(max_height)) / 2;

	let mut current_x = offset_x;
	for (c, metrics) in shape.text.chars().zip(metrics_list.iter()) {
		let (_, bitmap) = shape.font.rasterize(c, shape.get_font_size() as f32);
		let glyph_w = metrics.width;
		let glyph_h = metrics.height;

		for y in 0..glyph_h {
			for x in 0..glyph_w {
				let pixel = bitmap[y * glyph_w + x];
				if current_x + x < width && offset_y + y < height && pixel > 127 {
					result[[offset_y + y, current_x + x]] = true;
				}
			}
		}

		current_x += metrics.advance_width.ceil() as usize;
	}
	result
}


pub fn mask_to_image(map: &Array2<bool>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	let (height, width) = map.dim();
	let mut image = ImageBuffer::new(width as u32, height as u32);

	for ((y, x), &occupied) in map.indexed_iter() {
		image.put_pixel(x as u32, y as u32,
						if occupied {
							Rgba([255, 255, 255, 255])  // 白色，完全不透明
						} else {
							Rgba([0, 0, 0, 0])  // 完全透明
						},
		);
	}

	image
}
#[cfg(test)]
mod tests {
	use super::*;
	use fontdue::FontSettings;


	#[test]
	fn test_mask() {
		// 加载字体数据（使用英文字体避免 fallback 错误）
		let font_data = std::fs::read("fonts/Roboto-Regular.ttf")
			.expect("Failed to load font file");
		let font = Font::from_bytes(font_data, FontSettings::default())
			.expect("Failed to parse font");

		let canvas = CanvasConfig {
			width: 1920,
			height: 1080,
			padding: 10,
		};

		let mut shape = ShapeConfig {
			text: "BRICS".to_string(), // 建议测试英文以确保字体支持
			font,
			font_size: FontSize::AutoFit,
		};

		let font_size = calculate_auto_font_size(&shape, &canvas);

		shape.font_size = FontSize::Fixed(font_size);


		let mask = calculate_mask(&canvas, &shape);

		// 基本尺寸断言
		assert_eq!(mask.shape(), &[canvas.height, canvas.width]);

		// 遮罩中应有非零像素（即至少有文字绘制）
		assert!(mask.iter().any(|&x| x), "Mask should contain some occupied pixels");

		let image = mask_to_image(&mask);
		image.save("test_output_mask.png").expect("Failed to save test mask image");
	}
}