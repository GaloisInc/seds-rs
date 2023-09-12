//! A simple drawing system
//!
//! This is a very simple drawing system that supports primitives, composites, transformations
//!
//! It also supports basic bounding boxes for placement algorithms
use nalgebra::{Matrix3, Point2, Vector2};

use crate::codegen::frame_diagram::format::ToSvg;
use crate::codegen::frame_diagram::style::{ShapeStyle, TextStyle};

/// Axis-Aligned Bounding Box (AABB)
pub struct BoundingBox {
    /// top left vertex 2D
    pub top_left: Point2<f32>,
    /// bottom right vertex 2D
    pub bottom_right: Point2<f32>,
}

/// Core Drawable Trait
pub trait Drawable: ToSvg {
    /// all drawable objects can be moved
    fn transform(&mut self, matrix: &Matrix3<f32>);

    /// all drawable objects can be bounded by a bounding box (axis-aligned)
    fn get_bounding_box(&self) -> BoundingBox;

    /// get string needed for a SVG viewBox
    fn get_viewbox_str(&self) -> String {
        let bb = self.get_bounding_box();
        format!(
            "{} {} {} {}",
            bb.top_left.x,
            bb.top_left.y,
            (bb.bottom_right.x - bb.top_left.x),
            (bb.bottom_right.y - bb.top_left.y)
        )
    }
}

/// CompositeDrawable - all items share the same local space
pub struct CompositeDrawable {
    /// all drawable items in a composite
    pub items: Vec<Box<dyn Drawable>>,
}

impl CompositeDrawable {
    /// create a new, empty composite drawable
    pub fn new() -> Self {
        CompositeDrawable { items: Vec::new() }
    }

    /// add a drawable to the composite
    pub fn add<T: Drawable + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    /// non-mutable iterate over all items
    pub fn iter_items(&self) -> std::slice::Iter<'_, Box<dyn Drawable>> {
        self.items.iter()
    }
}

impl Drawable for CompositeDrawable {
    fn transform(&mut self, matrix: &Matrix3<f32>) {
        for item in &mut self.items {
            item.transform(matrix);
        }
    }

    fn get_bounding_box(&self) -> BoundingBox {
        if self.items.is_empty() {
            return BoundingBox {
                top_left: Point2::origin(),
                bottom_right: Point2::origin(),
            };
        }

        let mut composite_bbox = self.items[0].get_bounding_box();

        for item in &self.items[1..] {
            let bbox = item.get_bounding_box();

            if bbox.top_left.x < composite_bbox.top_left.x {
                composite_bbox.top_left.x = bbox.top_left.x;
            }

            if bbox.top_left.y < composite_bbox.top_left.y {
                composite_bbox.top_left.y = bbox.top_left.y;
            }

            if bbox.bottom_right.x > composite_bbox.bottom_right.x {
                composite_bbox.bottom_right.x = bbox.bottom_right.x;
            }

            if bbox.bottom_right.y > composite_bbox.bottom_right.y {
                composite_bbox.bottom_right.y = bbox.bottom_right.y;
            }
        }

        composite_bbox
    }
}

impl Default for CompositeDrawable {
    fn default() -> Self {
        Self::new()
    }
}

/// Define the Rectangle
pub struct Rectangle {
    /// Rectangle Width
    pub width: f32,
    /// Rectangle Height
    pub height: f32,
    /// Text in Rectangle (serves as a textbox)
    pub name: String,
    /// Transformation to Apply
    pub transform: Matrix3<f32>,
    /// Style Attributes for Rectangle
    pub style: ShapeStyle,
    /// Style Attributes for Text
    pub text_style: TextStyle,
}

impl Rectangle {
    /// Constructor
    pub fn new(width: f32, height: f32, name: String) -> Self {
        Self {
            width,
            height,
            name,
            transform: Matrix3::<f32>::identity(),
            style: ShapeStyle::default(),
            text_style: TextStyle::default(),
        }
    }

    fn get_transformed_point(&self, point: Point2<f32>) -> Point2<f32> {
        self.transform.transform_point(&point)
    }
}

impl Drawable for Rectangle {
    fn transform(&mut self, matrix: &Matrix3<f32>) {
        // Apply the new transformation on top of any existing transformation
        self.transform *= matrix;
    }

    fn get_bounding_box(&self) -> BoundingBox {
        // Calculate the transformed corners
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        let top_left = self.get_transformed_point(Point2::new(-half_width, -half_height));
        let bottom_right = self.get_transformed_point(Point2::new(half_width, half_height));

        BoundingBox {
            top_left,
            bottom_right,
        }
    }
}

/// Canvas is like a CompositeShape with Linear Repositioning
/// i.e., it uses the bounding boxes to place the shapes so that
/// they do not overlap
pub struct Canvas {
    /// drawable objects in the canvas
    pub drawables: Vec<Box<dyn Drawable>>,
}

impl Canvas {
    /// constructor
    pub fn new() -> Self {
        Canvas {
            drawables: Vec::new(),
        }
    }

    /// add a new drawable, repositioning from left to right
    pub fn add<T: Drawable + 'static>(&mut self, drawable: T) {
        let mut transformed_drawable = Box::new(drawable);
        if let Some(last_drawable) = self.drawables.last() {
            let last_bounding_box = last_drawable.get_bounding_box();
            let current_bounding_box = transformed_drawable.get_bounding_box();

            let translate_vector = Vector2::new(
                last_bounding_box.bottom_right.x + current_bounding_box.bottom_right.x + 10.0, // 10.0 is a gap for simplicity
                0.0,
            );

            let translate_matrix = Matrix3::new_translation(&translate_vector);
            transformed_drawable.transform(&translate_matrix);
        }

        self.drawables.push(transformed_drawable);
    }

    /// obtain bounding box for all drawables
    pub fn get_bounding_box(&self) -> BoundingBox {
        if self.drawables.is_empty() {
            return BoundingBox {
                top_left: Point2::origin(),
                bottom_right: Point2::origin(),
            };
        }

        let mut composite_bbox = self.drawables[0].get_bounding_box();

        for item in &self.drawables[1..] {
            let bbox = item.get_bounding_box();

            if bbox.top_left.x < composite_bbox.top_left.x {
                composite_bbox.top_left.x = bbox.top_left.x;
            }

            if bbox.top_left.y < composite_bbox.top_left.y {
                composite_bbox.top_left.y = bbox.top_left.y;
            }

            if bbox.bottom_right.x > composite_bbox.bottom_right.x {
                composite_bbox.bottom_right.x = bbox.bottom_right.x;
            }

            if bbox.bottom_right.y > composite_bbox.bottom_right.y {
                composite_bbox.bottom_right.y = bbox.bottom_right.y;
            }
        }

        composite_bbox
    }

    /// reposition all shapes so that they fall in (0, infty)^2
    pub fn reposition(&mut self) {
        let bb = self.get_bounding_box();

        let translate_vector = Vector2::new(-bb.top_left.x, -bb.top_left.y);

        let translate_matrix = Matrix3::new_translation(&translate_vector);

        for d in self.drawables.iter_mut() {
            d.transform(&translate_matrix);
        }
    }

    /// get string needed for a SVG viewBox
    pub fn get_viewbox_str(&self) -> String {
        let bb = self.get_bounding_box();
        format!(
            "{} {} {} {}",
            bb.top_left.x,
            bb.top_left.y,
            (bb.bottom_right.x - bb.top_left.x),
            (bb.bottom_right.y - bb.top_left.y)
        )
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
