//! Tests for `over_*_or_empty` iteration patterns.
//! Mirrors aas-core3.0-typescript/test/types.overXOrEmpty.spec.ts
//!
//! Since Rust does not have `overXOrEmpty()` methods, we use
//! `Option<Vec<T>>.as_deref().unwrap_or_default()` equivalents.

mod common;

/// Assert that the optional slice has the expected item count.
fn assert_count(opt: Option<&[impl Sized]>, expected: usize) {
    let count = opt.map(|s| s.len()).unwrap_or(0);
    assert_eq!(count, expected);
}

/// Generates a non-default test: loads maximal, checks field is Some.
macro_rules! over_non_default {
    ($test_name:ident, $load_expr:expr, $as_fn:ident, $field:ident) => {
        #[test]
        fn $test_name() {
            let instance = $load_expr;
            let inner = instance.$as_fn().unwrap();
            let opt = inner.$field.as_deref();
            assert!(
                opt.is_some(),
                "{} should be set in maximal example",
                stringify!($field)
            );
            assert_count(opt, opt.unwrap().len());
        }
    };
}

/// Generates a default test: loads minimal, checks field is None.
macro_rules! over_default {
    ($test_name:ident, $load_expr:expr, $as_fn:ident, $field:ident) => {
        #[test]
        fn $test_name() {
            let instance = $load_expr;
            let inner = instance.$as_fn().unwrap();
            assert!(
                inner.$field.is_none(),
                "{} should be None in minimal example",
                stringify!($field)
            );
            assert_count(inner.$field.as_deref(), 0);
        }
    };
}

/// Generates both non-default and default tests using ContainedInEnvironment data.
macro_rules! over_both {
    (
        $non_default:ident, $default:ident,
        $type_name:expr, $is_fn:ident, $as_fn:ident, $field:ident
    ) => {
        over_non_default!(
            $non_default,
            common::load_maximal_of($type_name, |c| c.$is_fn()),
            $as_fn,
            $field
        );
        over_default!(
            $default,
            common::load_minimal_of($type_name, |c| c.$is_fn()),
            $as_fn,
            $field
        );
    };
}

// ── Extension ──────────────────────────────────────────────────────────────────

over_both!(
    test_extension_over_supplemental_semantic_ids_or_empty_non_default,
    test_extension_over_supplemental_semantic_ids_or_empty_default,
    "Extension", is_extension, as_extension, supplemental_semantic_ids
);

over_both!(
    test_extension_over_refers_to_or_empty_non_default,
    test_extension_over_refers_to_or_empty_default,
    "Extension", is_extension, as_extension, refers_to
);

// ── AdministrativeInformation ────────────────────────────────────────────────

over_both!(
    test_administrative_information_over_embedded_data_specifications_non_default,
    test_administrative_information_over_embedded_data_specifications_default,
    "AdministrativeInformation",
    is_administrative_information,
    as_administrative_information,
    embedded_data_specifications
);

// ── Qualifier ────────────────────────────────────────────────────────────────

over_both!(
    test_qualifier_over_supplemental_semantic_ids_non_default,
    test_qualifier_over_supplemental_semantic_ids_default,
    "Qualifier", is_qualifier, as_qualifier, supplemental_semantic_ids
);

// ── AssetAdministrationShell ─────────────────────────────────────────────────

over_both!(
    test_aas_over_extensions_non_default,
    test_aas_over_extensions_default,
    "AssetAdministrationShell",
    is_asset_administration_shell,
    as_asset_administration_shell,
    extensions
);

over_both!(
    test_aas_over_display_name_non_default,
    test_aas_over_display_name_default,
    "AssetAdministrationShell",
    is_asset_administration_shell,
    as_asset_administration_shell,
    display_name
);

over_both!(
    test_aas_over_description_non_default,
    test_aas_over_description_default,
    "AssetAdministrationShell",
    is_asset_administration_shell,
    as_asset_administration_shell,
    description
);

over_both!(
    test_aas_over_embedded_data_specifications_non_default,
    test_aas_over_embedded_data_specifications_default,
    "AssetAdministrationShell",
    is_asset_administration_shell,
    as_asset_administration_shell,
    embedded_data_specifications
);

over_both!(
    test_aas_over_submodels_non_default,
    test_aas_over_submodels_default,
    "AssetAdministrationShell",
    is_asset_administration_shell,
    as_asset_administration_shell,
    submodels
);

// ── AssetInformation ─────────────────────────────────────────────────────────

// Note: non-default test is not generated in the TypeScript test suite.
over_default!(
    test_asset_information_over_specific_asset_ids_or_empty_default,
    common::load_minimal_of("AssetInformation", |c| c.is_asset_information()),
    as_asset_information,
    specific_asset_ids
);

// ── SpecificAssetId ──────────────────────────────────────────────────────────

over_both!(
    test_specific_asset_id_over_supplemental_semantic_ids_non_default,
    test_specific_asset_id_over_supplemental_semantic_ids_default,
    "SpecificAssetId", is_specific_asset_id, as_specific_asset_id, supplemental_semantic_ids
);

// ── Submodel ─────────────────────────────────────────────────────────────────

over_both!(
    test_submodel_over_extensions_non_default,
    test_submodel_over_extensions_default,
    "Submodel", is_submodel, as_submodel, extensions
);

over_both!(
    test_submodel_over_display_name_non_default,
    test_submodel_over_display_name_default,
    "Submodel", is_submodel, as_submodel, display_name
);

over_both!(
    test_submodel_over_description_non_default,
    test_submodel_over_description_default,
    "Submodel", is_submodel, as_submodel, description
);

over_both!(
    test_submodel_over_supplemental_semantic_ids_non_default,
    test_submodel_over_supplemental_semantic_ids_default,
    "Submodel", is_submodel, as_submodel, supplemental_semantic_ids
);

over_both!(
    test_submodel_over_qualifiers_non_default,
    test_submodel_over_qualifiers_default,
    "Submodel", is_submodel, as_submodel, qualifiers
);

over_both!(
    test_submodel_over_embedded_data_specifications_non_default,
    test_submodel_over_embedded_data_specifications_default,
    "Submodel", is_submodel, as_submodel, embedded_data_specifications
);

over_both!(
    test_submodel_over_submodel_elements_non_default,
    test_submodel_over_submodel_elements_default,
    "Submodel", is_submodel, as_submodel, submodel_elements
);

// ── RelationshipElement ───────────────────────────────────────────────────────

over_both!(
    test_relationship_element_over_extensions_non_default,
    test_relationship_element_over_extensions_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    extensions
);

over_both!(
    test_relationship_element_over_display_name_non_default,
    test_relationship_element_over_display_name_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    display_name
);

over_both!(
    test_relationship_element_over_description_non_default,
    test_relationship_element_over_description_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    description
);

over_both!(
    test_relationship_element_over_supplemental_semantic_ids_non_default,
    test_relationship_element_over_supplemental_semantic_ids_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    supplemental_semantic_ids
);

over_both!(
    test_relationship_element_over_qualifiers_non_default,
    test_relationship_element_over_qualifiers_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    qualifiers
);

over_both!(
    test_relationship_element_over_embedded_data_specifications_non_default,
    test_relationship_element_over_embedded_data_specifications_default,
    "RelationshipElement",
    is_relationship_element,
    as_relationship_element,
    embedded_data_specifications
);

// ── SubmodelElementList ───────────────────────────────────────────────────────

over_both!(
    test_submodel_element_list_over_extensions_non_default,
    test_submodel_element_list_over_extensions_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    extensions
);

over_both!(
    test_submodel_element_list_over_display_name_non_default,
    test_submodel_element_list_over_display_name_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    display_name
);

over_both!(
    test_submodel_element_list_over_description_non_default,
    test_submodel_element_list_over_description_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    description
);

over_both!(
    test_submodel_element_list_over_supplemental_semantic_ids_non_default,
    test_submodel_element_list_over_supplemental_semantic_ids_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    supplemental_semantic_ids
);

over_both!(
    test_submodel_element_list_over_qualifiers_non_default,
    test_submodel_element_list_over_qualifiers_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    qualifiers
);

over_both!(
    test_submodel_element_list_over_embedded_data_specifications_non_default,
    test_submodel_element_list_over_embedded_data_specifications_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    embedded_data_specifications
);

over_both!(
    test_submodel_element_list_over_value_non_default,
    test_submodel_element_list_over_value_default,
    "SubmodelElementList",
    is_submodel_element_list,
    as_submodel_element_list,
    value
);

// ── SubmodelElementCollection ─────────────────────────────────────────────────

over_both!(
    test_submodel_element_collection_over_extensions_non_default,
    test_submodel_element_collection_over_extensions_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    extensions
);

over_both!(
    test_submodel_element_collection_over_display_name_non_default,
    test_submodel_element_collection_over_display_name_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    display_name
);

over_both!(
    test_submodel_element_collection_over_description_non_default,
    test_submodel_element_collection_over_description_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    description
);

over_both!(
    test_submodel_element_collection_over_supplemental_semantic_ids_non_default,
    test_submodel_element_collection_over_supplemental_semantic_ids_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    supplemental_semantic_ids
);

over_both!(
    test_submodel_element_collection_over_qualifiers_non_default,
    test_submodel_element_collection_over_qualifiers_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    qualifiers
);

over_both!(
    test_submodel_element_collection_over_embedded_data_specifications_non_default,
    test_submodel_element_collection_over_embedded_data_specifications_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    embedded_data_specifications
);

over_both!(
    test_submodel_element_collection_over_value_non_default,
    test_submodel_element_collection_over_value_default,
    "SubmodelElementCollection",
    is_submodel_element_collection,
    as_submodel_element_collection,
    value
);

// ── Property ─────────────────────────────────────────────────────────────────

over_both!(
    test_property_over_extensions_non_default,
    test_property_over_extensions_default,
    "Property", is_property, as_property, extensions
);

over_both!(
    test_property_over_display_name_non_default,
    test_property_over_display_name_default,
    "Property", is_property, as_property, display_name
);

over_both!(
    test_property_over_description_non_default,
    test_property_over_description_default,
    "Property", is_property, as_property, description
);

over_both!(
    test_property_over_supplemental_semantic_ids_non_default,
    test_property_over_supplemental_semantic_ids_default,
    "Property", is_property, as_property, supplemental_semantic_ids
);

over_both!(
    test_property_over_qualifiers_non_default,
    test_property_over_qualifiers_default,
    "Property", is_property, as_property, qualifiers
);

over_both!(
    test_property_over_embedded_data_specifications_non_default,
    test_property_over_embedded_data_specifications_default,
    "Property", is_property, as_property, embedded_data_specifications
);

// ── MultiLanguageProperty ────────────────────────────────────────────────────

over_both!(
    test_multi_language_property_over_extensions_non_default,
    test_multi_language_property_over_extensions_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    extensions
);

over_both!(
    test_multi_language_property_over_display_name_non_default,
    test_multi_language_property_over_display_name_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    display_name
);

over_both!(
    test_multi_language_property_over_description_non_default,
    test_multi_language_property_over_description_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    description
);

over_both!(
    test_multi_language_property_over_supplemental_semantic_ids_non_default,
    test_multi_language_property_over_supplemental_semantic_ids_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    supplemental_semantic_ids
);

over_both!(
    test_multi_language_property_over_qualifiers_non_default,
    test_multi_language_property_over_qualifiers_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    qualifiers
);

over_both!(
    test_multi_language_property_over_embedded_data_specifications_non_default,
    test_multi_language_property_over_embedded_data_specifications_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    embedded_data_specifications
);

over_both!(
    test_multi_language_property_over_value_non_default,
    test_multi_language_property_over_value_default,
    "MultiLanguageProperty",
    is_multi_language_property,
    as_multi_language_property,
    value
);

// ── Range ─────────────────────────────────────────────────────────────────────

over_both!(
    test_range_over_extensions_non_default,
    test_range_over_extensions_default,
    "Range", is_range, as_range, extensions
);

over_both!(
    test_range_over_display_name_non_default,
    test_range_over_display_name_default,
    "Range", is_range, as_range, display_name
);

over_both!(
    test_range_over_description_non_default,
    test_range_over_description_default,
    "Range", is_range, as_range, description
);

over_both!(
    test_range_over_supplemental_semantic_ids_non_default,
    test_range_over_supplemental_semantic_ids_default,
    "Range", is_range, as_range, supplemental_semantic_ids
);

over_both!(
    test_range_over_qualifiers_non_default,
    test_range_over_qualifiers_default,
    "Range", is_range, as_range, qualifiers
);

over_both!(
    test_range_over_embedded_data_specifications_non_default,
    test_range_over_embedded_data_specifications_default,
    "Range", is_range, as_range, embedded_data_specifications
);

// ── ReferenceElement ──────────────────────────────────────────────────────────

over_both!(
    test_reference_element_over_extensions_non_default,
    test_reference_element_over_extensions_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    extensions
);

over_both!(
    test_reference_element_over_display_name_non_default,
    test_reference_element_over_display_name_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    display_name
);

over_both!(
    test_reference_element_over_description_non_default,
    test_reference_element_over_description_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    description
);

over_both!(
    test_reference_element_over_supplemental_semantic_ids_non_default,
    test_reference_element_over_supplemental_semantic_ids_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    supplemental_semantic_ids
);

over_both!(
    test_reference_element_over_qualifiers_non_default,
    test_reference_element_over_qualifiers_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    qualifiers
);

over_both!(
    test_reference_element_over_embedded_data_specifications_non_default,
    test_reference_element_over_embedded_data_specifications_default,
    "ReferenceElement",
    is_reference_element,
    as_reference_element,
    embedded_data_specifications
);

// ── Blob ──────────────────────────────────────────────────────────────────────

over_both!(
    test_blob_over_extensions_non_default,
    test_blob_over_extensions_default,
    "Blob", is_blob, as_blob, extensions
);

over_both!(
    test_blob_over_display_name_non_default,
    test_blob_over_display_name_default,
    "Blob", is_blob, as_blob, display_name
);

over_both!(
    test_blob_over_description_non_default,
    test_blob_over_description_default,
    "Blob", is_blob, as_blob, description
);

over_both!(
    test_blob_over_supplemental_semantic_ids_non_default,
    test_blob_over_supplemental_semantic_ids_default,
    "Blob", is_blob, as_blob, supplemental_semantic_ids
);

over_both!(
    test_blob_over_qualifiers_non_default,
    test_blob_over_qualifiers_default,
    "Blob", is_blob, as_blob, qualifiers
);

over_both!(
    test_blob_over_embedded_data_specifications_non_default,
    test_blob_over_embedded_data_specifications_default,
    "Blob", is_blob, as_blob, embedded_data_specifications
);

// ── File ──────────────────────────────────────────────────────────────────────

over_both!(
    test_file_over_extensions_non_default,
    test_file_over_extensions_default,
    "File", is_file, as_file, extensions
);

over_both!(
    test_file_over_display_name_non_default,
    test_file_over_display_name_default,
    "File", is_file, as_file, display_name
);

over_both!(
    test_file_over_description_non_default,
    test_file_over_description_default,
    "File", is_file, as_file, description
);

over_both!(
    test_file_over_supplemental_semantic_ids_non_default,
    test_file_over_supplemental_semantic_ids_default,
    "File", is_file, as_file, supplemental_semantic_ids
);

over_both!(
    test_file_over_qualifiers_non_default,
    test_file_over_qualifiers_default,
    "File", is_file, as_file, qualifiers
);

over_both!(
    test_file_over_embedded_data_specifications_non_default,
    test_file_over_embedded_data_specifications_default,
    "File", is_file, as_file, embedded_data_specifications
);

// ── AnnotatedRelationshipElement ──────────────────────────────────────────────

over_both!(
    test_annotated_relationship_element_over_extensions_non_default,
    test_annotated_relationship_element_over_extensions_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    extensions
);

over_both!(
    test_annotated_relationship_element_over_display_name_non_default,
    test_annotated_relationship_element_over_display_name_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    display_name
);

over_both!(
    test_annotated_relationship_element_over_description_non_default,
    test_annotated_relationship_element_over_description_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    description
);

over_both!(
    test_annotated_relationship_element_over_supplemental_semantic_ids_non_default,
    test_annotated_relationship_element_over_supplemental_semantic_ids_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    supplemental_semantic_ids
);

over_both!(
    test_annotated_relationship_element_over_qualifiers_non_default,
    test_annotated_relationship_element_over_qualifiers_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    qualifiers
);

over_both!(
    test_annotated_relationship_element_over_embedded_data_specifications_non_default,
    test_annotated_relationship_element_over_embedded_data_specifications_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    embedded_data_specifications
);

over_both!(
    test_annotated_relationship_element_over_annotations_non_default,
    test_annotated_relationship_element_over_annotations_default,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element,
    as_annotated_relationship_element,
    annotations
);

// ── Entity ───────────────────────────────────────────────────────────────────

over_both!(
    test_entity_over_extensions_non_default,
    test_entity_over_extensions_default,
    "Entity", is_entity, as_entity, extensions
);

over_both!(
    test_entity_over_display_name_non_default,
    test_entity_over_display_name_default,
    "Entity", is_entity, as_entity, display_name
);

over_both!(
    test_entity_over_description_non_default,
    test_entity_over_description_default,
    "Entity", is_entity, as_entity, description
);

over_both!(
    test_entity_over_supplemental_semantic_ids_non_default,
    test_entity_over_supplemental_semantic_ids_default,
    "Entity", is_entity, as_entity, supplemental_semantic_ids
);

over_both!(
    test_entity_over_qualifiers_non_default,
    test_entity_over_qualifiers_default,
    "Entity", is_entity, as_entity, qualifiers
);

over_both!(
    test_entity_over_embedded_data_specifications_non_default,
    test_entity_over_embedded_data_specifications_default,
    "Entity", is_entity, as_entity, embedded_data_specifications
);

over_both!(
    test_entity_over_statements_non_default,
    test_entity_over_statements_default,
    "Entity", is_entity, as_entity, statements
);

// Note: non-default test is not generated in the TypeScript test suite.
over_default!(
    test_entity_over_specific_asset_ids_or_empty_default,
    common::load_minimal_of("Entity", |c| c.is_entity()),
    as_entity,
    specific_asset_ids
);

// ── BasicEventElement ─────────────────────────────────────────────────────────

over_both!(
    test_basic_event_element_over_extensions_non_default,
    test_basic_event_element_over_extensions_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    extensions
);

over_both!(
    test_basic_event_element_over_display_name_non_default,
    test_basic_event_element_over_display_name_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    display_name
);

over_both!(
    test_basic_event_element_over_description_non_default,
    test_basic_event_element_over_description_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    description
);

over_both!(
    test_basic_event_element_over_supplemental_semantic_ids_non_default,
    test_basic_event_element_over_supplemental_semantic_ids_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    supplemental_semantic_ids
);

over_both!(
    test_basic_event_element_over_qualifiers_non_default,
    test_basic_event_element_over_qualifiers_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    qualifiers
);

over_both!(
    test_basic_event_element_over_embedded_data_specifications_non_default,
    test_basic_event_element_over_embedded_data_specifications_default,
    "BasicEventElement",
    is_basic_event_element,
    as_basic_event_element,
    embedded_data_specifications
);

// ── Operation ────────────────────────────────────────────────────────────────

over_both!(
    test_operation_over_extensions_non_default,
    test_operation_over_extensions_default,
    "Operation", is_operation, as_operation, extensions
);

over_both!(
    test_operation_over_display_name_non_default,
    test_operation_over_display_name_default,
    "Operation", is_operation, as_operation, display_name
);

over_both!(
    test_operation_over_description_non_default,
    test_operation_over_description_default,
    "Operation", is_operation, as_operation, description
);

over_both!(
    test_operation_over_supplemental_semantic_ids_non_default,
    test_operation_over_supplemental_semantic_ids_default,
    "Operation", is_operation, as_operation, supplemental_semantic_ids
);

over_both!(
    test_operation_over_qualifiers_non_default,
    test_operation_over_qualifiers_default,
    "Operation", is_operation, as_operation, qualifiers
);

over_both!(
    test_operation_over_embedded_data_specifications_non_default,
    test_operation_over_embedded_data_specifications_default,
    "Operation", is_operation, as_operation, embedded_data_specifications
);

over_both!(
    test_operation_over_input_variables_non_default,
    test_operation_over_input_variables_default,
    "Operation", is_operation, as_operation, input_variables
);

over_both!(
    test_operation_over_output_variables_non_default,
    test_operation_over_output_variables_default,
    "Operation", is_operation, as_operation, output_variables
);

over_both!(
    test_operation_over_inoutput_variables_non_default,
    test_operation_over_inoutput_variables_default,
    "Operation", is_operation, as_operation, inoutput_variables
);

// ── Capability ────────────────────────────────────────────────────────────────

over_both!(
    test_capability_over_extensions_non_default,
    test_capability_over_extensions_default,
    "Capability", is_capability, as_capability, extensions
);

over_both!(
    test_capability_over_display_name_non_default,
    test_capability_over_display_name_default,
    "Capability", is_capability, as_capability, display_name
);

over_both!(
    test_capability_over_description_non_default,
    test_capability_over_description_default,
    "Capability", is_capability, as_capability, description
);

over_both!(
    test_capability_over_supplemental_semantic_ids_non_default,
    test_capability_over_supplemental_semantic_ids_default,
    "Capability", is_capability, as_capability, supplemental_semantic_ids
);

over_both!(
    test_capability_over_qualifiers_non_default,
    test_capability_over_qualifiers_default,
    "Capability", is_capability, as_capability, qualifiers
);

over_both!(
    test_capability_over_embedded_data_specifications_non_default,
    test_capability_over_embedded_data_specifications_default,
    "Capability", is_capability, as_capability, embedded_data_specifications
);

// ── ConceptDescription ───────────────────────────────────────────────────────

over_both!(
    test_concept_description_over_extensions_non_default,
    test_concept_description_over_extensions_default,
    "ConceptDescription",
    is_concept_description,
    as_concept_description,
    extensions
);

over_both!(
    test_concept_description_over_display_name_non_default,
    test_concept_description_over_display_name_default,
    "ConceptDescription",
    is_concept_description,
    as_concept_description,
    display_name
);

over_both!(
    test_concept_description_over_description_non_default,
    test_concept_description_over_description_default,
    "ConceptDescription",
    is_concept_description,
    as_concept_description,
    description
);

over_both!(
    test_concept_description_over_embedded_data_specifications_non_default,
    test_concept_description_over_embedded_data_specifications_default,
    "ConceptDescription",
    is_concept_description,
    as_concept_description,
    embedded_data_specifications
);

over_both!(
    test_concept_description_over_is_case_of_non_default,
    test_concept_description_over_is_case_of_default,
    "ConceptDescription",
    is_concept_description,
    as_concept_description,
    is_case_of
);

// ── Environment ───────────────────────────────────────────────────────────────

over_non_default!(
    test_environment_over_asset_administration_shells_non_default,
    common::load_maximal_environment(),
    as_environment,
    asset_administration_shells
);

over_default!(
    test_environment_over_asset_administration_shells_default,
    common::load_minimal_environment(),
    as_environment,
    asset_administration_shells
);

over_non_default!(
    test_environment_over_submodels_non_default,
    common::load_maximal_environment(),
    as_environment,
    submodels
);

over_default!(
    test_environment_over_submodels_default,
    common::load_minimal_environment(),
    as_environment,
    submodels
);

over_non_default!(
    test_environment_over_concept_descriptions_non_default,
    common::load_maximal_environment(),
    as_environment,
    concept_descriptions
);

over_default!(
    test_environment_over_concept_descriptions_default,
    common::load_minimal_environment(),
    as_environment,
    concept_descriptions
);

// ── DataSpecificationIec61360 ─────────────────────────────────────────────────

over_both!(
    test_data_specification_iec61360_over_short_name_non_default,
    test_data_specification_iec61360_over_short_name_default,
    "DataSpecificationIec61360",
    is_data_specification_iec61360,
    as_data_specification_iec61360,
    short_name
);

over_both!(
    test_data_specification_iec61360_over_definition_non_default,
    test_data_specification_iec61360_over_definition_default,
    "DataSpecificationIec61360",
    is_data_specification_iec61360,
    as_data_specification_iec61360,
    definition
);
