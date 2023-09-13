//! Rustdoc Generation for the Rust Code Generation

use std::io::Error;

use crate::eds::ast::{
    BooleanDataType, Constraint, ConstraintSet, ContainerDataType, DataType, DataTypeSet, Entry,
    EnumeratedDataType, FixedValueEntry, FloatDataType, IntegerDataType, LengthEntry, MinMaxRange,
    MinMaxRangeType, NamedEntityType, Package, StringDataType,
};

use super::{context::CodegenContext, diagram::get_datatype_packet_svg};


use prettytable::{format, Cell, Row, Table};

/// formatter so that the item can be represented in rustdoc
pub trait FormatRustDoc {
    /// convert self to a form represented in rustdoc string
    fn to_docstring(&self) -> String;
}

/// Codegen for RustDoc
pub trait ToRustDoc {
    /// convert to a RustDoc description
    fn to_description(&self, ctx: &CodegenContext) -> String;
}

/// build the doc string from a NamedEntityType
fn get_doc_string(
    name: Option<&NamedEntityType>,
    name_entity_type: &NamedEntityType,
    dt: &DataType,
    ctx: &CodegenContext,
) -> String {
    let mut description = String::new();
    description.push_str(&name_entity_type.name.0.to_string());

    // Select the relevant variant: name if present, otherwise name_entity_type
    let relevant_name = name.unwrap_or(name_entity_type);

    if let Some(long_description) = &relevant_name.long_description {
        description.push_str(&format!("\n{}", long_description.text));
    } else if let Some(short_description) = &relevant_name.short_description {
        description.push_str(&format!(" - {}", short_description));
    }

    let svg_res = get_datatype_packet_svg(dt, ctx);

    match svg_res {
        Ok(svg) => description.replace(r"#[packet_diagram]", &svg),
        Err(_e) => description,
    }
}

/// generate markdown table that summarized the constraint docs
fn get_constraint_docs(constraint_set: &ConstraintSet) -> Result<String, Error> {
    let mut table = Table::new();

    // format required by rustdoc (GitHub markdown tables)
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', '|', '|', '|'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    // Add a row with headers
    table.set_titles(Row::new(vec![
        Cell::new("Entry Name"),
        Cell::new("Constraint Type"),
        Cell::new("Details"),
    ]));

    for constraint in &constraint_set.constraints {
        let _ = match constraint {
            Constraint::RangeConstraint(rc) => table.add_row(Row::new(vec![
                Cell::new(&rc.entry.0),
                Cell::new("Range"),
                Cell::new(&rc.range.min_max_range.to_docstring()),
            ])),
            Constraint::TypeConstraint(tc) => table.add_row(Row::new(vec![
                Cell::new(&tc.entry.0),
                Cell::new("Type"),
                Cell::new(&tc.type_.0),
            ])),
            Constraint::ValueConstraint(vc) => table.add_row(Row::new(vec![
                Cell::new(&vc.entry.0),
                Cell::new("Value"),
                Cell::new(&vc.value.0),
            ])),
        };
    }

    // Convert the table to markdown and return
    let mut output = Vec::new();

    let _ = table.print(&mut output)?;
    String::from_utf8(output).map_err(|e| Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))
}

/// generate summary tables for the package docs
fn get_data_type_docs(data_type_set: &DataTypeSet) -> Result<String, Error> {
    // Helper function to create a table from a collection of data types
    fn create_table(data: &Vec<(String, String)>) -> Result<String, Error> {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separator(
                format::LinePosition::Title,
                format::LineSeparator::new('-', '|', '|', '|'),
            )
            .padding(1, 1)
            .build();
        table.set_format(format);

        table.set_titles(Row::new(vec![Cell::new("Entry Name"), Cell::new("Base")]));

        for (name, datatype) in data {
            let _row = table.add_row(Row::new(vec![Cell::new(name), Cell::new(datatype)]));
        }

        let mut output = Vec::new();
        let _ = table.print(&mut output)?;
        String::from_utf8(output)
            .map_err(|e| Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))
    }

    // Separate data by type
    let mut integers = Vec::new();
    let mut containers = Vec::new();
    // ... Similarly, create collections for other types ...

    for data_type in &data_type_set.data_types {
        match data_type {
            DataType::IntegerDataType(data) => {
                integers.push((
                    data.name_entity_type.name.0.clone(),
                    "IntegerDataType".to_string(),
                ));
            }
            DataType::ContainerDataType(data) => match &data.base_type {
                Some(bn) => containers.push((data.name_entity_type.name.0.clone(), bn.0.clone())),
                None => {
                    containers.push((data.name_entity_type.name.0.clone(), "<None>".to_string()))
                }
            },
            // ... Handle other DataType variants similarly ...
            _ => {}
        }
    }

    // Generate tables for each type and combine them
    let _integer_table = if !integers.is_empty() {
        Some(create_table(&integers)?)
    } else {
        None
    };

    let container_table = if !containers.is_empty() {
        Some(create_table(&containers)?)
    } else {
        None
    };

    // Return all the tables combined by newline
    Ok(vec![container_table /*, ... other tables ... */]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n\n"))
}

/// format MinMaxRange in RustDoc using set builder notation
impl FormatRustDoc for MinMaxRange {
    fn to_docstring(&self) -> String {
        match &self.range_type {
            MinMaxRangeType::ExclusiveMinExclusiveMax => {
                format!("\\{{ x \\| {} < x < {} \\}}", self.min.0, self.max.0)
            }
            MinMaxRangeType::InclusiveMinInclusiveMax => {
                format!("\\{{x | {} \\leq x \\leq {} \\}}", self.min.0, self.max.0)
            }
            MinMaxRangeType::InclusiveMinExclusiveMax => {
                format!("\\{{x | {} \\leq x < {} \\}}", self.min.0, self.max.0)
            }
            MinMaxRangeType::ExclusiveMinInclusiveMax => {
                format!("\\{{x | {} < x \\leq {} \\}}", self.min.0, self.max.0)
            }
            MinMaxRangeType::GreaterThan => format!("\\{{x | {} < x \\}}", self.min.0),
            MinMaxRangeType::AtLeast => format!("\\{{x | {} \\leq x \\}}", self.min.0),
            MinMaxRangeType::LessThan => format!("\\{{x | x < {} \\}}", self.max.0),
            MinMaxRangeType::AtMost => format!("\\{{x | x \\leq {} \\}}", self.max.0),
        }
    }
}

impl ToRustDoc for Package {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        let descr = get_doc_string(name, &self.name_entity_type, &DataType::NoneDataType, ctx);
        let table = get_data_type_docs(&self.data_type_set)
            .unwrap_or("ERROR generating DataType summary".to_string());
        format!("{}\n\n## Containers\n\n{}", descr, table)
    }
}

impl ToRustDoc for EnumeratedDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::EnumeratedDataType(self.clone()),
            ctx,
        )
    }
}

impl ToRustDoc for StringDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::StringDataType(self.clone()),
            ctx,
        )
    }
}

impl ToRustDoc for FloatDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::FloatDataType(self.clone()),
            ctx,
        )
    }
}

impl ToRustDoc for IntegerDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::IntegerDataType(self.clone()),
            ctx,
        )
    }
}

impl ToRustDoc for BooleanDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::BooleanDataType(self.clone()),
            ctx,
        )
    }
}

impl ToRustDoc for ContainerDataType {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        let name = ctx.name;
        let doc = get_doc_string(
            name,
            &self.name_entity_type,
            &DataType::ContainerDataType(self.clone()),
            ctx,
        );

        match &self.constraint_set {
            Some(cs) => {
                format!(
                    "{}\n## Constraints\n{}",
                    doc,
                    get_constraint_docs(cs)
                        .unwrap_or("ERROR Generating Constraint Set".to_string())
                )
            }
            None => doc,
        }
    }
}

impl ToRustDoc for LengthEntry {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        get_doc_string(
            Some(&self.name_entity_type),
            &self.name_entity_type,
            &DataType::NoneDataType,
            ctx,
        )
    }
}

impl ToRustDoc for Entry {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        get_doc_string(
            Some(&self.name_entity_type),
            &self.name_entity_type,
            &DataType::NoneDataType,
            ctx,
        )
    }
}

impl ToRustDoc for FixedValueEntry {
    fn to_description(&self, ctx: &CodegenContext) -> String {
        get_doc_string(
            Some(&self.name_entity_type),
            &self.name_entity_type,
            &DataType::NoneDataType,
            ctx,
        )
    }
}
