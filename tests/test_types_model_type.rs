//! Tests for `model_type()` methods and stringification.
//! Mirrors aas-core3.0-typescript/test/types.modelType.spec.ts

mod common;

use aas_rs::stringification::{model_type_from_str, model_type_to_str, must_model_type_to_str};
use aas_rs::types::class::Class;
use aas_rs::ModelType;

/// Helper: load the minimal example of `type_name`, find the first instance matching
/// `condition`, and return it as a `Class`.
macro_rules! load_minimal {
    ($type_name:expr, $is_fn:ident) => {{
        common::load_minimal_of($type_name, |c| c.$is_fn())
    }};
}

#[test]
fn test_model_type_of_extension() {
    let instance = load_minimal!("Extension", is_extension);
    assert_eq!(instance.model_type(), ModelType::Extension);
}

#[test]
fn test_model_type_from_str_extension() {
    assert_eq!(model_type_from_str("Extension"), Some(ModelType::Extension));
}

#[test]
fn test_model_type_to_str_extension() {
    assert_eq!(must_model_type_to_str(ModelType::Extension), "Extension");
}

#[test]
fn test_model_type_of_administrative_information() {
    let instance = load_minimal!("AdministrativeInformation", is_administrative_information);
    assert_eq!(instance.model_type(), ModelType::AdministrativeInformation);
}

#[test]
fn test_model_type_from_str_administrative_information() {
    assert_eq!(
        model_type_from_str("AdministrativeInformation"),
        Some(ModelType::AdministrativeInformation)
    );
}

#[test]
fn test_model_type_to_str_administrative_information() {
    assert_eq!(
        must_model_type_to_str(ModelType::AdministrativeInformation),
        "AdministrativeInformation"
    );
}

#[test]
fn test_model_type_of_qualifier() {
    let instance = load_minimal!("Qualifier", is_qualifier);
    assert_eq!(instance.model_type(), ModelType::Qualifier);
}

#[test]
fn test_model_type_from_str_qualifier() {
    assert_eq!(model_type_from_str("Qualifier"), Some(ModelType::Qualifier));
}

#[test]
fn test_model_type_to_str_qualifier() {
    assert_eq!(must_model_type_to_str(ModelType::Qualifier), "Qualifier");
}

#[test]
fn test_model_type_of_asset_administration_shell() {
    let instance = load_minimal!("AssetAdministrationShell", is_asset_administration_shell);
    assert_eq!(instance.model_type(), ModelType::AssetAdministrationShell);
}

#[test]
fn test_model_type_from_str_asset_administration_shell() {
    assert_eq!(
        model_type_from_str("AssetAdministrationShell"),
        Some(ModelType::AssetAdministrationShell)
    );
}

#[test]
fn test_model_type_to_str_asset_administration_shell() {
    assert_eq!(
        must_model_type_to_str(ModelType::AssetAdministrationShell),
        "AssetAdministrationShell"
    );
}

#[test]
fn test_model_type_of_asset_information() {
    let instance = load_minimal!("AssetInformation", is_asset_information);
    assert_eq!(instance.model_type(), ModelType::AssetInformation);
}

#[test]
fn test_model_type_from_str_asset_information() {
    assert_eq!(
        model_type_from_str("AssetInformation"),
        Some(ModelType::AssetInformation)
    );
}

#[test]
fn test_model_type_to_str_asset_information() {
    assert_eq!(
        must_model_type_to_str(ModelType::AssetInformation),
        "AssetInformation"
    );
}

#[test]
fn test_model_type_of_resource() {
    let instance = load_minimal!("Resource", is_resource);
    assert_eq!(instance.model_type(), ModelType::Resource);
}

#[test]
fn test_model_type_from_str_resource() {
    assert_eq!(model_type_from_str("Resource"), Some(ModelType::Resource));
}

#[test]
fn test_model_type_to_str_resource() {
    assert_eq!(must_model_type_to_str(ModelType::Resource), "Resource");
}

#[test]
fn test_model_type_of_specific_asset_id() {
    let instance = load_minimal!("SpecificAssetId", is_specific_asset_id);
    assert_eq!(instance.model_type(), ModelType::SpecificAssetId);
}

#[test]
fn test_model_type_from_str_specific_asset_id() {
    assert_eq!(
        model_type_from_str("SpecificAssetId"),
        Some(ModelType::SpecificAssetId)
    );
}

#[test]
fn test_model_type_to_str_specific_asset_id() {
    assert_eq!(
        must_model_type_to_str(ModelType::SpecificAssetId),
        "SpecificAssetId"
    );
}

#[test]
fn test_model_type_of_submodel() {
    let instance = load_minimal!("Submodel", is_submodel);
    assert_eq!(instance.model_type(), ModelType::Submodel);
}

#[test]
fn test_model_type_from_str_submodel() {
    assert_eq!(model_type_from_str("Submodel"), Some(ModelType::Submodel));
}

#[test]
fn test_model_type_to_str_submodel() {
    assert_eq!(must_model_type_to_str(ModelType::Submodel), "Submodel");
}

#[test]
fn test_model_type_of_relationship_element() {
    let instance = load_minimal!("RelationshipElement", is_relationship_element);
    assert_eq!(instance.model_type(), ModelType::RelationshipElement);
}

#[test]
fn test_model_type_from_str_relationship_element() {
    assert_eq!(
        model_type_from_str("RelationshipElement"),
        Some(ModelType::RelationshipElement)
    );
}

#[test]
fn test_model_type_to_str_relationship_element() {
    assert_eq!(
        must_model_type_to_str(ModelType::RelationshipElement),
        "RelationshipElement"
    );
}

#[test]
fn test_model_type_of_submodel_element_list() {
    let instance = load_minimal!("SubmodelElementList", is_submodel_element_list);
    assert_eq!(instance.model_type(), ModelType::SubmodelElementList);
}

#[test]
fn test_model_type_from_str_submodel_element_list() {
    assert_eq!(
        model_type_from_str("SubmodelElementList"),
        Some(ModelType::SubmodelElementList)
    );
}

#[test]
fn test_model_type_to_str_submodel_element_list() {
    assert_eq!(
        must_model_type_to_str(ModelType::SubmodelElementList),
        "SubmodelElementList"
    );
}

#[test]
fn test_model_type_of_submodel_element_collection() {
    let instance = load_minimal!("SubmodelElementCollection", is_submodel_element_collection);
    assert_eq!(instance.model_type(), ModelType::SubmodelElementCollection);
}

#[test]
fn test_model_type_from_str_submodel_element_collection() {
    assert_eq!(
        model_type_from_str("SubmodelElementCollection"),
        Some(ModelType::SubmodelElementCollection)
    );
}

#[test]
fn test_model_type_to_str_submodel_element_collection() {
    assert_eq!(
        must_model_type_to_str(ModelType::SubmodelElementCollection),
        "SubmodelElementCollection"
    );
}

#[test]
fn test_model_type_of_property() {
    let instance = load_minimal!("Property", is_property);
    assert_eq!(instance.model_type(), ModelType::Property);
}

#[test]
fn test_model_type_from_str_property() {
    assert_eq!(model_type_from_str("Property"), Some(ModelType::Property));
}

#[test]
fn test_model_type_to_str_property() {
    assert_eq!(must_model_type_to_str(ModelType::Property), "Property");
}

#[test]
fn test_model_type_of_multi_language_property() {
    let instance = load_minimal!("MultiLanguageProperty", is_multi_language_property);
    assert_eq!(instance.model_type(), ModelType::MultiLanguageProperty);
}

#[test]
fn test_model_type_from_str_multi_language_property() {
    assert_eq!(
        model_type_from_str("MultiLanguageProperty"),
        Some(ModelType::MultiLanguageProperty)
    );
}

#[test]
fn test_model_type_to_str_multi_language_property() {
    assert_eq!(
        must_model_type_to_str(ModelType::MultiLanguageProperty),
        "MultiLanguageProperty"
    );
}

#[test]
fn test_model_type_of_range() {
    let instance = load_minimal!("Range", is_range);
    assert_eq!(instance.model_type(), ModelType::Range);
}

#[test]
fn test_model_type_from_str_range() {
    assert_eq!(model_type_from_str("Range"), Some(ModelType::Range));
}

#[test]
fn test_model_type_to_str_range() {
    assert_eq!(must_model_type_to_str(ModelType::Range), "Range");
}

#[test]
fn test_model_type_of_reference_element() {
    let instance = load_minimal!("ReferenceElement", is_reference_element);
    assert_eq!(instance.model_type(), ModelType::ReferenceElement);
}

#[test]
fn test_model_type_from_str_reference_element() {
    assert_eq!(
        model_type_from_str("ReferenceElement"),
        Some(ModelType::ReferenceElement)
    );
}

#[test]
fn test_model_type_to_str_reference_element() {
    assert_eq!(
        must_model_type_to_str(ModelType::ReferenceElement),
        "ReferenceElement"
    );
}

#[test]
fn test_model_type_of_blob() {
    let instance = load_minimal!("Blob", is_blob);
    assert_eq!(instance.model_type(), ModelType::Blob);
}

#[test]
fn test_model_type_from_str_blob() {
    assert_eq!(model_type_from_str("Blob"), Some(ModelType::Blob));
}

#[test]
fn test_model_type_to_str_blob() {
    assert_eq!(must_model_type_to_str(ModelType::Blob), "Blob");
}

#[test]
fn test_model_type_of_file() {
    let instance = load_minimal!("File", is_file);
    assert_eq!(instance.model_type(), ModelType::File);
}

#[test]
fn test_model_type_from_str_file() {
    assert_eq!(model_type_from_str("File"), Some(ModelType::File));
}

#[test]
fn test_model_type_to_str_file() {
    assert_eq!(must_model_type_to_str(ModelType::File), "File");
}

#[test]
fn test_model_type_of_annotated_relationship_element() {
    let instance = load_minimal!(
        "AnnotatedRelationshipElement",
        is_annotated_relationship_element
    );
    assert_eq!(
        instance.model_type(),
        ModelType::AnnotatedRelationshipElement
    );
}

#[test]
fn test_model_type_from_str_annotated_relationship_element() {
    assert_eq!(
        model_type_from_str("AnnotatedRelationshipElement"),
        Some(ModelType::AnnotatedRelationshipElement)
    );
}

#[test]
fn test_model_type_to_str_annotated_relationship_element() {
    assert_eq!(
        must_model_type_to_str(ModelType::AnnotatedRelationshipElement),
        "AnnotatedRelationshipElement"
    );
}

#[test]
fn test_model_type_of_entity() {
    let instance = load_minimal!("Entity", is_entity);
    assert_eq!(instance.model_type(), ModelType::Entity);
}

#[test]
fn test_model_type_from_str_entity() {
    assert_eq!(model_type_from_str("Entity"), Some(ModelType::Entity));
}

#[test]
fn test_model_type_to_str_entity() {
    assert_eq!(must_model_type_to_str(ModelType::Entity), "Entity");
}

#[test]
fn test_model_type_of_event_payload() {
    let path = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("EventPayload")
        .join("minimal.json");
    let v = common::load_json(&path);
    let ep = aas_rs::jsonization::event_payload_from_jsonable(&v)
        .expect("EventPayload deserialization should succeed");
    let cls = Class::EventPayload(ep);
    assert_eq!(cls.model_type(), ModelType::EventPayload);
}

#[test]
fn test_model_type_from_str_event_payload() {
    assert_eq!(
        model_type_from_str("EventPayload"),
        Some(ModelType::EventPayload)
    );
}

#[test]
fn test_model_type_to_str_event_payload() {
    assert_eq!(
        must_model_type_to_str(ModelType::EventPayload),
        "EventPayload"
    );
}

#[test]
fn test_model_type_of_basic_event_element() {
    let instance = load_minimal!("BasicEventElement", is_basic_event_element);
    assert_eq!(instance.model_type(), ModelType::BasicEventElement);
}

#[test]
fn test_model_type_from_str_basic_event_element() {
    assert_eq!(
        model_type_from_str("BasicEventElement"),
        Some(ModelType::BasicEventElement)
    );
}

#[test]
fn test_model_type_to_str_basic_event_element() {
    assert_eq!(
        must_model_type_to_str(ModelType::BasicEventElement),
        "BasicEventElement"
    );
}

#[test]
fn test_model_type_of_operation() {
    let instance = load_minimal!("Operation", is_operation);
    assert_eq!(instance.model_type(), ModelType::Operation);
}

#[test]
fn test_model_type_from_str_operation() {
    assert_eq!(model_type_from_str("Operation"), Some(ModelType::Operation));
}

#[test]
fn test_model_type_to_str_operation() {
    assert_eq!(must_model_type_to_str(ModelType::Operation), "Operation");
}

#[test]
fn test_model_type_of_operation_variable() {
    let instance = load_minimal!("OperationVariable", is_operation_variable);
    assert_eq!(instance.model_type(), ModelType::OperationVariable);
}

#[test]
fn test_model_type_from_str_operation_variable() {
    assert_eq!(
        model_type_from_str("OperationVariable"),
        Some(ModelType::OperationVariable)
    );
}

#[test]
fn test_model_type_to_str_operation_variable() {
    assert_eq!(
        must_model_type_to_str(ModelType::OperationVariable),
        "OperationVariable"
    );
}

#[test]
fn test_model_type_of_capability() {
    let instance = load_minimal!("Capability", is_capability);
    assert_eq!(instance.model_type(), ModelType::Capability);
}

#[test]
fn test_model_type_from_str_capability() {
    assert_eq!(
        model_type_from_str("Capability"),
        Some(ModelType::Capability)
    );
}

#[test]
fn test_model_type_to_str_capability() {
    assert_eq!(must_model_type_to_str(ModelType::Capability), "Capability");
}

#[test]
fn test_model_type_of_concept_description() {
    let instance = load_minimal!("ConceptDescription", is_concept_description);
    assert_eq!(instance.model_type(), ModelType::ConceptDescription);
}

#[test]
fn test_model_type_from_str_concept_description() {
    assert_eq!(
        model_type_from_str("ConceptDescription"),
        Some(ModelType::ConceptDescription)
    );
}

#[test]
fn test_model_type_to_str_concept_description() {
    assert_eq!(
        must_model_type_to_str(ModelType::ConceptDescription),
        "ConceptDescription"
    );
}

#[test]
fn test_model_type_of_reference() {
    let instance = load_minimal!("Reference", is_reference);
    assert_eq!(instance.model_type(), ModelType::Reference);
}

#[test]
fn test_model_type_from_str_reference() {
    assert_eq!(model_type_from_str("Reference"), Some(ModelType::Reference));
}

#[test]
fn test_model_type_to_str_reference() {
    assert_eq!(must_model_type_to_str(ModelType::Reference), "Reference");
}

#[test]
fn test_model_type_of_key() {
    let instance = load_minimal!("Key", is_key);
    assert_eq!(instance.model_type(), ModelType::Key);
}

#[test]
fn test_model_type_from_str_key() {
    assert_eq!(model_type_from_str("Key"), Some(ModelType::Key));
}

#[test]
fn test_model_type_to_str_key() {
    assert_eq!(must_model_type_to_str(ModelType::Key), "Key");
}

#[test]
fn test_model_type_of_lang_string_name_type() {
    let instance = load_minimal!("LangStringNameType", is_lang_string_name_type);
    assert_eq!(instance.model_type(), ModelType::LangStringNameType);
}

#[test]
fn test_model_type_from_str_lang_string_name_type() {
    assert_eq!(
        model_type_from_str("LangStringNameType"),
        Some(ModelType::LangStringNameType)
    );
}

#[test]
fn test_model_type_to_str_lang_string_name_type() {
    assert_eq!(
        must_model_type_to_str(ModelType::LangStringNameType),
        "LangStringNameType"
    );
}

#[test]
fn test_model_type_of_lang_string_text_type() {
    let instance = load_minimal!("LangStringTextType", is_lang_string_text_type);
    assert_eq!(instance.model_type(), ModelType::LangStringTextType);
}

#[test]
fn test_model_type_from_str_lang_string_text_type() {
    assert_eq!(
        model_type_from_str("LangStringTextType"),
        Some(ModelType::LangStringTextType)
    );
}

#[test]
fn test_model_type_to_str_lang_string_text_type() {
    assert_eq!(
        must_model_type_to_str(ModelType::LangStringTextType),
        "LangStringTextType"
    );
}

#[test]
fn test_model_type_of_environment() {
    let path = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("Environment")
        .join("minimal.json");
    let env = Class::Environment(common::load_environment(&path));
    assert_eq!(env.model_type(), ModelType::Environment);
}

#[test]
fn test_model_type_from_str_environment() {
    assert_eq!(
        model_type_from_str("Environment"),
        Some(ModelType::Environment)
    );
}

#[test]
fn test_model_type_to_str_environment() {
    assert_eq!(
        must_model_type_to_str(ModelType::Environment),
        "Environment"
    );
}

#[test]
fn test_model_type_of_embedded_data_specification() {
    let instance = load_minimal!("EmbeddedDataSpecification", is_embedded_data_specification);
    assert_eq!(instance.model_type(), ModelType::EmbeddedDataSpecification);
}

#[test]
fn test_model_type_from_str_embedded_data_specification() {
    assert_eq!(
        model_type_from_str("EmbeddedDataSpecification"),
        Some(ModelType::EmbeddedDataSpecification)
    );
}

#[test]
fn test_model_type_to_str_embedded_data_specification() {
    assert_eq!(
        must_model_type_to_str(ModelType::EmbeddedDataSpecification),
        "EmbeddedDataSpecification"
    );
}

#[test]
fn test_model_type_of_level_type() {
    let instance = load_minimal!("LevelType", is_level_type);
    assert_eq!(instance.model_type(), ModelType::LevelType);
}

#[test]
fn test_model_type_from_str_level_type() {
    assert_eq!(model_type_from_str("LevelType"), Some(ModelType::LevelType));
}

#[test]
fn test_model_type_to_str_level_type() {
    assert_eq!(must_model_type_to_str(ModelType::LevelType), "LevelType");
}

#[test]
fn test_model_type_of_value_reference_pair() {
    let instance = load_minimal!("ValueReferencePair", is_value_reference_pair);
    assert_eq!(instance.model_type(), ModelType::ValueReferencePair);
}

#[test]
fn test_model_type_from_str_value_reference_pair() {
    assert_eq!(
        model_type_from_str("ValueReferencePair"),
        Some(ModelType::ValueReferencePair)
    );
}

#[test]
fn test_model_type_to_str_value_reference_pair() {
    assert_eq!(
        must_model_type_to_str(ModelType::ValueReferencePair),
        "ValueReferencePair"
    );
}

#[test]
fn test_model_type_of_value_list() {
    let instance = load_minimal!("ValueList", is_value_list);
    assert_eq!(instance.model_type(), ModelType::ValueList);
}

#[test]
fn test_model_type_from_str_value_list() {
    assert_eq!(model_type_from_str("ValueList"), Some(ModelType::ValueList));
}

#[test]
fn test_model_type_to_str_value_list() {
    assert_eq!(must_model_type_to_str(ModelType::ValueList), "ValueList");
}

#[test]
fn test_model_type_of_lang_string_preferred_name_type_iec61360() {
    let instance = load_minimal!(
        "LangStringPreferredNameTypeIec61360",
        is_lang_string_preferred_name_type_iec61360
    );
    assert_eq!(
        instance.model_type(),
        ModelType::LangStringPreferredNameTypeIec61360
    );
}

#[test]
fn test_model_type_from_str_lang_string_preferred_name_type_iec61360() {
    assert_eq!(
        model_type_from_str("LangStringPreferredNameTypeIec61360"),
        Some(ModelType::LangStringPreferredNameTypeIec61360)
    );
}

#[test]
fn test_model_type_to_str_lang_string_preferred_name_type_iec61360() {
    assert_eq!(
        must_model_type_to_str(ModelType::LangStringPreferredNameTypeIec61360),
        "LangStringPreferredNameTypeIec61360"
    );
}

#[test]
fn test_model_type_of_lang_string_short_name_type_iec61360() {
    let instance = load_minimal!(
        "LangStringShortNameTypeIec61360",
        is_lang_string_short_name_type_iec61360
    );
    assert_eq!(
        instance.model_type(),
        ModelType::LangStringShortNameTypeIec61360
    );
}

#[test]
fn test_model_type_from_str_lang_string_short_name_type_iec61360() {
    assert_eq!(
        model_type_from_str("LangStringShortNameTypeIec61360"),
        Some(ModelType::LangStringShortNameTypeIec61360)
    );
}

#[test]
fn test_model_type_to_str_lang_string_short_name_type_iec61360() {
    assert_eq!(
        must_model_type_to_str(ModelType::LangStringShortNameTypeIec61360),
        "LangStringShortNameTypeIec61360"
    );
}

#[test]
fn test_model_type_of_lang_string_definition_type_iec61360() {
    let instance = load_minimal!(
        "LangStringDefinitionTypeIec61360",
        is_lang_string_definition_type_iec61360
    );
    assert_eq!(
        instance.model_type(),
        ModelType::LangStringDefinitionTypeIec61360
    );
}

#[test]
fn test_model_type_from_str_lang_string_definition_type_iec61360() {
    assert_eq!(
        model_type_from_str("LangStringDefinitionTypeIec61360"),
        Some(ModelType::LangStringDefinitionTypeIec61360)
    );
}

#[test]
fn test_model_type_to_str_lang_string_definition_type_iec61360() {
    assert_eq!(
        must_model_type_to_str(ModelType::LangStringDefinitionTypeIec61360),
        "LangStringDefinitionTypeIec61360"
    );
}

#[test]
fn test_model_type_of_data_specification_iec61360() {
    let instance = load_minimal!("DataSpecificationIec61360", is_data_specification_iec61360);
    assert_eq!(instance.model_type(), ModelType::DataSpecificationIec61360);
}

#[test]
fn test_model_type_from_str_data_specification_iec61360() {
    assert_eq!(
        model_type_from_str("DataSpecificationIec61360"),
        Some(ModelType::DataSpecificationIec61360)
    );
}

#[test]
fn test_model_type_to_str_data_specification_iec61360() {
    assert_eq!(
        must_model_type_to_str(ModelType::DataSpecificationIec61360),
        "DataSpecificationIec61360"
    );
}

#[test]
fn test_model_type_from_invalid_string() {
    assert_eq!(
        model_type_from_str("This is definitely not a valid model type."),
        None
    );
}

#[test]
fn test_model_type_to_str_invalid() {
    // model_type_to_str returns None for unknown values; since ModelType is
    // an exhaustive enum in Rust, all valid variants should succeed.
    // We just verify known variants round-trip.
    let result = model_type_to_str(ModelType::Extension);
    assert!(result.is_some());
}
