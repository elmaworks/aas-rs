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

/// Test that JSON files in `ContainedInEnvironment/Unexpected/Invalid/{causeDir}/{type_name}`
/// deserialize successfully but fail verification, with errors matching golden `.json.errors` files.
fn invalid_test(type_name: &str) {
    let invalid_root = common::test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Unexpected")
        .join("Invalid");

    for cause_dir in common::find_immediate_subdirectories(&invalid_root) {
        let cls_dir = cause_dir.join(type_name);
        if !cls_dir.exists() {
            continue;
        }

        let json_files = common::find_json_files(&cls_dir);
        for path in &json_files {
            let original = common::load_json(path);
            let env = jsonization::environment_from_jsonable_value(&original).unwrap_or_else(|e| {
                panic!(
                    "Expected deserialization to succeed for {}: {e}",
                    path.display()
                )
            });

            let env_class = aas_rs::types::class::Class::Environment(env);
            let errors = verification::verify(&env_class, true);

            // Format like TypeScript: each error as "{path}: {message}", joined by \n, trailing \n
            let mut lines: Vec<String> = errors.iter().map(|e| format!("{e}")).collect();
            lines.push(String::new());
            let got = lines.join("\n");

            let golden_path = std::path::PathBuf::from(format!("{}.errors", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got,
                expected,
                "Verification errors mismatch for {}",
                path.display()
            );
        }
    }
}

/// Test that JSON files in `ContainedInEnvironment/Unexpected/Unserializable/{causeDir}/{type_name}`
/// fail deserialization with errors matching golden `.json.error` files.
fn unserializable_test(type_name: &str) {
    let unserializable_root = common::test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Unexpected")
        .join("Unserializable");

    for cause_dir in common::find_immediate_subdirectories(&unserializable_root) {
        if cause_dir
            .file_name()
            .map(|n| n == "UnexpectedAdditionalProperty")
            .unwrap_or(false)
        {
            continue;
        }

        let cls_dir = cause_dir.join(type_name);
        if !cls_dir.exists() {
            continue;
        }

        let json_files = common::find_json_files(&cls_dir);
        for path in &json_files {
            let original = common::load_json(path);
            let error = match jsonization::environment_from_jsonable_value(&original) {
                Err(e) => e,
                Ok(_) => panic!("Expected deserialization to fail for {}", path.display()),
            };

            // Rust path starts with "." for Property segments; golden files omit the leading ".".
            let rust_output = format!("{error}\n");
            let got = rust_output.strip_prefix('.').unwrap_or(&rust_output);

            let golden_path = std::path::PathBuf::from(format!("{}.error", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got,
                expected,
                "Deserialization error mismatch for {}",
                path.display()
            );
        }
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
