//! Style Attributes for the Drawables System'

/// Simple RGB Color
/// TODO: is there a better option for this?
pub struct Color {
    /// red channel
    pub red: u8,
    /// green channel
    pub green: u8,
    /// blue channel
    pub blue: u8,
}

impl Color {
    /// Black Color
    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    /// Convert to a string suitable for SVG
    pub fn svg_rgb(&self) -> String {
        format!("rgb({},{},{})", self.red, self.green, self.blue)
    }
}

/// Default Color is Black
impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

/// Style Attributes for Drawables Shapes
pub struct ShapeStyle {
    /// Width of Stroke (None is no stroke)
    pub stroke_width: Option<f32>,
    /// Stroke Color
    pub stroke: Color,
    /// Fill Color (None is no fill)
    /// TODO: this is inconsistent with stroke
    pub fill: Option<Color>,
    /// Fill Opacity
    pub fill_opacity: f32,
    /// Stroke opacity
    pub stroke_opacity: f32,
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            stroke_width: Some(1.0),
            stroke: Color::black(),
            fill: None,
            fill_opacity: 1.0,
            stroke_opacity: 1.0,
        }
    }
}

/// Text Attributes for Drawables Shapes
#[derive(Default)]
pub struct TextStyle {
    /// fixed font size (None means the shape can resize)
    pub font_size: Option<f32>,
}
