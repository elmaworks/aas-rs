//! Tests for JSON round-trip of concrete classes outside a container.
//! Mirrors aas-core3.0-typescript/test/jsonization.concreteClassesOutsideContainer.spec.ts

mod common;

use aas_rs::jsonization;

/// Load the maximal instance of `type_name` from ContainedInEnvironment/Expected,
/// serialize it to JSON, deserialize using class_from_jsonable, serialize again, and compare.
fn outside_container_round_trip(type_name: &str) {
    let expected_model_type = aas_rs::stringification::model_type_from_str(type_name)
        .unwrap_or_else(|| panic!("Unknown type name: {type_name}"));

    let instance = common::load_maximal_of(type_name, |c| c.model_type() == expected_model_type);

    // Serialize to JSON (with modelType).
    let first_json = jsonization::to_jsonable(&instance);

    // Deserialize from JSON using class_from_jsonable.
    let another_instance = jsonization::class_from_jsonable(&first_json).unwrap_or_else(|e| {
        panic!("Failed to deserialize {type_name} from JSON: {e}\nJSON: {first_json}")
    });

    // Serialize again.
    let second_json = jsonization::to_jsonable(&another_instance);

    assert_eq!(
        first_json, second_json,
        "JSON round-trip mismatch for {type_name}"
    );
}

macro_rules! outside_container_test {
    ($test_name:ident, $type_name:expr) => {
        #[test]
        fn $test_name() {
            outside_container_round_trip($type_name);
        }
    };
}

outside_container_test!(test_extension_outside_container, "Extension");
outside_container_test!(
    test_administrative_information_outside_container,
    "AdministrativeInformation"
);
outside_container_test!(test_qualifier_outside_container, "Qualifier");
outside_container_test!(
    test_asset_administration_shell_outside_container,
    "AssetAdministrationShell"
);
outside_container_test!(test_asset_information_outside_container, "AssetInformation");
outside_container_test!(test_resource_outside_container, "Resource");
outside_container_test!(test_specific_asset_id_outside_container, "SpecificAssetId");
outside_container_test!(test_submodel_outside_container, "Submodel");
outside_container_test!(
    test_relationship_element_outside_container,
    "RelationshipElement"
);
outside_container_test!(
    test_submodel_element_list_outside_container,
    "SubmodelElementList"
);
outside_container_test!(
    test_submodel_element_collection_outside_container,
    "SubmodelElementCollection"
);
outside_container_test!(test_property_outside_container, "Property");
outside_container_test!(
    test_multi_language_property_outside_container,
    "MultiLanguageProperty"
);
outside_container_test!(test_range_outside_container, "Range");
outside_container_test!(test_reference_element_outside_container, "ReferenceElement");
outside_container_test!(test_blob_outside_container, "Blob");
outside_container_test!(test_file_outside_container, "File");
outside_container_test!(
    test_annotated_relationship_element_outside_container,
    "AnnotatedRelationshipElement"
);
outside_container_test!(test_entity_outside_container, "Entity");
outside_container_test!(
    test_basic_event_element_outside_container,
    "BasicEventElement"
);
outside_container_test!(test_operation_outside_container, "Operation");
outside_container_test!(
    test_operation_variable_outside_container,
    "OperationVariable"
);
outside_container_test!(test_capability_outside_container, "Capability");
outside_container_test!(
    test_concept_description_outside_container,
    "ConceptDescription"
);
outside_container_test!(test_reference_outside_container, "Reference");
outside_container_test!(test_key_outside_container, "Key");
outside_container_test!(
    test_lang_string_name_type_outside_container,
    "LangStringNameType"
);
outside_container_test!(
    test_lang_string_text_type_outside_container,
    "LangStringTextType"
);
outside_container_test!(
    test_embedded_data_specification_outside_container,
    "EmbeddedDataSpecification"
);
outside_container_test!(test_level_type_outside_container, "LevelType");
outside_container_test!(
    test_value_reference_pair_outside_container,
    "ValueReferencePair"
);
outside_container_test!(test_value_list_outside_container, "ValueList");
outside_container_test!(
    test_lang_string_preferred_name_type_iec61360_outside_container,
    "LangStringPreferredNameTypeIec61360"
);
outside_container_test!(
    test_lang_string_short_name_type_iec61360_outside_container,
    "LangStringShortNameTypeIec61360"
);
outside_container_test!(
    test_lang_string_definition_type_iec61360_outside_container,
    "LangStringDefinitionTypeIec61360"
);
outside_container_test!(
    test_data_specification_iec61360_outside_container,
    "DataSpecificationIec61360"
);

#[test]
fn test_environment_outside_container() {
    let instance = common::load_maximal_environment();
    let first_json = jsonization::to_jsonable(&instance);
    let env = jsonization::environment_from_jsonable_value(&first_json)
        .unwrap_or_else(|e| panic!("Failed to deserialize Environment: {e}"));
    let second_json =
        jsonization::to_jsonable(&aas_rs::types::class::Class::Environment(env));
    assert_eq!(first_json, second_json);
}

#[test]
fn test_event_payload_outside_container() {
    let instance = common::load_maximal_event_payload();
    let first_json = jsonization::to_jsonable(&instance);
    let ep = jsonization::event_payload_from_jsonable(&first_json)
        .unwrap_or_else(|e| panic!("Failed to deserialize EventPayload: {e}"));
    let second_json =
        jsonization::to_jsonable(&aas_rs::types::class::Class::EventPayload(ep));
    assert_eq!(first_json, second_json);
}
