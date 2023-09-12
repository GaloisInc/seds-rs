//! Methods to make diagrams from the seds ast
use std::vec;

use crate::codegen::frame_diagram::{format::ToSvg, frame::PacketFrame, minify::minify_svg};

use crate::eds::ast::{DataType, EntryElement};

use super::{context::CodegenContext, RustCodegenError};

/// Get a PacketFrame model from a datatype (needed for diagramming)
fn get_frame_model(
    datatype: &DataType,
    ctx: &CodegenContext,
) -> Result<PacketFrame, RustCodegenError> {
    match &datatype {
        DataType::NoneDataType => Err(RustCodegenError::UnsupportedDataType(Box::new(datatype.clone()))),
        DataType::IntegerDataType(dt) => Ok(PacketFrame {
            name: dt.name_entity_type.name.0.clone(),
            bits: dt.encoding.size_in_bits,
            children: vec![],
        }),
        DataType::BooleanDataType(dt) => Ok(PacketFrame {
            name: dt.name_entity_type.name.0.clone(),
            bits: dt.encoding.size_in_bits,
            children: vec![],
        }),
        DataType::EnumeratedDataType(dt) => Ok(PacketFrame {
            name: dt.name_entity_type.name.0.clone(),
            bits: dt.encoding.size_in_bits,
            children: vec![],
        }),
        DataType::FloatDataType(dt) => Ok(PacketFrame {
            name: dt.name_entity_type.name.0.clone(),
            bits: dt.encoding.size_in_bits,
            children: vec![],
        }),
        DataType::StringDataType(dt) => Ok(PacketFrame {
            name: dt.name_entity_type.name.0.clone(),
            bits: dt.length,
            children: vec![],
        }),
        DataType::ContainerDataType(dt) => {
            let mut children = match &dt.base_type {
                Some(bdt) => vec![get_frame_model(ctx.lookup_ident(&bdt.0)?.data_type, ctx)?],
                None => vec![],
            };
            let entry_children: Vec<PacketFrame> = match &dt.entry_list {
                Some(el) => {
                    let mut children = vec![];
                    for entry in el.entries.iter() {
                        let dt = match &entry {
                            EntryElement::Entry(e) => ctx.lookup_ident(&e.type_.0)?.data_type,
                            EntryElement::FixedValueEntry(e) => {
                                ctx.lookup_ident(&e.type_.0)?.data_type
                            }
                            EntryElement::ErrorControlEntry(e) => {
                                ctx.lookup_ident(&e.type_.0)?.data_type
                            }
                            EntryElement::LengthEntry(e) => ctx.lookup_ident(&e.type_.0)?.data_type,
                            e => panic!("{:?} is not supported", e),
                        };
                        let pf = get_frame_model(dt, ctx)?;
                        children.push(pf);
                    }
                    children
                }
                None => vec![],
            };
            children.extend(entry_children);
            let lengths: Vec<usize> = children.iter().map(|c| c.bits).collect();
            Ok(PacketFrame {
                name: dt.name_entity_type.name.0.clone(),
                bits: if !lengths.is_empty() {
                    lengths.iter().sum()
                } else {
                    0
                },
                children,
            })
        }
        d => panic!("{:?} is not supported", d),
    }
}

/// get SVG diagram for datatype
pub fn get_datatype_packet_svg(
    datatype: &DataType,
    ctx: &CodegenContext,
) -> Result<String, RustCodegenError> {
    let pf = get_frame_model(datatype, ctx)?;
    let svg = pf.to_svg();
    Ok(minify_svg(&svg.to_string(), 5)
        .map_err(|_| RustCodegenError::SVGConversion)?
        .replace('\n', ""))
}
