use crate::eds::raw;
use crate::eds::resolved;
use crate::expr::ExpressionContext;

use super::raw::IntegerDataEncoding;
use super::resolved::Identifier;
use super::resolved::Literal;

/// trait to convert a raw EDS component to a resolved EDS component
pub trait Resolve<T> {
    fn resolve(&self, ectx: &ExpressionContext) -> T;
}

impl Resolve<resolved::PackageFile> for raw::PackageFile {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::PackageFile {
        resolved::PackageFile {
            package: self.package.iter().map(|p| p.resolve(ectx)).collect(),
        }
    }
}

impl Resolve<resolved::Package> for raw::Package {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::Package {
        resolved::Package {
            name_entity_type: self.name_entity_type.resolve(ectx),
            data_type_set: match self.data_type_set {
                Some(ref dts) => dts.resolve(ectx),
                None => resolved::DataTypeSet {
                    data_types: Vec::new(),
                },
            },
        }
    }
}

impl Resolve<resolved::DataTypeSet> for raw::DataTypeSet {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::DataTypeSet {
        resolved::DataTypeSet {
            data_types: self.data_types.iter().map(|dt| dt.resolve(ectx)).collect(),
        }
    }
}

impl Resolve<resolved::DataType> for raw::DataType {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::DataType {
        match self {
            raw::DataType::IntegerDataType(dt) => {
                resolved::DataType::IntegerDataType(dt.resolve(ectx))
            }
            _ => panic!("not implemented"),
        }
    }
}

impl Resolve<resolved::IntegerDataType> for raw::IntegerDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::IntegerDataType {
        resolved::IntegerDataType {
            name: Identifier(self.name_entity_type.name.clone()),
            short_description: self
                .name_entity_type
                .short_description
                .clone()
                .unwrap_or("".to_string()),
            integer_data_encoding: match self.encoding {
                Some(ref ide) => ide.resolve(ectx),
                None => resolved::IntegerDataEncoding {
                    size_in_bits: 0,
                    encoding: resolved::IntegerEncoding::Unsigned,
                    byte_order: resolved::ByteOrder::LittleEndian,
                },
            },
            range: resolved::Range {
                min_max_range: resolved::MinMaxRange {
                    min: Literal("0".to_string()),
                    max: Literal("0".to_string()),
                    range_type: resolved::MinMaxRangeType::AtLeast,
                },
            },
        }
    }
}

impl Resolve<resolved::IntegerDataEncoding> for IntegerDataEncoding {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::IntegerDataEncoding {
        resolved::IntegerDataEncoding {
            size_in_bits: {
                let sib_eval = ectx.eval_expression(&self.size_in_bits).unwrap();
                // convert Value to string
                let sib_ustring = sib_eval.to_string();
                let sib_usize = sib_ustring.parse::<usize>().unwrap();
                sib_usize
            },
            encoding: {
                let encoding_eval = ectx.eval_expression(&self.encoding).unwrap();
                // convert Value to string
                let encoding_string = encoding_eval.as_string().unwrap();
                println!("encoding_string: {}", encoding_string.as_str());
                match encoding_string.as_str() {
                    "unsigned" => resolved::IntegerEncoding::Unsigned,
                    "signMagnitude" => resolved::IntegerEncoding::SignMagnitude,
                    "onesComplement" => resolved::IntegerEncoding::OnesComplement,
                    "twosComplement" => resolved::IntegerEncoding::TwosComplement,
                    "binaryCodedDecimal" => resolved::IntegerEncoding::BinaryCodedDecimal,
                    _ => panic!("invalid encoding {}", encoding_string.as_str()),
                }
            },
            byte_order: {
                let byte_order_eval = ectx.eval_expression(&self.byte_order).unwrap();
                // convert Value to string
                let byte_order_string = byte_order_eval.as_string().unwrap();
                match byte_order_string.as_str() {
                    "littleEndian" => resolved::ByteOrder::LittleEndian,
                    "bigEndian" => resolved::ByteOrder::BigEndian,
                    _ => panic!("invalid byte order"),
                }
            },
        }
    }
}

impl Resolve<resolved::NamedEntityType> for raw::NamedEntityType {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::NamedEntityType {
        resolved::NamedEntityType {
            name: Identifier(self.name.clone()),
            short_description: self.short_description.clone(),
            long_description: match &self.long_description {
                Some(ld) => Some(ld.resolve(ectx)),
                None => None,
            },
        }
    }
}

impl Resolve<resolved::LongDescription> for raw::LongDescription {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::LongDescription {
        resolved::LongDescription {
            text: self.text.clone(),
        }
    }
}

pub fn resolve_package_file(package_file: &raw::PackageFile) -> resolved::PackageFile {
    let ectx = ExpressionContext::new();
    package_file.resolve(&ectx)
}
