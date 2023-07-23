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
    InvalidEncodingAndPrecision(String),
    InvalidRangeType(String),
    InvalidCharacter(String),
    InvalidExpressionString(String),
}

fn eval_to_string(s: &String, ectx: &ExpressionContext) -> Result<String, ResolveError> {
    let encoding_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    encoding_eval
        .as_string()
        .map_err(|e| ResolveError::ExpressionError(e))
}

fn eval_to_i64(s: &String, ectx: &ExpressionContext) -> Result<i64, ResolveError> {
    let encoding_eval = ectx
        .eval_expression(s)
        .map_err(|e| ResolveError::ExpressionContextError(e))?;
    // convert Value to string
    encoding_eval
        .as_int()
        .map_err(|e| ResolveError::ExpressionError(e))
}

fn string_to_int_encoding(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::IntegerEncoding, ResolveError> {
    let encoding_string = eval_to_string(s, ectx)?;
    match encoding_string.as_str() {
        "unsigned" => Ok(resolved::IntegerEncoding::Unsigned),
        "signMagnitude" => Ok(resolved::IntegerEncoding::SignMagnitude),
        "onesComplement" => Ok(resolved::IntegerEncoding::OnesComplement),
        "twosComplement" => Ok(resolved::IntegerEncoding::TwosComplement),
        "binaryCodedDecimal" => Ok(resolved::IntegerEncoding::BinaryCodedDecimal),
        _ => Err(ResolveError::InvalidEncoding(encoding_string)),
    }
}

fn string_to_str_encoding(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::StringEncoding, ResolveError> {
    let encoding_string = eval_to_string(s, ectx)?;
    match encoding_string.as_str() {
        "ASCII" => Ok(resolved::StringEncoding::ASCII),
        "UTF-8" => Ok(resolved::StringEncoding::UTF8),
        _ => Err(ResolveError::InvalidEncoding(encoding_string)),
    }
}

fn string_to_byte_order(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::ByteOrder, ResolveError> {
    let bo_string = eval_to_string(s, ectx)?;
    match bo_string.as_str() {
        "littleEndian" => Ok(resolved::ByteOrder::LittleEndian),
        "bigEndian" => Ok(resolved::ByteOrder::BigEndian),
        _ => Err(ResolveError::InvalidByteOrder(bo_string)),
    }
}

fn string_to_usize(s: &String, ectx: &ExpressionContext) -> Result<usize, ResolveError> {
    Ok(eval_to_i64(s, ectx)? as usize)
}

fn string_to_false_value(s: &String, ectx: &ExpressionContext) -> Result<bool, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "zeroIsFalse" => Ok(true),
        "nonZeroIsFalse" => Ok(false),
        _ => Err(ResolveError::InvalidFalseValue(s_string)),
    }
}

fn string_to_boolean(s: &String, ectx: &ExpressionContext) -> Result<bool, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ResolveError::InvalidFalseValue(s_string)),
    }
}

fn string_to_encoding_and_precision(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::FloatEncodingAndPrecision, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "IEEE754_2008_single" => Ok(resolved::FloatEncodingAndPrecision::IEEE7542008Single),
        "IEEE754_2008_double" => Ok(resolved::FloatEncodingAndPrecision::IEEE7542008Double),
        "IEEE754_2008_quadruple" => Ok(resolved::FloatEncodingAndPrecision::IEEE7542008Quadruple),
        "MILSTD_1750A_simple" => Ok(resolved::FloatEncodingAndPrecision::MILSTD1770ASimple),
        "MILSTD_1750A_extended" => Ok(resolved::FloatEncodingAndPrecision::MILSTD1770AExtended),
        _ => Err(ResolveError::InvalidEncodingAndPrecision(s_string)),
    }
}

fn string_to_range_type(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<resolved::MinMaxRangeType, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "atLeast" => Ok(resolved::MinMaxRangeType::AtLeast),
        "atMost" => Ok(resolved::MinMaxRangeType::AtMost),
        "greaterThan" => Ok(resolved::MinMaxRangeType::GreaterThan),
        "lessThan" => Ok(resolved::MinMaxRangeType::LessThan),
        "exclusiveMinExclusiveMax" => Ok(resolved::MinMaxRangeType::ExclusiveMinExclusiveMax),
        "exclusiveMinInclusiveMax" => Ok(resolved::MinMaxRangeType::ExclusiveMinInclusiveMax),
        "inclusiveMinExclusiveMax" => Ok(resolved::MinMaxRangeType::InclusiveMinExclusiveMax),
        "inclusiveMinInclusiveMax" => Ok(resolved::MinMaxRangeType::InclusiveMinInclusiveMax),
        _ => Err(ResolveError::InvalidRangeType(s_string)),
    }
}

fn string_to_tc(s: &String, ectx: &ExpressionContext) -> Result<char, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;

    Ok(match &s_string[..] {
        "\\n" => '\n',
        "\\t" => '\t',
        "\\r" => '\r',
        "\\0" => '\0',
        _ => s_string
            .chars()
            .next()
            .ok_or(ResolveError::InvalidCharacter(s_string))?,
    })
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
            metadata: match self.metadata {
                Some(ref m) => Some(m.resolve(ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<resolved::MetaData> for raw::MetaData {
    fn resolve(&self, _: &ExpressionContext) -> Result<resolved::MetaData, ResolveError> {
        Ok(resolved::MetaData {
            creation_date: self.creation_date.clone(),
            creator: self.creator.clone(),
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
            raw::DataType::FloatDataType(dt) => {
                Ok(resolved::DataType::FloatDataType(dt.resolve(ectx)?))
            }
            raw::DataType::StringDataType(dt) => {
                Ok(resolved::DataType::StringDataType(dt.resolve(ectx)?))
            }
            raw::DataType::BooleanDataType(dt) => {
                Ok(resolved::DataType::BooleanDataType(dt.resolve(ectx)?))
            }
            _ => panic!("not implemented"),
        }
    }
}

impl Resolve<resolved::FloatDataType> for raw::FloatDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::FloatDataType, ResolveError> {
        Ok(resolved::FloatDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
                Some(ref fde) => fde.resolve(ectx)?,
                None => resolved::FloatDataEncoding {
                    size_in_bits: 0,
                    encoding_and_precision: resolved::FloatEncodingAndPrecision::IEEE7542008Single,
                    byte_order: resolved::ByteOrder::LittleEndian,
                },
            },
            range: match self.range {
                Some(ref r) => Some(r.resolve(ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<resolved::FloatDataEncoding> for raw::FloatDataEncoding {
    fn resolve(
        &self,
        ectx: &ExpressionContext,
    ) -> Result<resolved::FloatDataEncoding, ResolveError> {
        Ok(resolved::FloatDataEncoding {
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
            encoding_and_precision: string_to_encoding_and_precision(
                &self.encoding_and_precision,
                ectx,
            )?,
            byte_order: string_to_byte_order(&self.byte_order, ectx)?,
        })
    }
}

impl Resolve<resolved::Range> for raw::Range {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::Range, ResolveError> {
        Ok(resolved::Range {
            min_max_range: self.min_max_range.resolve(ectx)?,
        })
    }
}

impl Resolve<resolved::MinMaxRange> for raw::MinMaxRange {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::MinMaxRange, ResolveError> {
        Ok(resolved::MinMaxRange {
            min: Literal(self.min.clone()),
            max: Literal(self.max.clone()),
            range_type: string_to_range_type(&self.range_type, ectx)?,
        })
    }
}

impl Resolve<resolved::StringDataType> for raw::StringDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::StringDataType, ResolveError> {
        Ok(resolved::StringDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            length: string_to_usize(&self.length, ectx)?,
            encoding: match self.encoding {
                Some(ref sde) => sde.resolve(ectx)?,
                None => resolved::StringDataEncoding {
                    encoding: resolved::StringEncoding::ASCII,
                    termination_character: None,
                },
            },
            fixed_length: match self.fixed_length {
                Some(ref fl) => string_to_boolean(fl, ectx)?,
                None => false,
            },
        })
    }
}

impl Resolve<resolved::StringDataEncoding> for raw::StringDataEncoding {
    fn resolve(
        &self,
        ectx: &ExpressionContext,
    ) -> Result<resolved::StringDataEncoding, ResolveError> {
        Ok(resolved::StringDataEncoding {
            encoding: match self.encoding {
                Some(ref se) => string_to_str_encoding(se, ectx)?,
                None => resolved::StringEncoding::ASCII,
            },
            termination_character: match &self.termination_character {
                Some(tc) => Some(string_to_tc(tc, ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<resolved::BooleanDataType> for raw::BooleanDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<resolved::BooleanDataType, ResolveError> {
        Ok(resolved::BooleanDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
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
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
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
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
                Some(ref ide) => ide.resolve(ectx)?,
                None => resolved::IntegerDataEncoding {
                    size_in_bits: 0,
                    encoding: resolved::IntegerEncoding::Unsigned,
                    byte_order: resolved::ByteOrder::LittleEndian,
                },
            },
            range: self.range.resolve(ectx)?,
        })
    }
}

impl Resolve<resolved::IntegerDataEncoding> for IntegerDataEncoding {
    fn resolve(
        &self,
        ectx: &ExpressionContext,
    ) -> Result<resolved::IntegerDataEncoding, ResolveError> {
        Ok(resolved::IntegerDataEncoding {
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
            encoding: string_to_int_encoding(&self.encoding, ectx)?,
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
