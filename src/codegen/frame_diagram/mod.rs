//! # Frame Diagramming Library
//!
//! The `frame_generator` library facilitates the representation and rendering of packet frames in a hierarchical, structured manner. It provides tools to describe packet frames at various granularities, from single-bit fields to composed packets with nested structures.
//!
//! ## Overview
//!
//! - **Frame Representation**: The central type of this library is `PacketFrame`, which can have nested frames, allowing users to create detailed and complex frame diagrams with ease.
//!
//! - **Rendering**: The frames can be transformed into visual representations using the provided drawing utilities. The current backend supports SVG format, which is suitable for web displays, documentation, or print.
//!
//! - **Extensibility**: The library is designed with extensibility in mind, allowing for the addition of new drawing backends, styles, or even new shapes.
//!
//! ## Modules
//!
//! - **drawable**: Provides traits and structs to convert frames into visual representations.
//!
//! - **format**: Houses utilities for converting drawable items into various formats, including SVG.
//!
//! - **frame**: Contains the core `PacketFrame` struct and related functionalities.
//!
//! - **style**: Allows customization of visual appearance for drawables.
//!
//!
//! ## Dependencies
//! - `svg`: External crate used for generating SVG files.

pub extern crate svg;

pub mod drawable;
pub mod format;
pub mod frame;
pub mod minify;
pub mod style;
