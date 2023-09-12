//! Drawables to Format

use svg::{node::element, Document};

use crate::codegen::frame_diagram::drawable::Drawable;
use crate::codegen::frame_diagram::style::{ShapeStyle, TextStyle};
use crate::codegen::frame_diagram::{
    drawable::{Canvas, CompositeDrawable, Rectangle},
    frame::PacketFrame,
};

/// SVG conversion traits
pub trait ToSvg {
    /// convert self into a SVG document
    fn to_svg(&self) -> Document;
}

impl ToSvg for CompositeDrawable {
    fn to_svg(&self) -> Document {
        // we pull the children of each to flattent the document
        let mut doc = Document::new().set("viewBox", self.get_viewbox_str());
        for item in &self.items {
            let mut item_doc = item.to_svg();
            for child in item_doc.get_children_mut().drain(..) {
                doc = doc.add(child);
            }
        }
        doc
    }
}

/// Apply SVG Style Based on Style Configuration Attributes
pub trait ApplyStyle<S> {
    /// Apply Style S to Self, creating a new self (builder pattern)
    fn apply_style(self, style: &S) -> Self;
}

/// Implement Shape Style for Rectangle
impl ApplyStyle<ShapeStyle> for element::Rectangle {
    fn apply_style(self, style: &ShapeStyle) -> Self {
        let mut node = self;
        node = node.set("stroke-opacity", style.stroke_opacity);
        node = match style.stroke_width {
            Some(sw) => node
                .set("stroke-width", sw)
                .set("stroke", style.stroke.svg_rgb()),
            None => node.set("stroke-width", 0.0),
        };
        node = match &style.fill {
            Some(c) => node
                .set("fill", c.svg_rgb())
                .set("fill-opacity", style.fill_opacity),
            None => node.set("fill-opacity", 0.0),
        };
        node
    }
}

/// Implement TextStyle for Text
impl ApplyStyle<TextStyle> for element::Text {
    fn apply_style(self, style: &TextStyle) -> Self {
        let mut node = self;
        node = match style.font_size {
            Some(fs) => node.set("font-size", fs),
            None => node,
        };
        node
    }
}

impl ToSvg for Rectangle {
    fn to_svg(&self) -> Document {
        let doc = Document::new().set("viewBox", self.get_viewbox_str());

        // Calculate the transformed top-left corner using the transformation matrix
        let top_left = self.transform.transform_point(&nalgebra::Point2::new(
            -self.width / 2.0,
            -self.height / 2.0,
        ));

        let vertical_size = (self.height / self.name.len() as f32).min(12.0);
        let horizontal_size = (self.width / self.name.len() as f32).min(12.0);

        let text_node = if horizontal_size >= vertical_size {
            element::Text::new()
                .set("x", (top_left.x + self.width / 2.0).to_string())
                .set("y", (top_left.y + self.height / 2.0).to_string())
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("font-family", "Monospace")
                .set("font-size", horizontal_size.to_string())
                .add(svg::node::Text::new(self.name.clone()))
                .apply_style(&self.text_style)
        } else {
            element::Text::new()
                .set("x", (top_left.x + self.width / 2.0).to_string())
                .set("y", (top_left.y + self.height / 2.0).to_string())
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("font-family", "Monospace")
                .set("font-size", vertical_size.to_string())
                .set("style", "writing-mode: tb;")
                .add(svg::node::Text::new(self.name.clone()))
        };

        doc.add(
            element::Rectangle::new()
                .set("x", top_left.x.to_string())
                .set("y", top_left.y.to_string())
                .set("width", self.width)
                .set("height", self.height)
                .apply_style(&self.style),
        )
        .add(text_node)
    }
}

impl ToSvg for Canvas {
    fn to_svg(&self) -> Document {
        // we pull the children of each to flattent the document
        let mut doc = Document::new().set("viewBox", self.get_viewbox_str());
        for item in &self.drawables {
            let mut item_doc = item.to_svg();
            for child in item_doc.get_children_mut().drain(..) {
                doc = doc.add(child);
            }
        }
        doc
    }
}

impl ToSvg for PacketFrame {
    fn to_svg(&self) -> Document {
        self.draw().to_svg()
    }
}
