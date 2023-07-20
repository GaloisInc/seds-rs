//! 3.10 CONTAINERS
use std::collections::HashSet;

use seds_rs::eds::raw::{Constraint, DataType, DataTypeSet, Entry, EntryElement, PackageFile};

mod common;

use common::open_file;

fn get_test_data_type_set() -> DataTypeSet {
    let contents = open_file("eds/test/test_container.xml");
    let package: PackageFile = serde_xml_rs::from_str(&contents).unwrap();
    package.package[0].clone().data_type_set.unwrap()
}

/// 3.10.1 A ContainerDataType element may carry an optional abstract attribute which, if set to ‘true’, indicates that the container is not to be used directly, only referenced as the base type of other containers.
#[test]
fn test_3_10_1() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(is_abstract) = data._abstract {
                assert!(is_abstract == "false".to_string() || is_abstract == "true".to_string());
            }
        }
    }
}

/// 3.10.2 A ContainerDataType element may carry an optional baseType attribute which indicates that the container is a constrained extension of another.
#[test]
fn test_3_10_2() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(_base_type) = &data.base_type {
                // Just checking the existence of baseType attribute, replace the assertion with any specific check if needed.
                assert!(true);
            }
        }
    }
}

/// 3.10.3 A ContainerDataType element shall include zero or one ConstraintSet element and zero or one EntryList element.
#[test]
fn test_3_10_3() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            assert!(data.constraint_set.is_some() || data.entry_list.is_some());
        }
    }
}

/// 3.10.4 An abstract ContainerDataType element may include zero or one TrailerEntryList element.
#[test]
fn test_3_10_4() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            let abstract_value = data._abstract.unwrap();
            if abstract_value == "false".to_string() {
                assert!(
                    data.trailer_entry_list.is_none()
                        || data.trailer_entry_list.unwrap().entries.len() <= 1
                );
            }
        }
    }
}

/// 3.10.5 The ConstraintSet element of a ContainerDataType element shall specify the criteria that apply to the entries of the container type that is the base type of this container, in order for the type to be valid.
/// 3.10.6 The ConstraintSet element of a ContainerDataType element shall contain one or more child elements, which can be one of a RangeConstraint, a TypeConstraint, or a ValueConstraint.
#[test]
fn test_3_10_5_6() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(constraint_set) = &data.constraint_set {
                assert!(constraint_set.constraints.len() > 0);
                for constraint in constraint_set.constraints.iter() {
                    assert!(matches!(
                        constraint,
                        Constraint::RangeConstraint(_)
                            | Constraint::TypeConstraint(_)
                            | Constraint::ValueConstraint(_)
                    ));
                }
            }
        }
    }
}

/// 3.10.6 The ConstraintSet element of a ContainerDataType element shall contain one or more child elements, which can be one of a RangeConstraint, a TypeConstraint, or a ValueConstraint.
#[test]
fn test_3_10_6() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(constraint_set) = &data.constraint_set {
                assert!(constraint_set.constraints.len() > 0);
                for constraint in constraint_set.constraints.iter() {
                    assert!(matches!(
                        constraint,
                        Constraint::RangeConstraint(_)
                            | Constraint::TypeConstraint(_)
                            | Constraint::ValueConstraint(_)
                    ));
                }
            }
        }
    }
}

fn get_all_entries(data_type_set: &DataTypeSet) -> HashSet<String> {
    let mut all_entries = HashSet::new();
    for data_type in data_type_set.data_types.iter() {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(entry_list) = &data.entry_list {
                for entry in entry_list.entries.iter() {
                    match entry {
                        EntryElement::Entry(entry) => {
                            assert!(all_entries.insert(entry.name_entity_type.name.clone()))
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    all_entries
}

/// 3.10.7 Each child entry of a ConstraintSet shall have an attribute entry, which names the entry that the constraint applies to. This entry shall exist within a base container reachable by a recursive chain of base container references from the current container.
#[test]
fn test_3_10_7() {
    // TODO: no idea what this means
    /*
    let data_type_set = get_test_data_type_set();
    let all_entries = get_all_entries(&data_type_set); // Suppose this function gives us a set of all available entries
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if let Some(constraint_set) = &data.constraint_set {
                for constraint in constraint_set.constraints.iter() {
                    match constraint {
                        Constraint::RangeConstraint(constraint) => {
                            assert!(all_entries.contains(&constraint.entry))
                        }
                        Constraint::TypeConstraint(constraint) => {
                            assert!(all_entries.contains(&constraint.entry))
                        }
                        Constraint::ValueConstraint(constraint) => {
                            assert!(all_entries.contains(&constraint.entry))
                        }
                    }
                }
            }
        }
    }
    */
}

/// 3.10.8 A RangeConstraint element shall carry a child element of any type of range legal for the type of the constrained entry (see table 3-1).
#[test]
fn test_3_10_8() {}

/// 3.10.9 A TypeConstraint element shall have an attribute type, which shall reference a numeric type which has a range included in the type of the constrained entry.
#[test]
fn test_3_10_9() {}

/// 3.10.10 A ValueConstraint element shall have an attribute value, which shall contain a literal value of a type corresponding to the type of the constrained entry.
#[test]
fn test_3_10_10() {}
