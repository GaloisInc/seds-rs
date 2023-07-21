use crate::eds::raw;
use crate::eds::resolved;
use crate::expr::ExpressionContext;

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
            data_type_set: resolved::DataTypeSet {
                data_types: Vec::new(),
            },
        }
    }
}

impl Resolve<resolved::NamedEntityType> for raw::NamedEntityType {
    fn resolve(&self, ectx: &ExpressionContext) -> resolved::NamedEntityType {
        resolved::NamedEntityType {
            name: resolved::Identifier(self.name.clone()),
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
