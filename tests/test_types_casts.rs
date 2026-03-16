//! Tests for `as_*` and `is_*` cast functions.
//! Mirrors aas-core3.0-typescript/test/types.casts.spec.ts

mod common;

/// Checks a single cast pair:
/// - if the method name matches `$is_self`, asserts true/Some
/// - otherwise asserts false/None
macro_rules! cast_check {
    ($inst:expr, $is_self:ident, $is_m:ident, $as_m:ident) => {
        if stringify!($is_self) == stringify!($is_m) {
            assert!(
                $inst.$is_m(),
                "Expected {} to be true",
                stringify!($is_m)
            );
            assert!(
                $inst.$as_m().is_some(),
                "Expected {} to return Some",
                stringify!($as_m)
            );
        } else {
            assert!(
                !$inst.$is_m(),
                "Expected {} to be false",
                stringify!($is_m)
            );
            assert!(
                $inst.$as_m().is_none(),
                "Expected {} to return None",
                stringify!($as_m)
            );
        }
    };
}

/// Generates a test that exhaustively checks all 38 concrete type cast pairs.
macro_rules! exhaustive_cast_test {
    ($test_name:ident, $load_expr:expr, $is_self:ident, $as_self:ident) => {
        #[test]
        fn $test_name() {
            let inst = $load_expr;
            cast_check!(inst, $is_self, is_extension, as_extension);
            cast_check!(
                inst,
                $is_self,
                is_administrative_information,
                as_administrative_information
            );
            cast_check!(inst, $is_self, is_qualifier, as_qualifier);
            cast_check!(
                inst,
                $is_self,
                is_asset_administration_shell,
                as_asset_administration_shell
            );
            cast_check!(inst, $is_self, is_asset_information, as_asset_information);
            cast_check!(inst, $is_self, is_resource, as_resource);
            cast_check!(inst, $is_self, is_specific_asset_id, as_specific_asset_id);
            cast_check!(inst, $is_self, is_submodel, as_submodel);
            cast_check!(
                inst,
                $is_self,
                is_relationship_element,
                as_relationship_element
            );
            cast_check!(
                inst,
                $is_self,
                is_submodel_element_list,
                as_submodel_element_list
            );
            cast_check!(
                inst,
                $is_self,
                is_submodel_element_collection,
                as_submodel_element_collection
            );
            cast_check!(inst, $is_self, is_property, as_property);
            cast_check!(
                inst,
                $is_self,
                is_multi_language_property,
                as_multi_language_property
            );
            cast_check!(inst, $is_self, is_range, as_range);
            cast_check!(inst, $is_self, is_reference_element, as_reference_element);
            cast_check!(inst, $is_self, is_blob, as_blob);
            cast_check!(inst, $is_self, is_file, as_file);
            cast_check!(
                inst,
                $is_self,
                is_annotated_relationship_element,
                as_annotated_relationship_element
            );
            cast_check!(inst, $is_self, is_entity, as_entity);
            cast_check!(inst, $is_self, is_event_payload, as_event_payload);
            cast_check!(
                inst,
                $is_self,
                is_basic_event_element,
                as_basic_event_element
            );
            cast_check!(inst, $is_self, is_operation, as_operation);
            cast_check!(inst, $is_self, is_operation_variable, as_operation_variable);
            cast_check!(inst, $is_self, is_capability, as_capability);
            cast_check!(
                inst,
                $is_self,
                is_concept_description,
                as_concept_description
            );
            cast_check!(inst, $is_self, is_reference, as_reference);
            cast_check!(inst, $is_self, is_key, as_key);
            cast_check!(
                inst,
                $is_self,
                is_lang_string_name_type,
                as_lang_string_name_type
            );
            cast_check!(
                inst,
                $is_self,
                is_lang_string_text_type,
                as_lang_string_text_type
            );
            cast_check!(inst, $is_self, is_environment, as_environment);
            cast_check!(
                inst,
                $is_self,
                is_embedded_data_specification,
                as_embedded_data_specification
            );
            cast_check!(inst, $is_self, is_level_type, as_level_type);
            cast_check!(
                inst,
                $is_self,
                is_value_reference_pair,
                as_value_reference_pair
            );
            cast_check!(inst, $is_self, is_value_list, as_value_list);
            cast_check!(
                inst,
                $is_self,
                is_lang_string_preferred_name_type_iec61360,
                as_lang_string_preferred_name_type_iec61360
            );
            cast_check!(
                inst,
                $is_self,
                is_lang_string_short_name_type_iec61360,
                as_lang_string_short_name_type_iec61360
            );
            cast_check!(
                inst,
                $is_self,
                is_lang_string_definition_type_iec61360,
                as_lang_string_definition_type_iec61360
            );
            cast_check!(
                inst,
                $is_self,
                is_data_specification_iec61360,
                as_data_specification_iec61360
            );
        }
    };
}

exhaustive_cast_test!(
    test_casts_over_extension,
    common::load_minimal_of("Extension", |c| c.is_extension()),
    is_extension,
    as_extension
);

exhaustive_cast_test!(
    test_casts_over_administrative_information,
    common::load_minimal_of("AdministrativeInformation", |c| {
        c.is_administrative_information()
    }),
    is_administrative_information,
    as_administrative_information
);

exhaustive_cast_test!(
    test_casts_over_qualifier,
    common::load_minimal_of("Qualifier", |c| c.is_qualifier()),
    is_qualifier,
    as_qualifier
);

exhaustive_cast_test!(
    test_casts_over_asset_administration_shell,
    common::load_minimal_of("AssetAdministrationShell", |c| {
        c.is_asset_administration_shell()
    }),
    is_asset_administration_shell,
    as_asset_administration_shell
);

exhaustive_cast_test!(
    test_casts_over_asset_information,
    common::load_minimal_of("AssetInformation", |c| c.is_asset_information()),
    is_asset_information,
    as_asset_information
);

exhaustive_cast_test!(
    test_casts_over_resource,
    common::load_minimal_of("Resource", |c| c.is_resource()),
    is_resource,
    as_resource
);

exhaustive_cast_test!(
    test_casts_over_specific_asset_id,
    common::load_minimal_of("SpecificAssetId", |c| c.is_specific_asset_id()),
    is_specific_asset_id,
    as_specific_asset_id
);

exhaustive_cast_test!(
    test_casts_over_submodel,
    common::load_minimal_of("Submodel", |c| c.is_submodel()),
    is_submodel,
    as_submodel
);

exhaustive_cast_test!(
    test_casts_over_relationship_element,
    common::load_minimal_of("RelationshipElement", |c| c.is_relationship_element()),
    is_relationship_element,
    as_relationship_element
);

exhaustive_cast_test!(
    test_casts_over_submodel_element_list,
    common::load_minimal_of("SubmodelElementList", |c| c.is_submodel_element_list()),
    is_submodel_element_list,
    as_submodel_element_list
);

exhaustive_cast_test!(
    test_casts_over_submodel_element_collection,
    common::load_minimal_of("SubmodelElementCollection", |c| {
        c.is_submodel_element_collection()
    }),
    is_submodel_element_collection,
    as_submodel_element_collection
);

exhaustive_cast_test!(
    test_casts_over_property,
    common::load_minimal_of("Property", |c| c.is_property()),
    is_property,
    as_property
);

exhaustive_cast_test!(
    test_casts_over_multi_language_property,
    common::load_minimal_of("MultiLanguageProperty", |c| c.is_multi_language_property()),
    is_multi_language_property,
    as_multi_language_property
);

exhaustive_cast_test!(
    test_casts_over_range,
    common::load_minimal_of("Range", |c| c.is_range()),
    is_range,
    as_range
);

exhaustive_cast_test!(
    test_casts_over_reference_element,
    common::load_minimal_of("ReferenceElement", |c| c.is_reference_element()),
    is_reference_element,
    as_reference_element
);

exhaustive_cast_test!(
    test_casts_over_blob,
    common::load_minimal_of("Blob", |c| c.is_blob()),
    is_blob,
    as_blob
);

exhaustive_cast_test!(
    test_casts_over_file,
    common::load_minimal_of("File", |c| c.is_file()),
    is_file,
    as_file
);

exhaustive_cast_test!(
    test_casts_over_annotated_relationship_element,
    common::load_minimal_of("AnnotatedRelationshipElement", |c| {
        c.is_annotated_relationship_element()
    }),
    is_annotated_relationship_element,
    as_annotated_relationship_element
);

exhaustive_cast_test!(
    test_casts_over_entity,
    common::load_minimal_of("Entity", |c| c.is_entity()),
    is_entity,
    as_entity
);

exhaustive_cast_test!(
    test_casts_over_event_payload,
    common::load_minimal_event_payload(),
    is_event_payload,
    as_event_payload
);

exhaustive_cast_test!(
    test_casts_over_basic_event_element,
    common::load_minimal_of("BasicEventElement", |c| c.is_basic_event_element()),
    is_basic_event_element,
    as_basic_event_element
);

exhaustive_cast_test!(
    test_casts_over_operation,
    common::load_minimal_of("Operation", |c| c.is_operation()),
    is_operation,
    as_operation
);

exhaustive_cast_test!(
    test_casts_over_operation_variable,
    common::load_minimal_of("OperationVariable", |c| c.is_operation_variable()),
    is_operation_variable,
    as_operation_variable
);

exhaustive_cast_test!(
    test_casts_over_capability,
    common::load_minimal_of("Capability", |c| c.is_capability()),
    is_capability,
    as_capability
);

exhaustive_cast_test!(
    test_casts_over_concept_description,
    common::load_minimal_of("ConceptDescription", |c| c.is_concept_description()),
    is_concept_description,
    as_concept_description
);

exhaustive_cast_test!(
    test_casts_over_reference,
    common::load_minimal_of("Reference", |c| c.is_reference()),
    is_reference,
    as_reference
);

exhaustive_cast_test!(
    test_casts_over_key,
    common::load_minimal_of("Key", |c| c.is_key()),
    is_key,
    as_key
);

exhaustive_cast_test!(
    test_casts_over_lang_string_name_type,
    common::load_minimal_of("LangStringNameType", |c| c.is_lang_string_name_type()),
    is_lang_string_name_type,
    as_lang_string_name_type
);

exhaustive_cast_test!(
    test_casts_over_lang_string_text_type,
    common::load_minimal_of("LangStringTextType", |c| c.is_lang_string_text_type()),
    is_lang_string_text_type,
    as_lang_string_text_type
);

exhaustive_cast_test!(
    test_casts_over_environment,
    common::load_minimal_of("Submodel", |c| c.is_environment()),
    is_environment,
    as_environment
);

exhaustive_cast_test!(
    test_casts_over_embedded_data_specification,
    common::load_minimal_of("EmbeddedDataSpecification", |c| {
        c.is_embedded_data_specification()
    }),
    is_embedded_data_specification,
    as_embedded_data_specification
);

exhaustive_cast_test!(
    test_casts_over_level_type,
    common::load_minimal_of("LevelType", |c| c.is_level_type()),
    is_level_type,
    as_level_type
);

exhaustive_cast_test!(
    test_casts_over_value_reference_pair,
    common::load_minimal_of("ValueReferencePair", |c| c.is_value_reference_pair()),
    is_value_reference_pair,
    as_value_reference_pair
);

exhaustive_cast_test!(
    test_casts_over_value_list,
    common::load_minimal_of("ValueList", |c| c.is_value_list()),
    is_value_list,
    as_value_list
);

exhaustive_cast_test!(
    test_casts_over_lang_string_preferred_name_type_iec61360,
    common::load_minimal_of("LangStringPreferredNameTypeIec61360", |c| {
        c.is_lang_string_preferred_name_type_iec61360()
    }),
    is_lang_string_preferred_name_type_iec61360,
    as_lang_string_preferred_name_type_iec61360
);

exhaustive_cast_test!(
    test_casts_over_lang_string_short_name_type_iec61360,
    common::load_minimal_of("LangStringShortNameTypeIec61360", |c| {
        c.is_lang_string_short_name_type_iec61360()
    }),
    is_lang_string_short_name_type_iec61360,
    as_lang_string_short_name_type_iec61360
);

exhaustive_cast_test!(
    test_casts_over_lang_string_definition_type_iec61360,
    common::load_minimal_of("LangStringDefinitionTypeIec61360", |c| {
        c.is_lang_string_definition_type_iec61360()
    }),
    is_lang_string_definition_type_iec61360,
    as_lang_string_definition_type_iec61360
);

exhaustive_cast_test!(
    test_casts_over_data_specification_iec61360,
    common::load_minimal_of("DataSpecificationIec61360", |c| {
        c.is_data_specification_iec61360()
    }),
    is_data_specification_iec61360,
    as_data_specification_iec61360
);
