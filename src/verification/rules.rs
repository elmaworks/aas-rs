//! Business rule verification functions for AAS.

#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{
    AAS_IDENTIFIABLES, AAS_REFERABLES, DATA_TYPE_IEC_61360_FOR_DOCUMENT,
    DATA_TYPE_IEC_61360_FOR_PROPERTY_OR_VALUE, DATA_TYPE_IEC_61360_FOR_REFERENCE, FRAGMENT_KEYS,
    GENERIC_FRAGMENT_KEYS, GENERIC_GLOBALLY_IDENTIFIABLES, GLOBALLY_IDENTIFIABLES,
    IEC_61360_DATA_TYPES_WITH_UNIT,
};
use crate::types::class::Class;
use crate::{
    AasSubmodelElements, DataTypeDefXsd, EmbeddedDataSpecification, Extension, KeyTypes,
    OperationVariable, Qualifier, Reference, ReferenceTypes,
};

use super::pattern::is_bcp47_for_english;
use super::value::value_consistent_with_xsd_type;

/// Check that all lang strings specify unique languages.
pub fn lang_strings_have_unique_languages<T: HasLanguage>(lang_strings: &[T]) -> bool {
    let mut seen = HashSet::new();
    for ls in lang_strings {
        if !seen.insert(ls.language()) {
            return false;
        }
    }
    true
}

/// Trait for items that have a language field.
pub trait HasLanguage {
    fn language(&self) -> &str;
}

macro_rules! impl_has_language {
    ($t:ty) => {
        impl HasLanguage for $t {
            fn language(&self) -> &str {
                &self.language
            }
        }
    };
}

use crate::{
    LangStringDefinitionTypeIec61360, LangStringNameType, LangStringPreferredNameTypeIec61360,
    LangStringShortNameTypeIec61360, LangStringTextType,
};

impl_has_language!(LangStringNameType);
impl_has_language!(LangStringTextType);
impl_has_language!(LangStringPreferredNameTypeIec61360);
impl_has_language!(LangStringShortNameTypeIec61360);
impl_has_language!(LangStringDefinitionTypeIec61360);

/// Check that qualifier types are unique.
pub fn qualifier_types_are_unique(qualifiers: &[Qualifier]) -> bool {
    let mut seen = HashSet::new();
    for q in qualifiers {
        if !seen.insert(q.type_.as_str()) {
            return false;
        }
    }
    true
}

/// Check that ID-shorts are unique across referables.
pub fn id_shorts_are_unique(classes: &[Class]) -> bool {
    let mut seen = HashSet::new();
    for cls in classes {
        if let Some(id_short) = get_id_short(cls) {
            if !seen.insert(id_short) {
                return false;
            }
        }
    }
    true
}

fn get_id_short(cls: &Class) -> Option<&str> {
    match cls {
        Class::RelationshipElement(x) => x.id_short.as_deref(),
        Class::SubmodelElementList(x) => x.id_short.as_deref(),
        Class::SubmodelElementCollection(x) => x.id_short.as_deref(),
        Class::Property(x) => x.id_short.as_deref(),
        Class::MultiLanguageProperty(x) => x.id_short.as_deref(),
        Class::Range(x) => x.id_short.as_deref(),
        Class::ReferenceElement(x) => x.id_short.as_deref(),
        Class::Blob(x) => x.id_short.as_deref(),
        Class::File(x) => x.id_short.as_deref(),
        Class::AnnotatedRelationshipElement(x) => x.id_short.as_deref(),
        Class::Entity(x) => x.id_short.as_deref(),
        Class::BasicEventElement(x) => x.id_short.as_deref(),
        Class::Operation(x) => x.id_short.as_deref(),
        Class::Capability(x) => x.id_short.as_deref(),
        _ => None,
    }
}

/// Check that ID-shorts of operation variables are unique across input, output, inoutput.
pub fn id_shorts_of_variables_are_unique(
    input_variables: Option<&[OperationVariable]>,
    output_variables: Option<&[OperationVariable]>,
    inoutput_variables: Option<&[OperationVariable]>,
) -> bool {
    let mut seen = HashSet::new();
    for vars in [input_variables, output_variables, inoutput_variables]
        .into_iter()
        .flatten()
    {
        for var in vars {
            if let Some(id_short) = get_id_short(&var.value) {
                if !seen.insert(id_short) {
                    return false;
                }
            }
        }
    }
    true
}

/// Check that extension names are unique.
pub fn extension_names_are_unique(extensions: &[Extension]) -> bool {
    let mut seen = HashSet::new();
    for ext in extensions {
        if !seen.insert(ext.name.as_str()) {
            return false;
        }
    }
    true
}

/// Check that all submodel elements have identical semanticId key values.
pub fn submodel_elements_have_identical_semantic_ids(elements: &[Class]) -> bool {
    let mut first_semantic_id: Option<&Reference> = None;
    for elem in elements {
        if let Some(sid) = get_semantic_id(elem) {
            if let Some(first) = first_semantic_id {
                if !reference_key_values_equal(first, sid) {
                    return false;
                }
            } else {
                first_semantic_id = Some(sid);
            }
        }
    }
    true
}

fn get_semantic_id(cls: &Class) -> Option<&Reference> {
    match cls {
        Class::RelationshipElement(x) => x.semantic_id.as_ref(),
        Class::SubmodelElementList(x) => x.semantic_id.as_ref(),
        Class::SubmodelElementCollection(x) => x.semantic_id.as_ref(),
        Class::Property(x) => x.semantic_id.as_ref(),
        Class::MultiLanguageProperty(x) => x.semantic_id.as_ref(),
        Class::Range(x) => x.semantic_id.as_ref(),
        Class::ReferenceElement(x) => x.semantic_id.as_ref(),
        Class::Blob(x) => x.semantic_id.as_ref(),
        Class::File(x) => x.semantic_id.as_ref(),
        Class::AnnotatedRelationshipElement(x) => x.semantic_id.as_ref(),
        Class::Entity(x) => x.semantic_id.as_ref(),
        Class::BasicEventElement(x) => x.semantic_id.as_ref(),
        Class::Operation(x) => x.semantic_id.as_ref(),
        Class::Capability(x) => x.semantic_id.as_ref(),
        _ => None,
    }
}

/// Check that `reference` is a model reference to the expected type.
pub fn is_model_reference_to(reference: &Reference, expected_type: KeyTypes) -> bool {
    reference.type_ == ReferenceTypes::ModelReference
        && !reference.keys.is_empty()
        && reference.keys.last().map(|k| k.type_) == Some(expected_type)
}

/// Check that `reference` is a model reference to a referable.
pub fn is_model_reference_to_referable(reference: &Reference) -> bool {
    reference.type_ == ReferenceTypes::ModelReference
        && !reference.keys.is_empty()
        && reference
            .keys
            .last()
            .is_some_and(|k| AAS_REFERABLES.contains(&k.type_))
}

/// Check that `that` and `other` references have equal key values.
pub fn reference_key_values_equal(that: &Reference, other: &Reference) -> bool {
    if that.keys.len() != other.keys.len() {
        return false;
    }
    that.keys
        .iter()
        .zip(other.keys.iter())
        .all(|(a, b)| a.value == b.value)
}

/// Check that all properties or ranges have the given value type.
pub fn properties_or_ranges_have_value_type(
    elements: &[Class],
    value_type: DataTypeDefXsd,
) -> bool {
    for elem in elements {
        match elem {
            Class::Property(p) => {
                if p.value_type != value_type {
                    return false;
                }
            }
            Class::Range(r) => {
                if r.value_type != value_type {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

/// Check that a submodel element is of the expected type.
pub fn submodel_element_is_of_type(element: &Class, expected_type: AasSubmodelElements) -> bool {
    match expected_type {
        AasSubmodelElements::AnnotatedRelationshipElement => {
            element.is_annotated_relationship_element()
        }
        AasSubmodelElements::BasicEventElement => element.is_basic_event_element(),
        AasSubmodelElements::Blob => element.is_blob(),
        AasSubmodelElements::Capability => element.is_capability(),
        AasSubmodelElements::DataElement => {
            element.is_blob()
                || element.is_file()
                || element.is_multi_language_property()
                || element.is_property()
                || element.is_range()
                || element.is_reference_element()
        }
        AasSubmodelElements::Entity => element.is_entity(),
        AasSubmodelElements::EventElement => element.is_basic_event_element(),
        AasSubmodelElements::File => element.is_file(),
        AasSubmodelElements::MultiLanguageProperty => element.is_multi_language_property(),
        AasSubmodelElements::Operation => element.is_operation(),
        AasSubmodelElements::Property => element.is_property(),
        AasSubmodelElements::Range => element.is_range(),
        AasSubmodelElements::ReferenceElement => element.is_reference_element(),
        AasSubmodelElements::RelationshipElement => element.is_relationship_element(),
        AasSubmodelElements::SubmodelElement => element.is_submodel_element(),
        AasSubmodelElements::SubmodelElementList => element.is_submodel_element_list(),
        AasSubmodelElements::SubmodelElementCollection => element.is_submodel_element_collection(),
    }
}

/// Check data specification IEC 61360s for PROPERTY or VALUE categories have appropriate data type.
pub fn data_specification_iec61360s_for_property_or_value_have_appropriate_data_type(
    embedded: &[EmbeddedDataSpecification],
) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                match content.data_type {
                    None => return false,
                    Some(dt) => {
                        if !DATA_TYPE_IEC_61360_FOR_PROPERTY_OR_VALUE.contains(&dt) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Check data specification IEC 61360s for REFERENCE category have appropriate data type.
pub fn data_specification_iec61360s_for_reference_have_appropriate_data_type(
    embedded: &[EmbeddedDataSpecification],
) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                match content.data_type {
                    None => return false,
                    Some(dt) => {
                        if !DATA_TYPE_IEC_61360_FOR_REFERENCE.contains(&dt) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Check data specification IEC 61360s for DOCUMENT category have appropriate data type.
pub fn data_specification_iec61360s_for_document_have_appropriate_data_type(
    embedded: &[EmbeddedDataSpecification],
) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                match content.data_type {
                    None => return false,
                    Some(dt) => {
                        if !DATA_TYPE_IEC_61360_FOR_DOCUMENT.contains(&dt) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Check that all IEC 61360 data specifications have a data type defined.
pub fn data_specification_iec61360s_have_data_type(embedded: &[EmbeddedDataSpecification]) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                if content.data_type.is_none() {
                    return false;
                }
            }
        }
    }
    true
}

/// Check that all IEC 61360 data specifications have a value.
pub fn data_specification_iec61360s_have_value(embedded: &[EmbeddedDataSpecification]) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                if content.value.is_none() {
                    return false;
                }
            }
        }
    }
    true
}

/// Check that all IEC 61360 data specifications have a definition in English.
pub fn data_specification_iec61360s_have_definition_at_least_in_english(
    embedded: &[EmbeddedDataSpecification],
) -> bool {
    for eds in embedded {
        if eds
            .data_specification_content
            .is_data_specification_iec61360()
        {
            if let Class::DataSpecificationIec61360(content) =
                eds.data_specification_content.as_ref()
            {
                match &content.definition {
                    None => return false,
                    Some(defs) => {
                        if !defs.iter().any(|ls| is_bcp47_for_english(&ls.language)) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Check Reference AASd-121 through AASd-127 constraints.
/// Returns true if the reference keys constraint passes for Reference type.
pub fn reference_keys_valid(reference: &Reference) -> (bool, bool, bool, bool, bool, bool, bool) {
    let has_keys = !reference.keys.is_empty();

    // AASd-121: first key must be globally identifiable
    let c121 = if has_keys {
        GLOBALLY_IDENTIFIABLES.contains(&reference.keys[0].type_)
    } else {
        true // no keys, skip
    };

    // AASd-122: external reference first key must be generic globally identifiable
    let c122 = if reference.type_ == ReferenceTypes::ExternalReference && has_keys {
        GENERIC_GLOBALLY_IDENTIFIABLES.contains(&reference.keys[0].type_)
    } else {
        true
    };

    // AASd-123: model reference first key must be AAS identifiable
    let c123 = if reference.type_ == ReferenceTypes::ModelReference && has_keys {
        AAS_IDENTIFIABLES.contains(&reference.keys[0].type_)
    } else {
        true
    };

    // AASd-124: external reference last key must be generic globally identifiable or fragment key
    let c124 = if reference.type_ == ReferenceTypes::ExternalReference && has_keys {
        let last = reference.keys.last().unwrap();
        GENERIC_GLOBALLY_IDENTIFIABLES.contains(&last.type_)
            || GENERIC_FRAGMENT_KEYS.contains(&last.type_)
    } else {
        true
    };

    // AASd-125: model reference with >1 keys: all keys after first must be fragment keys
    let c125 = if reference.type_ == ReferenceTypes::ModelReference && reference.keys.len() > 1 {
        reference.keys[1..]
            .iter()
            .all(|k| FRAGMENT_KEYS.contains(&k.type_))
    } else {
        true
    };

    // AASd-126: model reference with >1 keys: all non-last keys must not be fragment-only
    let c126 = if reference.type_ == ReferenceTypes::ModelReference && reference.keys.len() > 1 {
        reference.keys[1..reference.keys.len() - 1]
            .iter()
            .all(|k| !GENERIC_FRAGMENT_KEYS.contains(&k.type_))
    } else {
        true
    };

    // AASd-127: model reference last key must be a referable (AAS referables or fragment)
    let c127 = if reference.type_ == ReferenceTypes::ModelReference && has_keys {
        let last = reference.keys.last().unwrap();
        AAS_REFERABLES.contains(&last.type_) || GENERIC_FRAGMENT_KEYS.contains(&last.type_)
    } else {
        true
    };

    (c121, c122, c123, c124, c125, c126, c127)
}

/// Public re-export of `value_consistent_with_xsd_type` for use in verifier.
pub use super::value::value_consistent_with_xsd_type as check_value_consistency;

/// Check that IEC 61360 data types with unit have unit or `unit_id`.
pub fn iec61360_data_type_with_unit_has_unit(
    data_type: Option<crate::DataTypeIec61360>,
    unit: Option<&str>,
    unit_id: Option<&Reference>,
) -> bool {
    match data_type {
        None => true,
        Some(dt) => {
            if IEC_61360_DATA_TYPES_WITH_UNIT.contains(&dt) {
                unit.is_some() || unit_id.is_some()
            } else {
                true
            }
        }
    }
}
