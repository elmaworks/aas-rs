//! Tests for type-checking (is_*, as_*) functions.
//! Mirrors aas-core3.0-typescript/test/types.typeMatches.spec.ts

mod common;

use aas_rs::types::class::Class;

fn load_minimal(type_name: &str, is_fn: impl Fn(&Class) -> bool) -> Class {
    common::load_minimal_of(type_name, is_fn)
}

// The TypeScript test verifies that typesMatch(a, b) is true only when
// both instances are the same concrete type. We model this as: for each
// concrete type T, is_T(instance) must be true exactly for instances of T.

#[test]
fn test_extension_is_extension() {
    let inst = load_minimal("Extension", |c| c.is_extension());
    assert!(inst.is_extension());
    assert!(!inst.is_administrative_information());
    assert!(!inst.is_qualifier());
    assert!(!inst.is_asset_administration_shell());
    assert!(!inst.is_submodel());
    assert!(!inst.is_property());
}

#[test]
fn test_administrative_information_is_administrative_information() {
    let inst = load_minimal("AdministrativeInformation", |c| {
        c.is_administrative_information()
    });
    assert!(inst.is_administrative_information());
    assert!(!inst.is_extension());
    assert!(!inst.is_qualifier());
}

#[test]
fn test_qualifier_is_qualifier() {
    let inst = load_minimal("Qualifier", |c| c.is_qualifier());
    assert!(inst.is_qualifier());
    assert!(!inst.is_extension());
}

#[test]
fn test_asset_administration_shell_is_asset_administration_shell() {
    let inst = load_minimal("AssetAdministrationShell", |c| {
        c.is_asset_administration_shell()
    });
    assert!(inst.is_asset_administration_shell());
    assert!(!inst.is_submodel());
}

#[test]
fn test_asset_information_is_asset_information() {
    let inst = load_minimal("AssetInformation", |c| c.is_asset_information());
    assert!(inst.is_asset_information());
    assert!(!inst.is_extension());
}

#[test]
fn test_resource_is_resource() {
    let inst = load_minimal("Resource", |c| c.is_resource());
    assert!(inst.is_resource());
    assert!(!inst.is_extension());
}

#[test]
fn test_specific_asset_id_is_specific_asset_id() {
    let inst = load_minimal("SpecificAssetId", |c| c.is_specific_asset_id());
    assert!(inst.is_specific_asset_id());
    assert!(!inst.is_extension());
}

#[test]
fn test_submodel_is_submodel() {
    let inst = load_minimal("Submodel", |c| c.is_submodel());
    assert!(inst.is_submodel());
    assert!(!inst.is_extension());
}

#[test]
fn test_property_is_property() {
    let inst = load_minimal("Property", |c| c.is_property());
    assert!(inst.is_property());
    assert!(!inst.is_submodel());
    assert!(!inst.is_extension());
}

#[test]
fn test_multi_language_property_is_multi_language_property() {
    let inst = load_minimal("MultiLanguageProperty", |c| c.is_multi_language_property());
    assert!(inst.is_multi_language_property());
    assert!(!inst.is_property());
}

#[test]
fn test_range_is_range() {
    let inst = load_minimal("Range", |c| c.is_range());
    assert!(inst.is_range());
    assert!(!inst.is_property());
}

#[test]
fn test_reference_element_is_reference_element() {
    let inst = load_minimal("ReferenceElement", |c| c.is_reference_element());
    assert!(inst.is_reference_element());
    assert!(!inst.is_property());
}

#[test]
fn test_blob_is_blob() {
    let inst = load_minimal("Blob", |c| c.is_blob());
    assert!(inst.is_blob());
    assert!(!inst.is_property());
}

#[test]
fn test_file_is_file() {
    let inst = load_minimal("File", |c| c.is_file());
    assert!(inst.is_file());
    assert!(!inst.is_blob());
}

#[test]
fn test_annotated_relationship_element_is_annotated_relationship_element() {
    let inst = load_minimal("AnnotatedRelationshipElement", |c| {
        c.is_annotated_relationship_element()
    });
    assert!(inst.is_annotated_relationship_element());
    assert!(!inst.is_relationship_element());
}

#[test]
fn test_entity_is_entity() {
    let inst = load_minimal("Entity", |c| c.is_entity());
    assert!(inst.is_entity());
    assert!(!inst.is_property());
}

#[test]
fn test_basic_event_element_is_basic_event_element() {
    let inst = load_minimal("BasicEventElement", |c| c.is_basic_event_element());
    assert!(inst.is_basic_event_element());
    assert!(!inst.is_entity());
}

#[test]
fn test_operation_is_operation() {
    let inst = load_minimal("Operation", |c| c.is_operation());
    assert!(inst.is_operation());
    assert!(!inst.is_entity());
}

#[test]
fn test_operation_variable_is_operation_variable() {
    let inst = load_minimal("OperationVariable", |c| c.is_operation_variable());
    assert!(inst.is_operation_variable());
    assert!(!inst.is_operation());
}

#[test]
fn test_capability_is_capability() {
    let inst = load_minimal("Capability", |c| c.is_capability());
    assert!(inst.is_capability());
    assert!(!inst.is_operation());
}

#[test]
fn test_concept_description_is_concept_description() {
    let inst = load_minimal("ConceptDescription", |c| c.is_concept_description());
    assert!(inst.is_concept_description());
    assert!(!inst.is_submodel());
}

#[test]
fn test_reference_is_reference() {
    let inst = load_minimal("Reference", |c| c.is_reference());
    assert!(inst.is_reference());
    assert!(!inst.is_key());
}

#[test]
fn test_key_is_key() {
    let inst = load_minimal("Key", |c| c.is_key());
    assert!(inst.is_key());
    assert!(!inst.is_reference());
}

#[test]
fn test_lang_string_name_type_is_lang_string_name_type() {
    let inst = load_minimal("LangStringNameType", |c| c.is_lang_string_name_type());
    assert!(inst.is_lang_string_name_type());
    assert!(!inst.is_lang_string_text_type());
}

#[test]
fn test_lang_string_text_type_is_lang_string_text_type() {
    let inst = load_minimal("LangStringTextType", |c| c.is_lang_string_text_type());
    assert!(inst.is_lang_string_text_type());
    assert!(!inst.is_lang_string_name_type());
}

#[test]
fn test_embedded_data_specification_is_embedded_data_specification() {
    let inst = load_minimal("EmbeddedDataSpecification", |c| {
        c.is_embedded_data_specification()
    });
    assert!(inst.is_embedded_data_specification());
    assert!(!inst.is_extension());
}

#[test]
fn test_level_type_is_level_type() {
    let inst = load_minimal("LevelType", |c| c.is_level_type());
    assert!(inst.is_level_type());
    assert!(!inst.is_extension());
}

#[test]
fn test_value_reference_pair_is_value_reference_pair() {
    let inst = load_minimal("ValueReferencePair", |c| c.is_value_reference_pair());
    assert!(inst.is_value_reference_pair());
    assert!(!inst.is_level_type());
}

#[test]
fn test_value_list_is_value_list() {
    let inst = load_minimal("ValueList", |c| c.is_value_list());
    assert!(inst.is_value_list());
    assert!(!inst.is_value_reference_pair());
}

#[test]
fn test_lang_string_preferred_name_type_iec61360() {
    let inst = load_minimal("LangStringPreferredNameTypeIec61360", |c| {
        c.is_lang_string_preferred_name_type_iec61360()
    });
    assert!(inst.is_lang_string_preferred_name_type_iec61360());
    assert!(!inst.is_lang_string_name_type());
}

#[test]
fn test_lang_string_short_name_type_iec61360() {
    let inst = load_minimal("LangStringShortNameTypeIec61360", |c| {
        c.is_lang_string_short_name_type_iec61360()
    });
    assert!(inst.is_lang_string_short_name_type_iec61360());
    assert!(!inst.is_lang_string_preferred_name_type_iec61360());
}

#[test]
fn test_lang_string_definition_type_iec61360() {
    let inst = load_minimal("LangStringDefinitionTypeIec61360", |c| {
        c.is_lang_string_definition_type_iec61360()
    });
    assert!(inst.is_lang_string_definition_type_iec61360());
    assert!(!inst.is_lang_string_short_name_type_iec61360());
}

#[test]
fn test_data_specification_iec61360_is_data_specification_iec61360() {
    let inst = load_minimal("DataSpecificationIec61360", |c| {
        c.is_data_specification_iec61360()
    });
    assert!(inst.is_data_specification_iec61360());
    assert!(!inst.is_embedded_data_specification());
}

#[test]
fn test_relationship_element_is_relationship_element() {
    let inst = load_minimal("RelationshipElement", |c| c.is_relationship_element());
    assert!(inst.is_relationship_element());
    // AnnotatedRelationshipElement is NOT a RelationshipElement in Rust enum terms
    let are_inst = load_minimal("AnnotatedRelationshipElement", |c| {
        c.is_annotated_relationship_element()
    });
    assert!(!are_inst.is_relationship_element());
}

#[test]
fn test_submodel_element_list_is_submodel_element_list() {
    let inst = load_minimal("SubmodelElementList", |c| c.is_submodel_element_list());
    assert!(inst.is_submodel_element_list());
    assert!(!inst.is_submodel_element_collection());
}

#[test]
fn test_submodel_element_collection_is_submodel_element_collection() {
    let inst = load_minimal("SubmodelElementCollection", |c| {
        c.is_submodel_element_collection()
    });
    assert!(inst.is_submodel_element_collection());
    assert!(!inst.is_submodel_element_list());
}

#[test]
fn test_environment_is_environment() {
    let inst = common::load_minimal_environment();
    assert!(inst.is_environment());
    assert!(!inst.is_submodel());
    assert!(!inst.is_extension());
}

#[test]
fn test_event_payload_is_event_payload() {
    let inst = common::load_minimal_event_payload();
    assert!(inst.is_event_payload());
    assert!(!inst.is_basic_event_element());
    assert!(!inst.is_extension());
}
