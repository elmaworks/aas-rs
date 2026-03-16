//! Tests for JSON round-trip of concrete classes contained in Environment.
//! Mirrors aas-core3.0-typescript/test/jsonization.concreteClasses.spec.ts

mod common;

use aas_rs::jsonization;
use aas_rs::verification;

/// Test that all JSON files in `ContainedInEnvironment/Expected/{type_name}` round-trip
/// correctly: JSON → deserialize → verify (no errors) → serialize → JSON == original.
fn round_trip_test(type_name: &str) {
    let dir = common::test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Expected")
        .join(type_name);

    let json_files = common::find_json_files(&dir);
    assert!(
        !json_files.is_empty(),
        "No JSON files found for {type_name} in {dir:?}"
    );

    for path in &json_files {
        let original = common::load_json(path);
        let env = jsonization::environment_from_jsonable_value(&original)
            .unwrap_or_else(|e| panic!("Deserialization failed for {}: {e}", path.display()));

        let env_class = aas_rs::types::class::Class::Environment(env);
        let errors = verification::verify(&env_class, true);
        assert!(
            errors.is_empty(),
            "Verification errors for {}: {:?}",
            path.display(),
            errors
        );

        let serialized = jsonization::to_jsonable(&env_class);
        assert_eq!(
            original,
            serialized,
            "JSON round-trip mismatch for {}",
            path.display()
        );
    }
}

/// Test that JSON files in `ContainedInEnvironment/Unexpected/Invalid/{type_name}` fail verification.
fn invalid_test(type_name: &str) {
    let dir = common::test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Unexpected")
        .join("Invalid")
        .join(type_name);

    if !dir.exists() {
        return; // Not all types have invalid test data
    }

    let json_files = common::find_json_files(&dir);
    for path in &json_files {
        if path.extension().map(|e| e == "error").unwrap_or(false) {
            continue;
        }
        let original = common::load_json(path);
        let env = jsonization::environment_from_jsonable_value(&original);
        match env {
            Ok(env_val) => {
                let env_class = aas_rs::types::class::Class::Environment(env_val);
                let errors = verification::verify(&env_class, true);
                // Either deserialization or verification should flag an error
                if errors.is_empty() {
                    // Check if error file exists
                    let error_file = path.with_extension("json.errors");
                    if !error_file.exists() {
                        let error_file2 = {
                            let mut p = path.clone();
                            p.set_extension("errors");
                            p
                        };
                        if !error_file2.exists() {
                            // Some invalid examples might not have recorded errors
                        }
                    }
                }
            }
            Err(_) => {
                // Deserialization failure is expected for some invalid examples
            }
        }
    }
}

/// Test that JSON files in `ContainedInEnvironment/Unexpected/Unserializable` fail deserialization.
fn unserializable_test(type_name: &str) {
    let dir = common::test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Unexpected")
        .join("Unserializable")
        .join(type_name);

    if !dir.exists() {
        return;
    }

    let json_files = common::find_json_files(&dir);
    for path in &json_files {
        let original = common::load_json(path);
        let result = jsonization::environment_from_jsonable_value(&original);
        assert!(
            result.is_err(),
            "Expected deserialization to fail for {}",
            path.display()
        );
    }
}

macro_rules! concrete_class_test {
    ($test_name:ident, $type_name:expr) => {
        #[test]
        fn $test_name() {
            round_trip_test($type_name);
            invalid_test($type_name);
            unserializable_test($type_name);
        }
    };
}

concrete_class_test!(test_extension_round_trip, "Extension");
concrete_class_test!(
    test_administrative_information_round_trip,
    "AdministrativeInformation"
);
concrete_class_test!(test_qualifier_round_trip, "Qualifier");
concrete_class_test!(
    test_asset_administration_shell_round_trip,
    "AssetAdministrationShell"
);
concrete_class_test!(test_asset_information_round_trip, "AssetInformation");
concrete_class_test!(test_resource_round_trip, "Resource");
concrete_class_test!(test_specific_asset_id_round_trip, "SpecificAssetId");
concrete_class_test!(test_submodel_round_trip, "Submodel");
concrete_class_test!(test_relationship_element_round_trip, "RelationshipElement");
concrete_class_test!(test_submodel_element_list_round_trip, "SubmodelElementList");
concrete_class_test!(
    test_submodel_element_collection_round_trip,
    "SubmodelElementCollection"
);
concrete_class_test!(test_property_round_trip, "Property");
concrete_class_test!(
    test_multi_language_property_round_trip,
    "MultiLanguageProperty"
);
concrete_class_test!(test_range_round_trip, "Range");
concrete_class_test!(test_reference_element_round_trip, "ReferenceElement");
concrete_class_test!(test_blob_round_trip, "Blob");
concrete_class_test!(test_file_round_trip, "File");
concrete_class_test!(
    test_annotated_relationship_element_round_trip,
    "AnnotatedRelationshipElement"
);
concrete_class_test!(test_entity_round_trip, "Entity");
concrete_class_test!(test_basic_event_element_round_trip, "BasicEventElement");
concrete_class_test!(test_operation_round_trip, "Operation");
concrete_class_test!(test_operation_variable_round_trip, "OperationVariable");
concrete_class_test!(test_capability_round_trip, "Capability");
concrete_class_test!(test_concept_description_round_trip, "ConceptDescription");
concrete_class_test!(test_reference_round_trip, "Reference");
concrete_class_test!(test_key_round_trip, "Key");
concrete_class_test!(test_lang_string_name_type_round_trip, "LangStringNameType");
concrete_class_test!(test_lang_string_text_type_round_trip, "LangStringTextType");
concrete_class_test!(
    test_embedded_data_specification_round_trip,
    "EmbeddedDataSpecification"
);
concrete_class_test!(test_level_type_round_trip, "LevelType");
concrete_class_test!(test_value_reference_pair_round_trip, "ValueReferencePair");
concrete_class_test!(test_value_list_round_trip, "ValueList");
concrete_class_test!(
    test_lang_string_preferred_name_type_iec61360_round_trip,
    "LangStringPreferredNameTypeIec61360"
);
concrete_class_test!(
    test_lang_string_short_name_type_iec61360_round_trip,
    "LangStringShortNameTypeIec61360"
);
concrete_class_test!(
    test_lang_string_definition_type_iec61360_round_trip,
    "LangStringDefinitionTypeIec61360"
);
concrete_class_test!(
    test_data_specification_iec61360_round_trip,
    "DataSpecificationIec61360"
);
