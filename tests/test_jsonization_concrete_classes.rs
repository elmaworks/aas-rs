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

// ─── Self-contained types (Environment & EventPayload) ──────────────────────

/// Round-trip test for self-contained Environment:
/// JSON → deserialize → verify → serialize → compare.
fn self_contained_environment_round_trip() {
    let dir = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("Environment");

    let json_files = common::find_json_files(&dir);
    assert!(!json_files.is_empty(), "No JSON files found in {dir:?}");

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
            original, serialized,
            "JSON round-trip mismatch for {}",
            path.display()
        );
    }
}

/// Format verification errors as "{path}: {message}" with trailing newline,
/// matching the TypeScript golden file format (empty path still gets ": ").
fn format_verification_errors(errors: &[verification::VerificationError]) -> String {
    let mut lines: Vec<String> = errors
        .iter()
        .map(|e| format!("{}: {}", e.path, e.message))
        .collect();
    lines.push(String::new());
    lines.join("\n")
}

/// Format a deserialization error as "{path}: {message}\n",
/// matching the TypeScript golden file format.
/// The Rust path uses "." prefixed segments (e.g., ".prop"); we strip the leading ".".
fn format_deserialization_error(error: &jsonization::DeserializationError) -> String {
    let path_str: String = error
        .path
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let path_str = path_str.strip_prefix('.').unwrap_or(&path_str);
    format!("{path_str}: {}\n", error.message)
}

/// Invalid test for self-contained Environment.
fn self_contained_environment_invalid() {
    let invalid_root = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Unexpected")
        .join("Invalid");

    for cause_dir in common::find_immediate_subdirectories(&invalid_root) {
        let cls_dir = cause_dir.join("Environment");
        if !cls_dir.exists() {
            continue;
        }

        let json_files = common::find_json_files(&cls_dir);
        for path in &json_files {
            let original = common::load_json(path);
            let env =
                jsonization::environment_from_jsonable_value(&original).unwrap_or_else(|e| {
                    panic!(
                        "Expected deserialization to succeed for {}: {e}",
                        path.display()
                    )
                });

            let env_class = aas_rs::types::class::Class::Environment(env);
            let errors = verification::verify(&env_class, true);
            let got = format_verification_errors(&errors);

            let golden_path = std::path::PathBuf::from(format!("{}.errors", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got, expected,
                "Verification errors mismatch for {}",
                path.display()
            );
        }
    }
}

/// Unserializable test for self-contained Environment.
fn self_contained_environment_unserializable() {
    let unserializable_root = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
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

        let cls_dir = cause_dir.join("Environment");
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

            let got = format_deserialization_error(&error);

            let golden_path = std::path::PathBuf::from(format!("{}.error", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got, expected,
                "Deserialization error mismatch for {}",
                path.display()
            );
        }
    }
}

/// Round-trip test for self-contained EventPayload.
/// Note: The test data JSON files do not include "modelType", but our serializer
/// adds it. So we compare: deserialize → serialize → deserialize → serialize.
fn self_contained_event_payload_round_trip() {
    let dir = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("EventPayload");

    let json_files = common::find_json_files(&dir);
    assert!(!json_files.is_empty(), "No JSON files found in {dir:?}");

    for path in &json_files {
        let original = common::load_json(path);
        let ep = jsonization::event_payload_from_jsonable(&original)
            .unwrap_or_else(|e| panic!("Deserialization failed for {}: {e}", path.display()));

        let ep_class = aas_rs::types::class::Class::EventPayload(ep);
        let errors = verification::verify(&ep_class, true);
        assert!(
            errors.is_empty(),
            "Verification errors for {}: {:?}",
            path.display(),
            errors
        );

        let first_json = jsonization::to_jsonable(&ep_class);

        // Deserialize again from the serialized output (which includes modelType).
        let ep2 = jsonization::event_payload_from_jsonable(&first_json)
            .unwrap_or_else(|e| {
                panic!("Re-deserialization failed for {}: {e}", path.display())
            });
        let second_json =
            jsonization::to_jsonable(&aas_rs::types::class::Class::EventPayload(ep2));

        assert_eq!(
            first_json, second_json,
            "JSON round-trip mismatch for {}",
            path.display()
        );
    }
}

/// Invalid test for self-contained EventPayload.
fn self_contained_event_payload_invalid() {
    let invalid_root = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Unexpected")
        .join("Invalid");

    for cause_dir in common::find_immediate_subdirectories(&invalid_root) {
        let cls_dir = cause_dir.join("EventPayload");
        if !cls_dir.exists() {
            continue;
        }

        let json_files = common::find_json_files(&cls_dir);
        for path in &json_files {
            let original = common::load_json(path);
            let ep = jsonization::event_payload_from_jsonable(&original).unwrap_or_else(|e| {
                panic!(
                    "Expected deserialization to succeed for {}: {e}",
                    path.display()
                )
            });

            let ep_class = aas_rs::types::class::Class::EventPayload(ep);
            let errors = verification::verify(&ep_class, true);
            let got = format_verification_errors(&errors);

            let golden_path = std::path::PathBuf::from(format!("{}.errors", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got, expected,
                "Verification errors mismatch for {}",
                path.display()
            );
        }
    }
}

/// Unserializable test for self-contained EventPayload.
fn self_contained_event_payload_unserializable() {
    let unserializable_root = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
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

        let cls_dir = cause_dir.join("EventPayload");
        if !cls_dir.exists() {
            continue;
        }

        let json_files = common::find_json_files(&cls_dir);
        for path in &json_files {
            let original = common::load_json(path);
            let error = match jsonization::event_payload_from_jsonable(&original) {
                Err(e) => e,
                Ok(_) => panic!("Expected deserialization to fail for {}", path.display()),
            };

            let got = format_deserialization_error(&error);

            let golden_path = std::path::PathBuf::from(format!("{}.error", path.display()));
            let expected = common::load_golden_text(&golden_path);
            assert_eq!(
                got, expected,
                "Deserialization error mismatch for {}",
                path.display()
            );
        }
    }
}

#[test]
fn test_environment_self_contained() {
    self_contained_environment_round_trip();
    self_contained_environment_invalid();
    self_contained_environment_unserializable();
}

#[test]
fn test_event_payload_self_contained() {
    self_contained_event_payload_round_trip();
    self_contained_event_payload_invalid();
    self_contained_event_payload_unserializable();
}
