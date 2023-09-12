//! Frame

use nalgebra::{Matrix3, Vector2};

use crate::codegen::frame_diagram::drawable::{CompositeDrawable, Drawable, Rectangle};

/// PacketFrame for Packet Frame diagram
pub struct PacketFrame {
    /// Name of the Frame Element
    pub name: String,
    /// Number of bits in the frame
    pub bits: usize,
    /// Child Frames sum(num of child bits) == bits
    pub children: Vec<PacketFrame>,
}

const PKT_HEIGHT: f32 = 20.0 * 3.0;
const PKT_LENGTH: f32 = 200.0 * 3.0;
const RULER_STROKE: f32 = 0.5;
const RULER_FONTSIZE: f32 = 10.0;
const PKT_STROKE: f32 = 0.8;

impl PacketFrame {
    /// Convert the PacketFrame into a Drawable
    pub fn draw(&self) -> CompositeDrawable {
        let scale = PKT_LENGTH / self.bits as f32;

        self.draw_scaled(scale, 0)
    }

    /// Recursive Method to Draw All Children PacketFrames
    pub fn draw_scaled(&self, scale: f32, bit_offset: usize) -> CompositeDrawable {
        let mut composite = CompositeDrawable::new();
        let size = self.bits as f32 * scale;

        // Create a rectangle representing this frame
        let mut rect = Rectangle::new(size, PKT_HEIGHT, self.name.clone());
        rect.style.stroke_width = Some(PKT_STROKE);
        composite.add(rect);

        let mut current_x = 0.0f32;
        let mut current_b = 0usize;

        // Recursively draw child frames
        for child in &self.children {
            let translate_vector = Vector2::new(
                current_x - 0.5 * (size - scale * child.bits as f32),
                -PKT_HEIGHT,
            );

            let translate_matrix = Matrix3::new_translation(&translate_vector);
            let mut child_drawable = child.draw_scaled(scale, bit_offset + current_b);
            child_drawable.transform(&translate_matrix);
            composite.add(child_drawable);

            current_x += scale * child.bits as f32;
            current_b += child.bits;
        }

        let height = scale;
        for i in bit_offset..(bit_offset + self.bits) {
            let mut tic = Rectangle::new(scale, height, String::new());
            tic.style.stroke_width = Some(RULER_STROKE);
            let translate_vector = Vector2::new(
                (i - bit_offset) as f32 * scale - (self.bits as f32 * scale) / 2.0 + scale / 2.0,
                (PKT_HEIGHT / 2.0) + height / 2.0,
            );
            let translate_matrix = Matrix3::new_translation(&translate_vector);
            tic.transform(&translate_matrix);
            composite.add(tic);

            if i % 8 == 0 {
                let mut label = Rectangle::new(scale * 2.0, height, format!("{}", i));
                label.style.stroke_width = None;
                label.text_style.font_size = Some(RULER_FONTSIZE);
                let translate_vector = Vector2::new(
                    (i - bit_offset) as f32 * scale - (self.bits as f32 * scale) / 2.0
                        + scale / 2.0,
                    (PKT_HEIGHT / 2.0) - 0.5 * height,
                );
                let translate_matrix = Matrix3::new_translation(&translate_vector);
                label.transform(&translate_matrix);
                composite.add(label);
            }
        }
        let mut label = Rectangle::new(
            scale * 2.0,
            height,
            format!("{}", bit_offset + self.bits - 1),
        );
        label.style.stroke_width = None;
        label.text_style.font_size = Some(RULER_FONTSIZE);
        let translate_vector = Vector2::new(
            (self.bits - 1) as f32 * scale - (self.bits as f32 * scale) / 2.0 + scale / 2.0,
            (PKT_HEIGHT / 2.0) - 0.5 * height,
        );
        let translate_matrix = Matrix3::new_translation(&translate_vector);
        label.transform(&translate_matrix);
        composite.add(label);

        composite
    }
}
