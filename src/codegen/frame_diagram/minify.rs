//! SVG Minification Options
//!
//! We use the (deprecated) Rust svgcleaner to strip and minify a SVG
//! We use the most aggressive settings
use svgcleaner::cleaner::clean_doc;
use svgcleaner::cleaner::parse_data;
use svgcleaner::CleaningOptions;
use svgcleaner::ParseOptions;
use svgcleaner::StyleJoinMode;
use svgcleaner::WriteOptions;

use svgdom::AttributesOrder;
use svgdom::Indent;
use svgdom::ListSeparator;
use svgdom::ToStringWithOptions;

/// cleaning options that we use for minification
fn cleaning_options() -> CleaningOptions {
    CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: true,
        merge_gradients: true,
        regroup_gradient_stops: true,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,

        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: true,
        remove_needless_attributes: true,
        remove_gradient_attributes: true,
        join_style_attributes: StyleJoinMode::None,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,

        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: true,

        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
    }
}

fn write_options() -> WriteOptions {
    WriteOptions {
        indent: Indent::Spaces(0),
        attributes_indent: Indent::None,
        use_single_quote: true,
        trim_hex_colors: true,
        write_hidden_attributes: false,
        remove_leading_zero: true,
        use_compact_path_notation: true,
        join_arc_to_flags: true,
        remove_duplicated_path_commands: true,
        use_implicit_lineto_commands: true,
        simplify_transform_matrices: true,
        list_separator: ListSeparator::Space,
        attributes_order: AttributesOrder::Alphabetical,
    }
}

/// possible errors from SVG minification
#[derive(Debug)]
pub enum MinificationError {
    /// minification re-parses using svgdom
    ParseError(svgdom::Error),
    /// minification runs svgcleaner
    CleanerError(svgcleaner::Error),
}

/// minify an svg string via svgcleaner
/// TODO: handle the unwraps
pub fn minify_svg(svg: &str, n_passes: u8) -> Result<String, MinificationError> {
    let po = ParseOptions::default();
    let co = cleaning_options();
    let wo = write_options();
    let mut document = parse_data(svg, &po).map_err(MinificationError::ParseError)?;
    for _ in 0..n_passes {
        clean_doc(&mut document, &co, &wo).map_err(MinificationError::CleanerError)?;
    }
    let doc_string = document.to_string_with_opt(&wo).replace('\n', "");
    Ok(doc_string)
}
