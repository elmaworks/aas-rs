//! Tests for `as_*` and `is_*` cast functions.
//! Mirrors aas-core3.0-typescript/test/types.casts.spec.ts

mod common;

#[test]
fn test_casts_over_extension() {
    let instance = common::load_minimal_of("Extension", |c| c.is_extension());

    assert!(instance.is_extension());
    assert!(instance.as_extension().is_some());

    // Extension is NOT any of these in the TypeScript model
    assert!(!instance.is_administrative_information());
    assert!(instance.as_administrative_information().is_none());
    assert!(!instance.is_qualifier());
    assert!(instance.as_qualifier().is_none());
    assert!(!instance.is_asset_administration_shell());
    assert!(instance.as_asset_administration_shell().is_none());
    assert!(!instance.is_asset_information());
    assert!(instance.as_asset_information().is_none());
    assert!(!instance.is_resource());
    assert!(instance.as_resource().is_none());
    assert!(!instance.is_specific_asset_id());
    assert!(instance.as_specific_asset_id().is_none());
    assert!(!instance.is_submodel());
    assert!(instance.as_submodel().is_none());
    assert!(!instance.is_relationship_element());
    assert!(instance.as_relationship_element().is_none());
    assert!(!instance.is_property());
    assert!(instance.as_property().is_none());
    assert!(!instance.is_blob());
    assert!(instance.as_blob().is_none());
}

#[test]
fn test_casts_over_property() {
    let instance = common::load_minimal_of("Property", |c| c.is_property());

    assert!(instance.is_property());
    assert!(instance.as_property().is_some());

    assert!(!instance.is_extension());
    assert!(instance.as_extension().is_none());
    assert!(!instance.is_submodel());
    assert!(instance.as_submodel().is_none());
    assert!(!instance.is_range());
    assert!(instance.as_range().is_none());
    assert!(!instance.is_blob());
    assert!(instance.as_blob().is_none());
}

#[test]
fn test_casts_over_submodel() {
    let instance = common::load_minimal_of("Submodel", |c| c.is_submodel());

    assert!(instance.is_submodel());
    assert!(instance.as_submodel().is_some());

    assert!(!instance.is_extension());
    assert!(!instance.is_property());
    assert!(!instance.is_asset_administration_shell());
    assert!(!instance.is_concept_description());
}

#[test]
fn test_casts_over_asset_administration_shell() {
    let instance = common::load_minimal_of("AssetAdministrationShell", |c| {
        c.is_asset_administration_shell()
    });

    assert!(instance.is_asset_administration_shell());
    assert!(instance.as_asset_administration_shell().is_some());

    assert!(!instance.is_submodel());
    assert!(!instance.is_concept_description());
}

#[test]
fn test_casts_over_concept_description() {
    let instance = common::load_minimal_of("ConceptDescription", |c| c.is_concept_description());

    assert!(instance.is_concept_description());
    assert!(instance.as_concept_description().is_some());

    assert!(!instance.is_submodel());
    assert!(!instance.is_asset_administration_shell());
}

#[test]
fn test_casts_over_blob() {
    let instance = common::load_minimal_of("Blob", |c| c.is_blob());

    assert!(instance.is_blob());
    assert!(instance.as_blob().is_some());

    assert!(!instance.is_file());
    assert!(!instance.is_property());
}

#[test]
fn test_casts_over_file() {
    let instance = common::load_minimal_of("File", |c| c.is_file());

    assert!(instance.is_file());
    assert!(instance.as_file().is_some());

    assert!(!instance.is_blob());
    assert!(!instance.is_property());
}

#[test]
fn test_casts_over_reference() {
    let instance = common::load_minimal_of("Reference", |c| c.is_reference());

    assert!(instance.is_reference());
    assert!(instance.as_reference().is_some());

    assert!(!instance.is_key());
}

#[test]
fn test_casts_over_key() {
    let instance = common::load_minimal_of("Key", |c| c.is_key());

    assert!(instance.is_key());
    assert!(instance.as_key().is_some());

    assert!(!instance.is_reference());
}

#[test]
fn test_casts_over_annotated_relationship_element() {
    let instance = common::load_minimal_of("AnnotatedRelationshipElement", |c| {
        c.is_annotated_relationship_element()
    });

    assert!(instance.is_annotated_relationship_element());
    assert!(instance.as_annotated_relationship_element().is_some());

    // TypeScript: AnnotatedRelationshipElement IS NOT RelationshipElement
    // (they are separate concrete types in Rust)
    assert!(!instance.is_relationship_element());
}

#[test]
fn test_casts_over_operation() {
    let instance = common::load_minimal_of("Operation", |c| c.is_operation());

    assert!(instance.is_operation());
    assert!(instance.as_operation().is_some());

    assert!(!instance.is_capability());
    assert!(!instance.is_entity());
}

#[test]
fn test_casts_over_data_specification_iec61360() {
    let instance = common::load_minimal_of("DataSpecificationIec61360", |c| {
        c.is_data_specification_iec61360()
    });

    assert!(instance.is_data_specification_iec61360());
    assert!(instance.as_data_specification_iec61360().is_some());

    assert!(!instance.is_embedded_data_specification());
    assert!(!instance.is_extension());
}
