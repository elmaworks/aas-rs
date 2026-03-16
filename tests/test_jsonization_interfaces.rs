//! Tests for JSON de/serialization of interfaces.
//! Mirrors aas-core3.0-typescript/test/jsonization.interfaces.spec.ts
//!
//! In Rust, all interface-based deserialization uses `class_from_jsonable`
//! which dispatches via the `modelType` field.

mod common;

use aas_rs::jsonization;

/// Load minimal instance of `type_name`, serialize, deserialize with class_from_jsonable,
/// serialize again, and compare.
fn interface_round_trip(type_name: &str) {
    let expected_model_type = aas_rs::stringification::model_type_from_str(type_name)
        .unwrap_or_else(|| panic!("Unknown type name: {type_name}"));

    let instance = common::load_minimal_of(type_name, |c| c.model_type() == expected_model_type);

    let first_json = jsonization::to_jsonable(&instance);

    let another_instance = jsonization::class_from_jsonable(&first_json)
        .unwrap_or_else(|e| panic!("Failed to deserialize {type_name}: {e}\nJSON: {first_json}"));

    let second_json = jsonization::to_jsonable(&another_instance);

    assert_eq!(
        first_json, second_json,
        "Interface round-trip mismatch for {type_name}"
    );
}

macro_rules! iface_test {
    ($test_name:ident, $iface:expr, $type_name:expr) => {
        #[test]
        fn $test_name() {
            interface_round_trip($type_name);
        }
    };
}

// IHasSemantics
iface_test!(
    test_ihas_semantics_relationship_element,
    "IHasSemantics",
    "RelationshipElement"
);
iface_test!(
    test_ihas_semantics_annotated_relationship_element,
    "IHasSemantics",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_ihas_semantics_basic_event_element,
    "IHasSemantics",
    "BasicEventElement"
);
iface_test!(test_ihas_semantics_blob, "IHasSemantics", "Blob");
iface_test!(
    test_ihas_semantics_capability,
    "IHasSemantics",
    "Capability"
);
iface_test!(test_ihas_semantics_entity, "IHasSemantics", "Entity");
iface_test!(test_ihas_semantics_file, "IHasSemantics", "File");
iface_test!(
    test_ihas_semantics_multi_language_property,
    "IHasSemantics",
    "MultiLanguageProperty"
);
iface_test!(test_ihas_semantics_operation, "IHasSemantics", "Operation");
iface_test!(test_ihas_semantics_property, "IHasSemantics", "Property");
iface_test!(test_ihas_semantics_range, "IHasSemantics", "Range");
iface_test!(
    test_ihas_semantics_reference_element,
    "IHasSemantics",
    "ReferenceElement"
);
iface_test!(test_ihas_semantics_submodel, "IHasSemantics", "Submodel");
iface_test!(
    test_ihas_semantics_submodel_element_collection,
    "IHasSemantics",
    "SubmodelElementCollection"
);
iface_test!(
    test_ihas_semantics_submodel_element_list,
    "IHasSemantics",
    "SubmodelElementList"
);

// IHasExtensions
iface_test!(
    test_ihas_extensions_relationship_element,
    "IHasExtensions",
    "RelationshipElement"
);
iface_test!(
    test_ihas_extensions_annotated_relationship_element,
    "IHasExtensions",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_ihas_extensions_asset_administration_shell,
    "IHasExtensions",
    "AssetAdministrationShell"
);
iface_test!(
    test_ihas_extensions_basic_event_element,
    "IHasExtensions",
    "BasicEventElement"
);
iface_test!(test_ihas_extensions_blob, "IHasExtensions", "Blob");
iface_test!(
    test_ihas_extensions_capability,
    "IHasExtensions",
    "Capability"
);
iface_test!(
    test_ihas_extensions_concept_description,
    "IHasExtensions",
    "ConceptDescription"
);
iface_test!(test_ihas_extensions_entity, "IHasExtensions", "Entity");
iface_test!(test_ihas_extensions_file, "IHasExtensions", "File");
iface_test!(
    test_ihas_extensions_multi_language_property,
    "IHasExtensions",
    "MultiLanguageProperty"
);
iface_test!(
    test_ihas_extensions_operation,
    "IHasExtensions",
    "Operation"
);
iface_test!(test_ihas_extensions_property, "IHasExtensions", "Property");
iface_test!(test_ihas_extensions_range, "IHasExtensions", "Range");
iface_test!(
    test_ihas_extensions_reference_element,
    "IHasExtensions",
    "ReferenceElement"
);
iface_test!(test_ihas_extensions_submodel, "IHasExtensions", "Submodel");
iface_test!(
    test_ihas_extensions_submodel_element_collection,
    "IHasExtensions",
    "SubmodelElementCollection"
);
iface_test!(
    test_ihas_extensions_submodel_element_list,
    "IHasExtensions",
    "SubmodelElementList"
);

// IReferable
iface_test!(
    test_ireferable_relationship_element,
    "IReferable",
    "RelationshipElement"
);
iface_test!(
    test_ireferable_annotated_relationship_element,
    "IReferable",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_ireferable_asset_administration_shell,
    "IReferable",
    "AssetAdministrationShell"
);
iface_test!(
    test_ireferable_basic_event_element,
    "IReferable",
    "BasicEventElement"
);
iface_test!(test_ireferable_blob, "IReferable", "Blob");
iface_test!(test_ireferable_capability, "IReferable", "Capability");
iface_test!(
    test_ireferable_concept_description,
    "IReferable",
    "ConceptDescription"
);
iface_test!(test_ireferable_entity, "IReferable", "Entity");
iface_test!(test_ireferable_file, "IReferable", "File");
iface_test!(
    test_ireferable_multi_language_property,
    "IReferable",
    "MultiLanguageProperty"
);
iface_test!(test_ireferable_operation, "IReferable", "Operation");
iface_test!(test_ireferable_property, "IReferable", "Property");
iface_test!(test_ireferable_range, "IReferable", "Range");
iface_test!(
    test_ireferable_reference_element,
    "IReferable",
    "ReferenceElement"
);
iface_test!(test_ireferable_submodel, "IReferable", "Submodel");
iface_test!(
    test_ireferable_submodel_element_collection,
    "IReferable",
    "SubmodelElementCollection"
);
iface_test!(
    test_ireferable_submodel_element_list,
    "IReferable",
    "SubmodelElementList"
);

// IIdentifiable
iface_test!(
    test_iidentifiable_asset_administration_shell,
    "IIdentifiable",
    "AssetAdministrationShell"
);
iface_test!(
    test_iidentifiable_concept_description,
    "IIdentifiable",
    "ConceptDescription"
);
iface_test!(test_iidentifiable_submodel, "IIdentifiable", "Submodel");

// IHasKind
iface_test!(test_ihas_kind_submodel, "IHasKind", "Submodel");

// IHasDataSpecification
iface_test!(
    test_ihas_data_specification_annotated_relationship_element,
    "IHasDataSpecification",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_ihas_data_specification_asset_administration_shell,
    "IHasDataSpecification",
    "AssetAdministrationShell"
);
iface_test!(
    test_ihas_data_specification_basic_event_element,
    "IHasDataSpecification",
    "BasicEventElement"
);
iface_test!(
    test_ihas_data_specification_blob,
    "IHasDataSpecification",
    "Blob"
);
iface_test!(
    test_ihas_data_specification_capability,
    "IHasDataSpecification",
    "Capability"
);
iface_test!(
    test_ihas_data_specification_concept_description,
    "IHasDataSpecification",
    "ConceptDescription"
);
iface_test!(
    test_ihas_data_specification_entity,
    "IHasDataSpecification",
    "Entity"
);
iface_test!(
    test_ihas_data_specification_file,
    "IHasDataSpecification",
    "File"
);
iface_test!(
    test_ihas_data_specification_multi_language_property,
    "IHasDataSpecification",
    "MultiLanguageProperty"
);
iface_test!(
    test_ihas_data_specification_operation,
    "IHasDataSpecification",
    "Operation"
);
iface_test!(
    test_ihas_data_specification_property,
    "IHasDataSpecification",
    "Property"
);
iface_test!(
    test_ihas_data_specification_range,
    "IHasDataSpecification",
    "Range"
);
iface_test!(
    test_ihas_data_specification_reference_element,
    "IHasDataSpecification",
    "ReferenceElement"
);
iface_test!(
    test_ihas_data_specification_relationship_element,
    "IHasDataSpecification",
    "RelationshipElement"
);
iface_test!(
    test_ihas_data_specification_submodel,
    "IHasDataSpecification",
    "Submodel"
);
iface_test!(
    test_ihas_data_specification_submodel_element_collection,
    "IHasDataSpecification",
    "SubmodelElementCollection"
);
iface_test!(
    test_ihas_data_specification_submodel_element_list,
    "IHasDataSpecification",
    "SubmodelElementList"
);

// IQualifiable
iface_test!(
    test_iqualifiable_annotated_relationship_element,
    "IQualifiable",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_iqualifiable_basic_event_element,
    "IQualifiable",
    "BasicEventElement"
);
iface_test!(test_iqualifiable_blob, "IQualifiable", "Blob");
iface_test!(test_iqualifiable_capability, "IQualifiable", "Capability");
iface_test!(test_iqualifiable_entity, "IQualifiable", "Entity");
iface_test!(test_iqualifiable_file, "IQualifiable", "File");
iface_test!(
    test_iqualifiable_multi_language_property,
    "IQualifiable",
    "MultiLanguageProperty"
);
iface_test!(test_iqualifiable_operation, "IQualifiable", "Operation");
iface_test!(test_iqualifiable_property, "IQualifiable", "Property");
iface_test!(test_iqualifiable_range, "IQualifiable", "Range");
iface_test!(
    test_iqualifiable_reference_element,
    "IQualifiable",
    "ReferenceElement"
);
iface_test!(
    test_iqualifiable_relationship_element,
    "IQualifiable",
    "RelationshipElement"
);
iface_test!(test_iqualifiable_submodel, "IQualifiable", "Submodel");
iface_test!(
    test_iqualifiable_submodel_element_collection,
    "IQualifiable",
    "SubmodelElementCollection"
);
iface_test!(
    test_iqualifiable_submodel_element_list,
    "IQualifiable",
    "SubmodelElementList"
);

// ISubmodelElement
iface_test!(
    test_isubmodel_element_annotated_relationship_element,
    "ISubmodelElement",
    "AnnotatedRelationshipElement"
);
iface_test!(
    test_isubmodel_element_basic_event_element,
    "ISubmodelElement",
    "BasicEventElement"
);
iface_test!(test_isubmodel_element_blob, "ISubmodelElement", "Blob");
iface_test!(
    test_isubmodel_element_capability,
    "ISubmodelElement",
    "Capability"
);
iface_test!(test_isubmodel_element_entity, "ISubmodelElement", "Entity");
iface_test!(test_isubmodel_element_file, "ISubmodelElement", "File");
iface_test!(
    test_isubmodel_element_multi_language_property,
    "ISubmodelElement",
    "MultiLanguageProperty"
);
iface_test!(
    test_isubmodel_element_operation,
    "ISubmodelElement",
    "Operation"
);
iface_test!(
    test_isubmodel_element_property,
    "ISubmodelElement",
    "Property"
);
iface_test!(test_isubmodel_element_range, "ISubmodelElement", "Range");
iface_test!(
    test_isubmodel_element_reference_element,
    "ISubmodelElement",
    "ReferenceElement"
);
iface_test!(
    test_isubmodel_element_relationship_element,
    "ISubmodelElement",
    "RelationshipElement"
);
iface_test!(
    test_isubmodel_element_submodel_element_collection,
    "ISubmodelElement",
    "SubmodelElementCollection"
);
iface_test!(
    test_isubmodel_element_submodel_element_list,
    "ISubmodelElement",
    "SubmodelElementList"
);

// IRelationshipElement
iface_test!(
    test_irelationship_element_relationship_element,
    "IRelationshipElement",
    "RelationshipElement"
);
iface_test!(
    test_irelationship_element_annotated_relationship_element,
    "IRelationshipElement",
    "AnnotatedRelationshipElement"
);

// IDataElement
iface_test!(test_idata_element_blob, "IDataElement", "Blob");
iface_test!(test_idata_element_file, "IDataElement", "File");
iface_test!(
    test_idata_element_multi_language_property,
    "IDataElement",
    "MultiLanguageProperty"
);
iface_test!(test_idata_element_property, "IDataElement", "Property");
iface_test!(test_idata_element_range, "IDataElement", "Range");
iface_test!(
    test_idata_element_reference_element,
    "IDataElement",
    "ReferenceElement"
);

// IEventElement
iface_test!(
    test_ievent_element_basic_event_element,
    "IEventElement",
    "BasicEventElement"
);

// IDataSpecificationContent
iface_test!(
    test_idata_specification_content_data_specification_iec61360,
    "IDataSpecificationContent",
    "DataSpecificationIec61360"
);

// Deserialization fail: non-object input to class_from_jsonable
#[test]
fn test_class_deserialization_fail() {
    let jsonable = serde_json::Value::String("This is not a JSON object.".to_string());
    let err = jsonization::class_from_jsonable(&jsonable).unwrap_err();
    assert_eq!(err.to_string(), "Expected a JSON object, but got: string");
}
