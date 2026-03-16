//! Tests for `descend()` and pass-through visitor.
//! Mirrors aas-core3.0-typescript/test/types.descendAndPassThroughVisitor.spec.ts

mod common;

use aas_rs::types::class::Class;

fn compare_trace(instance: &Class, type_name: &str, example: &str) {
    let got = common::descend_trace(instance);
    let golden_path = common::test_data_dir()
        .join("descend")
        .join(type_name)
        .join(format!("{example}.trace"));
    common::assert_trace_eq(&got, &golden_path);
}

macro_rules! descend_test {
    ($test_name:ident, $type_name:expr, $is_fn:ident) => {
        #[test]
        fn $test_name() {
            let instance = common::load_maximal_of($type_name, |c| c.$is_fn());
            compare_trace(&instance, $type_name, "maximal.json");
        }
    };
}

// --- PassThroughVisitor helpers ---

struct TracingVisitor {
    log: Vec<String>,
}

impl TracingVisitor {
    fn new() -> Self {
        Self { log: Vec::new() }
    }

    fn visit(&mut self, instance: &Class) {
        self.log.push(common::trace_mark(instance));
        for child in instance.descend_once() {
            self.visit(&child);
        }
    }
}

/// Verifies that recursive `descend_once()` produces the same sequence as `descend()`.
fn expect_descend_and_visitor_same(instance: &Class) {
    let descend_result: Vec<String> =
        instance.descend().map(|d| common::trace_mark(&d)).collect();

    let mut visitor = TracingVisitor::new();
    for child in instance.descend_once() {
        visitor.visit(&child);
    }

    assert_eq!(descend_result, visitor.log);
}

macro_rules! pass_through_test {
    ($test_name:ident, $type_name:expr, $is_fn:ident) => {
        #[test]
        fn $test_name() {
            let instance = common::load_maximal_of($type_name, |c| c.$is_fn());
            expect_descend_and_visitor_same(&instance);
        }
    };
}

descend_test!(test_descend_extension, "Extension", is_extension);
descend_test!(
    test_descend_administrative_information,
    "AdministrativeInformation",
    is_administrative_information
);
descend_test!(test_descend_qualifier, "Qualifier", is_qualifier);
descend_test!(
    test_descend_asset_administration_shell,
    "AssetAdministrationShell",
    is_asset_administration_shell
);
descend_test!(
    test_descend_asset_information,
    "AssetInformation",
    is_asset_information
);
descend_test!(test_descend_resource, "Resource", is_resource);
descend_test!(
    test_descend_specific_asset_id,
    "SpecificAssetId",
    is_specific_asset_id
);
descend_test!(test_descend_submodel, "Submodel", is_submodel);
descend_test!(
    test_descend_relationship_element,
    "RelationshipElement",
    is_relationship_element
);
descend_test!(
    test_descend_submodel_element_list,
    "SubmodelElementList",
    is_submodel_element_list
);
descend_test!(
    test_descend_submodel_element_collection,
    "SubmodelElementCollection",
    is_submodel_element_collection
);
descend_test!(test_descend_property, "Property", is_property);
descend_test!(
    test_descend_multi_language_property,
    "MultiLanguageProperty",
    is_multi_language_property
);
descend_test!(test_descend_range, "Range", is_range);
descend_test!(
    test_descend_reference_element,
    "ReferenceElement",
    is_reference_element
);
descend_test!(test_descend_blob, "Blob", is_blob);
descend_test!(test_descend_file, "File", is_file);
descend_test!(
    test_descend_annotated_relationship_element,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element
);
descend_test!(test_descend_entity, "Entity", is_entity);
descend_test!(
    test_descend_basic_event_element,
    "BasicEventElement",
    is_basic_event_element
);
descend_test!(test_descend_operation, "Operation", is_operation);
descend_test!(
    test_descend_operation_variable,
    "OperationVariable",
    is_operation_variable
);
descend_test!(test_descend_capability, "Capability", is_capability);
descend_test!(
    test_descend_concept_description,
    "ConceptDescription",
    is_concept_description
);
descend_test!(test_descend_reference, "Reference", is_reference);
descend_test!(test_descend_key, "Key", is_key);
descend_test!(
    test_descend_lang_string_name_type,
    "LangStringNameType",
    is_lang_string_name_type
);
descend_test!(
    test_descend_lang_string_text_type,
    "LangStringTextType",
    is_lang_string_text_type
);
descend_test!(
    test_descend_embedded_data_specification,
    "EmbeddedDataSpecification",
    is_embedded_data_specification
);
descend_test!(test_descend_level_type, "LevelType", is_level_type);
descend_test!(
    test_descend_value_reference_pair,
    "ValueReferencePair",
    is_value_reference_pair
);
descend_test!(test_descend_value_list, "ValueList", is_value_list);
descend_test!(
    test_descend_lang_string_preferred_name_type_iec61360,
    "LangStringPreferredNameTypeIec61360",
    is_lang_string_preferred_name_type_iec61360
);
descend_test!(
    test_descend_lang_string_short_name_type_iec61360,
    "LangStringShortNameTypeIec61360",
    is_lang_string_short_name_type_iec61360
);
descend_test!(
    test_descend_lang_string_definition_type_iec61360,
    "LangStringDefinitionTypeIec61360",
    is_lang_string_definition_type_iec61360
);
descend_test!(
    test_descend_data_specification_iec61360,
    "DataSpecificationIec61360",
    is_data_specification_iec61360
);

#[test]
fn test_descend_event_payload() {
    let path = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("EventPayload")
        .join("maximal.json");
    let v = common::load_json(&path);
    let ep = aas_rs::jsonization::event_payload_from_jsonable(&v)
        .expect("EventPayload deserialization should succeed");
    let instance = Class::EventPayload(ep);
    let got = common::descend_trace(&instance);
    let golden_path = common::test_data_dir()
        .join("descend")
        .join("EventPayload")
        .join("maximal.json.trace");
    common::assert_trace_eq(&got, &golden_path);
}

#[test]
fn test_descend_environment() {
    let instance = common::load_maximal_environment();
    let got = common::descend_trace(&instance);
    let golden_path = common::test_data_dir()
        .join("descend")
        .join("Environment")
        .join("maximal.json.trace");
    common::assert_trace_eq(&got, &golden_path);
}

// --- PassThroughVisitor tests ---

pass_through_test!(test_pass_through_extension, "Extension", is_extension);
pass_through_test!(
    test_pass_through_administrative_information,
    "AdministrativeInformation",
    is_administrative_information
);
pass_through_test!(test_pass_through_qualifier, "Qualifier", is_qualifier);
pass_through_test!(
    test_pass_through_asset_administration_shell,
    "AssetAdministrationShell",
    is_asset_administration_shell
);
pass_through_test!(
    test_pass_through_asset_information,
    "AssetInformation",
    is_asset_information
);
pass_through_test!(test_pass_through_resource, "Resource", is_resource);
pass_through_test!(
    test_pass_through_specific_asset_id,
    "SpecificAssetId",
    is_specific_asset_id
);
pass_through_test!(test_pass_through_submodel, "Submodel", is_submodel);
pass_through_test!(
    test_pass_through_relationship_element,
    "RelationshipElement",
    is_relationship_element
);
pass_through_test!(
    test_pass_through_submodel_element_list,
    "SubmodelElementList",
    is_submodel_element_list
);
pass_through_test!(
    test_pass_through_submodel_element_collection,
    "SubmodelElementCollection",
    is_submodel_element_collection
);
pass_through_test!(test_pass_through_property, "Property", is_property);
pass_through_test!(
    test_pass_through_multi_language_property,
    "MultiLanguageProperty",
    is_multi_language_property
);
pass_through_test!(test_pass_through_range, "Range", is_range);
pass_through_test!(
    test_pass_through_reference_element,
    "ReferenceElement",
    is_reference_element
);
pass_through_test!(test_pass_through_blob, "Blob", is_blob);
pass_through_test!(test_pass_through_file, "File", is_file);
pass_through_test!(
    test_pass_through_annotated_relationship_element,
    "AnnotatedRelationshipElement",
    is_annotated_relationship_element
);
pass_through_test!(test_pass_through_entity, "Entity", is_entity);
pass_through_test!(
    test_pass_through_basic_event_element,
    "BasicEventElement",
    is_basic_event_element
);
pass_through_test!(test_pass_through_operation, "Operation", is_operation);
pass_through_test!(
    test_pass_through_operation_variable,
    "OperationVariable",
    is_operation_variable
);
pass_through_test!(test_pass_through_capability, "Capability", is_capability);
pass_through_test!(
    test_pass_through_concept_description,
    "ConceptDescription",
    is_concept_description
);
pass_through_test!(test_pass_through_reference, "Reference", is_reference);
pass_through_test!(test_pass_through_key, "Key", is_key);
pass_through_test!(
    test_pass_through_lang_string_name_type,
    "LangStringNameType",
    is_lang_string_name_type
);
pass_through_test!(
    test_pass_through_lang_string_text_type,
    "LangStringTextType",
    is_lang_string_text_type
);
pass_through_test!(
    test_pass_through_embedded_data_specification,
    "EmbeddedDataSpecification",
    is_embedded_data_specification
);
pass_through_test!(test_pass_through_level_type, "LevelType", is_level_type);
pass_through_test!(
    test_pass_through_value_reference_pair,
    "ValueReferencePair",
    is_value_reference_pair
);
pass_through_test!(test_pass_through_value_list, "ValueList", is_value_list);
pass_through_test!(
    test_pass_through_lang_string_preferred_name_type_iec61360,
    "LangStringPreferredNameTypeIec61360",
    is_lang_string_preferred_name_type_iec61360
);
pass_through_test!(
    test_pass_through_lang_string_short_name_type_iec61360,
    "LangStringShortNameTypeIec61360",
    is_lang_string_short_name_type_iec61360
);
pass_through_test!(
    test_pass_through_lang_string_definition_type_iec61360,
    "LangStringDefinitionTypeIec61360",
    is_lang_string_definition_type_iec61360
);
pass_through_test!(
    test_pass_through_data_specification_iec61360,
    "DataSpecificationIec61360",
    is_data_specification_iec61360
);

#[test]
fn test_pass_through_event_payload() {
    let path = common::test_data_dir()
        .join("Json")
        .join("SelfContained")
        .join("Expected")
        .join("EventPayload")
        .join("maximal.json");
    let v = common::load_json(&path);
    let ep = aas_rs::jsonization::event_payload_from_jsonable(&v)
        .expect("EventPayload deserialization should succeed");
    let instance = Class::EventPayload(ep);
    expect_descend_and_visitor_same(&instance);
}

#[test]
fn test_pass_through_environment() {
    let instance = common::load_maximal_environment();
    expect_descend_and_visitor_same(&instance);
}
