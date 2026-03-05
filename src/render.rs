use crate::core::model::{CanvasConfig, CloudPlacement, Rotation};
use svg::Document;
use svg::node::element::Text;

pub fn render_svg(
    canvas: &CanvasConfig,
    placements: &[CloudPlacement],
    font_family: &str,
) -> String {
    let mut doc = Document::new()
        .set("width", canvas.width)
        .set("height", canvas.height)
        .set("viewBox", (0, 0, canvas.width, canvas.height))
        .set("xmlns", "http://www.w3.org/2000/svg")
        .set("xmlns:xlink", "http://www.w3.org/1999/xlink");

    for placement in placements {
        let mut element = Text::new(&placement.word)
            .set("x", placement.x)
            .set("y", placement.y)
            .set("font-family", font_family)
            .set("font-size", placement.font_size)
            .set("fill", placement.color.as_str())
            .set("dominant-baseline", "hanging")
            .set("text-anchor", "start");

        if placement.rotation == Rotation::Deg90 {
            element = element.set(
                "transform",
                format!("rotate(90 {} {})", placement.x, placement.y),
            );
        }

        doc = doc.add(element);
    }

    doc.to_string()
}
