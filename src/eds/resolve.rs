use evalexpr::EvalexprError;

use crate::eds::raw;
use crate::eds::resolved;
use crate::expr::ExpressionContext;
use crate::expr::NamespaceError;

use super::raw::IntegerDataEncoding;
use super::resolved::Identifier;
use super::resolved::Literal;

/// Errors that can occur during resolution
#[derive(Debug)]
pub enum ResolveError {
    ExpressionError(EvalexprError),
    ExpressionContextError(NamespaceError),
    InvalidEncoding(String),
    InvalidByteOrder(String),
    InvalidSizeInBits(String),
    InvalidFalseValue(String),
    InvalidExpressionString(String),
}

fn string_to_encoding(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::IntegerEncoding, ResolveError> {
    let encoding_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    let encoding_string = encoding_eval
        .as_string()
        .map_err(|e| ResolveError::ExpressionError(e))?;
    match encoding_string.as_str() {
        "unsigned" => Ok(resolved::IntegerEncoding::Unsigned),
        "signMagnitude" => Ok(resolved::IntegerEncoding::SignMagnitude),
        "onesComplement" => Ok(resolved::IntegerEncoding::OnesComplement),
        "twosComplement" => Ok(resolved::IntegerEncoding::TwosComplement),
        "binaryCodedDecimal" => Ok(resolved::IntegerEncoding::BinaryCodedDecimal),
        _ => Err(ResolveError::InvalidEncoding(encoding_string)),
    }
}

fn string_to_byte_order(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::ByteOrder, ResolveError> {
    let bo_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    let bo_string = bo_eval
        .as_string()
        .map_err(|e| ResolveError::ExpressionError(e))?;
    match bo_string.as_str() {
        "littleEndian" => Ok(resolved::ByteOrder::LittleEndian),
        "bigEndian" => Ok(resolved::ByteOrder::BigEndian),
        _ => Err(ResolveError::InvalidByteOrder(bo_string)),
    }
}

fn string_to_size_in_bits(s: &String, ectx: &ExpressionContext) -> Result<usize, ResolveError> {
    let sib_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    let sib_ustring = sib_eval.to_string();
    let sib_result = sib_ustring.parse::<usize>();
    match sib_result {
        Err(_) => Err(ResolveError::InvalidSizeInBits(sib_ustring)),
        Ok(sib_usize) => Ok(sib_usize),
    }
}

fn string_to_false_value(s: &String, ectx: &ExpressionContext) -> Result<bool, ResolveError> {
    let s_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    let s_string = s_eval
        .as_string()
        .map_err(|e| ResolveError::ExpressionError(e))?;
    match s_string.as_str() {
        "zeroIsFalse" => Ok(true),
        "nonZeroIsFalse" => Ok(false),
        _ => Err(ResolveError::InvalidFalseValue(s_string)),
    }
}

/// trait to convert a raw EDS component to a resolved EDS component
pub trait Resolve<T> {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<T, ResolveError>;
}

impl Resolve<resolved::PackageFile> for raw::PackageFile {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::PackageFile, ResolveError> {
        let package = self
            .package
            .iter()
            .map(|p| p.resolve(ectx))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(resolved::PackageFile { package })
    }
}

impl Resolve<resolved::Package> for raw::Package {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::Package, ResolveError> {
        Ok(resolved::Package {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            data_type_set: match self.data_type_set {
                Some(ref dts) => dts.resolve(ectx)?,
                None => resolved::DataTypeSet {
                    data_types: Vec::new(),
                },
            },
        })
    }
}

impl Resolve<resolved::DataTypeSet> for raw::DataTypeSet {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::DataTypeSet, ResolveError> {
        let data_types = self
            .data_types
            .iter()
            .map(|p| p.resolve(ectx))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(resolved::DataTypeSet {
            data_types: data_types,
        })
    }
}

impl Resolve<resolved::DataType> for raw::DataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::DataType, ResolveError> {
        match self {
            raw::DataType::IntegerDataType(dt) => {
                Ok(resolved::DataType::IntegerDataType(dt.resolve(ectx)?))
            }
            raw::DataType::BooleanDataType(dt) => {
                Ok(resolved::DataType::BooleanDataType(dt.resolve(ectx)?))
            }
            _ => panic!("not implemented"),
        }
    }
}

impl Resolve<resolved::BooleanDataType> for raw::BooleanDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::BooleanDataType, ResolveError> {
        Ok(resolved::BooleanDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            boolean_data_encoding: match self.encoding {
                Some(ref bde) => bde.resolve(ectx)?,
                None => resolved::BooleanDataEncoding {
                    size_in_bits: 1,
                    false_value: true,
                },
            },
        })
    }
}

impl Resolve<resolved::BooleanDataEncoding> for raw::BooleanDataEncoding {
    fn resolve(
        &self,
        ectx: &ExpressionContext,
    ) -> Result<resolved::BooleanDataEncoding, ResolveError> {
        Ok(resolved::BooleanDataEncoding {
            size_in_bits: string_to_size_in_bits(&self.size_in_bits, ectx)?,
            false_value: match self.false_value {
                Some(ref fv) => string_to_false_value(fv, ectx)?,
                None => true,
            },
        })
    }
}

impl Resolve<resolved::IntegerDataType> for raw::IntegerDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::IntegerDataType, ResolveError> {
        Ok(resolved::IntegerDataType {
            name: Identifier(self.name_entity_type.name.clone()),
            short_description: self
                .name_entity_type
                .short_description
                .clone()
                .unwrap_or("".to_string()),
            integer_data_encoding: match self.encoding {
                Some(ref ide) => ide.resolve(ectx)?,
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
        })
    }
}

impl Resolve<resolved::IntegerDataEncoding> for IntegerDataEncoding {
    fn resolve(
        &self,
        ectx: &ExpressionContext,
    ) -> Result<resolved::IntegerDataEncoding, ResolveError> {
        Ok(resolved::IntegerDataEncoding {
            size_in_bits: string_to_size_in_bits(&self.size_in_bits, ectx)?,
            encoding: string_to_encoding(&self.encoding, ectx)?,
            byte_order: string_to_byte_order(&self.byte_order, ectx)?,
        })
    }
}

impl Resolve<resolved::NamedEntityType> for raw::NamedEntityType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::NamedEntityType, ResolveError> {
        Ok(resolved::NamedEntityType {
            name: Identifier(self.name.clone()),
            short_description: self.short_description.clone(),
            long_description: match &self.long_description {
                Some(ld) => Some(ld.resolve(ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<resolved::LongDescription> for raw::LongDescription {
    fn resolve(&self, _: &ExpressionContext) -> Result<resolved::LongDescription, ResolveError> {
        Ok(resolved::LongDescription {
            text: self.text.clone(),
        })
    }
}

pub fn resolve_package_file(
    package_file: &raw::PackageFile,
) -> Result<resolved::PackageFile, ResolveError> {
    let ectx = ExpressionContext::new();
    package_file.resolve(&ectx)
}
