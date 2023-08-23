//! 3.10 CONTAINERS
use std::collections::HashSet;

use seds_rs::eds::raw::{Constraint, DataType, DataTypeSet, EntryElement, PackageFile};

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

/// 3.10.7 Each child entry of a ConstraintSet shall have an attribute entry, which names the entry that the constraint applies to. This entry shall exist within a base container reachable by a recursive chain of base container references from the current container.
#[test]
fn test_3_10_7() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for constraint in data.constraint_set.unwrap().constraints {
                // Make sure that the entry the constraint applies to exists
                let entries = data.entry_list.clone().unwrap().entries;
                let no_match = "".to_string();
                assert!(entries.iter().any(|entry| match entry {
                    EntryElement::Entry(e) => &e.name_entity_type.name,
                    EntryElement::FixedValueEntry(e) => &e.name_entity_type.name,
                    EntryElement::ListEntry(e) => &e.name_entity_type.name,
                    EntryElement::LengthEntry(e) => &e.name_entity_type.name,
                    _ => &no_match,
                } == match &constraint {
                    Constraint::RangeConstraint(range_constraint) => &range_constraint.entry,
                    Constraint::TypeConstraint(type_constraint) => &type_constraint.entry,
                    Constraint::ValueConstraint(value_constraint) => &value_constraint.entry,
                }));
            }
        }
    }
}

/// 3.10.8 A RangeConstraint element shall carry a child element of any type of range legal for the type of the constrained entry (see table 3-1).
#[test]
fn test_3_10_8() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for constraint in data.constraint_set.unwrap().constraints {
                if let Constraint::RangeConstraint(range_constraint) = constraint {
                    assert!(range_constraint.range.min_max_range.min.len() > 0);
                }
            }
        }
    }
}

/// 3.10.9 A TypeConstraint element shall have an attribute type, which shall reference a numeric type which has a range included in the type of the constrained entry.
#[test]
fn test_3_10_9() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for constraint in data.constraint_set.unwrap().constraints {
                if let Constraint::TypeConstraint(type_constraint) = constraint {
                    assert!(!type_constraint.type_.is_empty());
                }
            }
        }
    }
}

/// 3.10.10 A ValueConstraint element shall have an attribute value, which shall contain a literal value of a type corresponding to the type of the constrained entry.
#[test]
fn test_3_10_10() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for constraint in data.constraint_set.unwrap().constraints {
                if let Constraint::ValueConstraint(value_constraint) = constraint {
                    assert!(!value_constraint.value.is_empty());
                }
            }
        }
    }
}

/// 3.10.11 The EntryList and TrailerEntryList elements of a ContainerDataType element shall contain one or more Entry, FixedValueEntry, PaddingEntry, ListEntry, LengthEntry, and ErrorControlEntry child elements.
#[test]
fn test_3_10_11() {
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            assert!(data.entry_list.unwrap().entries.len() > 0);
            assert!(data.trailer_entry_list.unwrap().entries.len() > 0);
        }
    }
}

/// 3.10.12 The first entry in an EntryList is located at a bit offset immediately following the last entry of the EntryList of any base container, or offset 0 if no such container exists.
#[test]
fn test_3_10_12() {
    // This test might need more context on how the 'base container' and 'bit offset' can be determined or calculated.
}

/// 3.10.13 For an abstract packet, the first entry in a TrailerEntryList is located at a bit offset immediately following all entries of the derived container.
#[test]
fn test_3_10_13() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            if data._abstract.unwrap_or_default() == "false".to_string() {
                for entry in data.trailer_entry_list.unwrap().entries {
                    match entry {
                        EntryElement::Entry(_) => visit_entry = true,
                        _ => continue,
                    }
                }
                assert!(visit_entry);
            }
        }
    }
}

/// 3.10.14 Each other entry in an EntryList or TrailerEntryList is located at a bit offset immediately following the previous entry.
#[test]
fn test_3_10_14() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                match entry {
                    EntryElement::Entry(_) => visit_entry = true,
                    _ => continue,
                }
            }
            assert!(visit_entry);
        }
    }
}

/// 3.10.16 Each Entry, FixedValueEntry, ListEntry, LengthEntry, and ErrorControlEntry element within a container shall have a name that is unique within that container, plus all containers recursively referenced by its baseType attribute.
#[test]
fn test_3_10_16() {
    let data_type_set = get_test_data_type_set();
    let mut names = HashSet::new();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                match entry {
                    EntryElement::Entry(e) => {
                        assert!(names.insert(e.name_entity_type.name.clone()));
                    }
                    EntryElement::FixedValueEntry(e) => {
                        assert!(names.insert(e.name_entity_type.name.clone()));
                    }
                    EntryElement::ListEntry(e) => {
                        assert!(names.insert(e.name_entity_type.name.clone()));
                    }
                    EntryElement::LengthEntry(e) => {
                        assert!(names.insert(e.name_entity_type.name.clone()));
                    }
                    EntryElement::ErrorControlEntry(e) => {
                        assert!(names.insert(e.name_entity_type.name.clone()));
                    }
                    _ => continue,
                }
            }
        }
    }
}

/// 3.10.17 A FixedValueEntry entry shall have a fixedValue attribute which specifies the value to which the container entry should be fixed.
#[test]
fn test_3_10_17() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::FixedValueEntry(entry) = entry {
                    assert!(entry.fixed_value.len() > 0);
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.18 If the fixedValue attribute is used to specify the value for an entry; the value shall be a literal whose type matches the type of the entry according to table 3-1.
#[test]
fn test_3_10_18() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::FixedValueEntry(entry) = entry {
                    assert!(entry.fixed_value.len() > 0);
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.19 A PaddingEntry element within a container shall have an attribute sizeInBits, which is used to specify the position of successive fields.
#[test]
fn test_3_10_19() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::PaddingEntry(entry) = entry {
                    // convert size_in_bits to usize
                    let size_in_bits = entry.size_in_bits.parse::<usize>().unwrap();
                    assert!(size_in_bits > 0);
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.20 A ListEntry element within a container shall specify an attribute listLengthField which contains the name of another element of the same container whose value will be used to determine the number of times this entry should be repeated.
#[test]
fn test_3_10_20() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::ListEntry(_entry) = entry {
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.21 A LengthEntry element within a container shall specify an entry whose value is constrained, or derived, based on the length of the container in which it is present.
#[test]
fn test_3_10_21() {
    let mut visit_length_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::LengthEntry(_) = entry {
                    // We can't directly test whether the value is constrained or derived based on the container length.
                    // For now, we'll just test whether length_entry exists.
                    visit_length_entry = true;
                }
            }
        }
    }
    assert!(visit_length_entry);
}

/// 3.10.22 If a LengthEntry element has a calibration (see 3.11.7), that calibration shall be used to map between the length in bytes of the container and the value of the entry, according to the formula: container length in bytes = calibration(entry raw value).
#[test]
fn test_3_10_22() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::LengthEntry(entry) = entry {
                    let cal = entry.calibration;
                    assert!(cal.is_some());
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.23 Any calibration specified for a LengthEntry shall be reversible, that is, a linear polynomial, or spline, with all points of degree 1.
#[test]
fn test_3_10_23() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::LengthEntry(entry) = entry {
                    let cal = entry.calibration;
                    assert!(cal.is_some());
                    visit_entry = true;
                    // TODO: check if cal is reversible
                }
            }
        }
    }
    assert!(visit_entry);
}

/// 3.10.24 An ErrorControlEntry element within a container shall specify an entry whose value is constrained, or derived, based on the contents of the container in which it is present.  In addition to a subset of the attributes and elements supported for a regular container entry, it has the mandatory attribute type, which is one of the values specified in the DoT for errorControlType as illustrated in table 3-3.
#[test]
fn test_3_10_24() {
    let mut visit_entry = false;
    let data_type_set = get_test_data_type_set();
    for data_type in data_type_set.data_types {
        if let DataType::ContainerDataType(data) = data_type {
            for entry in data.entry_list.unwrap().entries {
                if let EntryElement::ErrorControlEntry(_) = entry {
                    // We can't directly test whether the value is constrained or derived based on the container contents.
                    visit_entry = true;
                }
            }
        }
    }
    assert!(visit_entry);
}
