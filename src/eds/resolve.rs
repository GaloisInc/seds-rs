use evalexpr::EvalexprError;

use crate::eds::ast;
use crate::eds::raw;
use crate::expr::ExpressionContext;
use crate::expr::NamespaceError;

use super::ast::Identifier;
use super::ast::Literal;
use super::raw::IntegerDataEncoding;

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
) -> Result<ast::IntegerEncoding, ResolveError> {
    let encoding_string = eval_to_string(s, ectx)?;
    match encoding_string.as_str() {
        "unsigned" => Ok(ast::IntegerEncoding::Unsigned),
        "signMagnitude" => Ok(ast::IntegerEncoding::SignMagnitude),
        "onesComplement" => Ok(ast::IntegerEncoding::OnesComplement),
        "twosComplement" => Ok(ast::IntegerEncoding::TwosComplement),
        "binaryCodedDecimal" => Ok(ast::IntegerEncoding::BinaryCodedDecimal),
        _ => Err(ResolveError::InvalidEncoding(encoding_string)),
    }
}

fn string_to_str_encoding(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<ast::StringEncoding, ResolveError> {
    let encoding_string = eval_to_string(s, ectx)?;
    match encoding_string.as_str() {
        "ASCII" => Ok(ast::StringEncoding::ASCII),
        "UTF-8" => Ok(ast::StringEncoding::UTF8),
        _ => Err(ResolveError::InvalidEncoding(encoding_string)),
    }
}

fn string_to_byte_order(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<ast::ByteOrder, ResolveError> {
    let bo_string = eval_to_string(s, ectx)?;
    match bo_string.as_str() {
        "littleEndian" => Ok(ast::ByteOrder::LittleEndian),
        "bigEndian" => Ok(ast::ByteOrder::BigEndian),
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
) -> Result<ast::FloatEncodingAndPrecision, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "IEEE754_2008_single" => Ok(ast::FloatEncodingAndPrecision::IEEE7542008Single),
        "IEEE754_2008_double" => Ok(ast::FloatEncodingAndPrecision::IEEE7542008Double),
        "IEEE754_2008_quadruple" => Ok(ast::FloatEncodingAndPrecision::IEEE7542008Quadruple),
        "MILSTD_1750A_simple" => Ok(ast::FloatEncodingAndPrecision::MILSTD1770ASimple),
        "MILSTD_1750A_extended" => Ok(ast::FloatEncodingAndPrecision::MILSTD1770AExtended),
        _ => Err(ResolveError::InvalidEncodingAndPrecision(s_string)),
    }
}

fn string_to_range_type(
    s: &String,
    ectx: &ExpressionContext,
) -> Result<ast::MinMaxRangeType, ResolveError> {
    let s_string = eval_to_string(s, ectx)?;
    match s_string.as_str() {
        "atLeast" => Ok(ast::MinMaxRangeType::AtLeast),
        "atMost" => Ok(ast::MinMaxRangeType::AtMost),
        "greaterThan" => Ok(ast::MinMaxRangeType::GreaterThan),
        "lessThan" => Ok(ast::MinMaxRangeType::LessThan),
        "exclusiveMinExclusiveMax" => Ok(ast::MinMaxRangeType::ExclusiveMinExclusiveMax),
        "exclusiveMinInclusiveMax" => Ok(ast::MinMaxRangeType::ExclusiveMinInclusiveMax),
        "inclusiveMinExclusiveMax" => Ok(ast::MinMaxRangeType::InclusiveMinExclusiveMax),
        "inclusiveMinInclusiveMax" => Ok(ast::MinMaxRangeType::InclusiveMinInclusiveMax),
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

impl Resolve<ast::PackageFile> for raw::PackageFile {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::PackageFile, ResolveError> {
        let package = self
            .package
            .iter()
            .map(|p| p.resolve(ectx))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ast::PackageFile { package })
    }
}

impl Resolve<ast::Package> for raw::Package {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::Package, ResolveError> {
        Ok(ast::Package {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            data_type_set: match self.data_type_set {
                Some(ref dts) => dts.resolve(ectx)?,
                None => ast::DataTypeSet {
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

impl Resolve<ast::MetaData> for raw::MetaData {
    fn resolve(&self, _: &ExpressionContext) -> Result<ast::MetaData, ResolveError> {
        Ok(ast::MetaData {
            creation_date: self.creation_date.clone(),
            creator: self.creator.clone(),
        })
    }
}

impl Resolve<ast::DataTypeSet> for raw::DataTypeSet {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::DataTypeSet, ResolveError> {
        let data_types = self
            .data_types
            .iter()
            .map(|p| p.resolve(ectx))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ast::DataTypeSet {
            data_types: data_types,
        })
    }
}

impl Resolve<ast::DataType> for raw::DataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::DataType, ResolveError> {
        match self {
            raw::DataType::IntegerDataType(dt) => {
                Ok(ast::DataType::IntegerDataType(dt.resolve(ectx)?))
            }
            raw::DataType::FloatDataType(dt) => Ok(ast::DataType::FloatDataType(dt.resolve(ectx)?)),
            raw::DataType::StringDataType(dt) => {
                Ok(ast::DataType::StringDataType(dt.resolve(ectx)?))
            }
            raw::DataType::BooleanDataType(dt) => {
                Ok(ast::DataType::BooleanDataType(dt.resolve(ectx)?))
            }
            _ => panic!("not implemented"),
        }
    }
}

impl Resolve<ast::FloatDataType> for raw::FloatDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::FloatDataType, ResolveError> {
        Ok(ast::FloatDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
                Some(ref fde) => fde.resolve(ectx)?,
                None => ast::FloatDataEncoding {
                    size_in_bits: 0,
                    encoding_and_precision: ast::FloatEncodingAndPrecision::IEEE7542008Single,
                    byte_order: ast::ByteOrder::LittleEndian,
                },
            },
            range: match self.range {
                Some(ref r) => Some(r.resolve(ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<ast::FloatDataEncoding> for raw::FloatDataEncoding {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::FloatDataEncoding, ResolveError> {
        Ok(ast::FloatDataEncoding {
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
            encoding_and_precision: string_to_encoding_and_precision(
                &self.encoding_and_precision,
                ectx,
            )?,
            byte_order: string_to_byte_order(&self.byte_order, ectx)?,
        })
    }
}

impl Resolve<ast::Range> for raw::Range {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::Range, ResolveError> {
        Ok(ast::Range {
            min_max_range: self.min_max_range.resolve(ectx)?,
        })
    }
}

impl Resolve<ast::MinMaxRange> for raw::MinMaxRange {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::MinMaxRange, ResolveError> {
        Ok(ast::MinMaxRange {
            min: Literal(self.min.clone()),
            max: Literal(self.max.clone()),
            range_type: string_to_range_type(&self.range_type, ectx)?,
        })
    }
}

impl Resolve<ast::StringDataType> for raw::StringDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::StringDataType, ResolveError> {
        Ok(ast::StringDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            length: string_to_usize(&self.length, ectx)?,
            encoding: match self.encoding {
                Some(ref sde) => sde.resolve(ectx)?,
                None => ast::StringDataEncoding {
                    encoding: ast::StringEncoding::ASCII,
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

impl Resolve<ast::StringDataEncoding> for raw::StringDataEncoding {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::StringDataEncoding, ResolveError> {
        Ok(ast::StringDataEncoding {
            encoding: match self.encoding {
                Some(ref se) => string_to_str_encoding(se, ectx)?,
                None => ast::StringEncoding::ASCII,
            },
            termination_character: match &self.termination_character {
                Some(tc) => Some(string_to_tc(tc, ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<ast::BooleanDataType> for raw::BooleanDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::BooleanDataType, ResolveError> {
        Ok(ast::BooleanDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
                Some(ref bde) => bde.resolve(ectx)?,
                None => ast::BooleanDataEncoding {
                    size_in_bits: 1,
                    false_value: true,
                },
            },
        })
    }
}

impl Resolve<ast::BooleanDataEncoding> for raw::BooleanDataEncoding {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::BooleanDataEncoding, ResolveError> {
        Ok(ast::BooleanDataEncoding {
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
            false_value: match self.false_value {
                Some(ref fv) => string_to_false_value(fv, ectx)?,
                None => true,
            },
        })
    }
}

impl Resolve<ast::IntegerDataType> for raw::IntegerDataType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::IntegerDataType, ResolveError> {
        Ok(ast::IntegerDataType {
            name_entity_type: self.name_entity_type.resolve(ectx)?,
            encoding: match self.encoding {
                Some(ref ide) => ide.resolve(ectx)?,
                None => ast::IntegerDataEncoding {
                    size_in_bits: 0,
                    encoding: ast::IntegerEncoding::Unsigned,
                    byte_order: ast::ByteOrder::LittleEndian,
                },
            },
            range: self.range.resolve(ectx)?,
        })
    }
}

impl Resolve<ast::IntegerDataEncoding> for IntegerDataEncoding {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::IntegerDataEncoding, ResolveError> {
        Ok(ast::IntegerDataEncoding {
            size_in_bits: string_to_usize(&self.size_in_bits, ectx)?,
            encoding: string_to_int_encoding(&self.encoding, ectx)?,
            byte_order: string_to_byte_order(&self.byte_order, ectx)?,
        })
    }
}

impl Resolve<ast::NamedEntityType> for raw::NamedEntityType {
    fn resolve(&self, ectx: &ExpressionContext) -> Result<ast::NamedEntityType, ResolveError> {
        Ok(ast::NamedEntityType {
            name: Identifier(self.name.clone()),
            short_description: self.short_description.clone(),
            long_description: match &self.long_description {
                Some(ld) => Some(ld.resolve(ectx)?),
                None => None,
            },
        })
    }
}

impl Resolve<ast::LongDescription> for raw::LongDescription {
    fn resolve(&self, _: &ExpressionContext) -> Result<ast::LongDescription, ResolveError> {
        Ok(ast::LongDescription {
            text: self.text.clone(),
        })
    }
}

pub fn resolve_package_file(
    package_file: &raw::PackageFile,
) -> Result<ast::PackageFile, ResolveError> {
    let ectx = ExpressionContext::new();
    package_file.resolve(&ectx)
}
