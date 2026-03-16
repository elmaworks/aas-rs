use super::structs_core::{
    AssetAdministrationShell, AssetInformation, ConceptDescription, Environment, Submodel,
};
use super::structs_elements::{
    AdministrativeInformation, AnnotatedRelationshipElement, BasicEventElement, Blob, Capability,
    EmbeddedDataSpecification, Entity, File, MultiLanguageProperty, Operation, Property, Range,
    ReferenceElement, RelationshipElement, SubmodelElementCollection, SubmodelElementList,
};
use super::structs_lang::{
    DataSpecificationIec61360, LangStringDefinitionTypeIec61360, LangStringNameType,
    LangStringPreferredNameTypeIec61360, LangStringShortNameTypeIec61360, LangStringTextType,
};
use super::structs_support::{
    EventPayload, Extension, Key, LevelType, OperationVariable, Qualifier, Reference, Resource,
    SpecificAssetId, ValueList, ValueReferencePair,
};

/// Visitor trait for visiting all AAS element types.
pub trait Visitor {
    /// Visits an [`Extension`].
    fn visit_extension(&mut self, that: &Extension);
    /// Visits an [`AdministrativeInformation`].
    fn visit_administrative_information(&mut self, that: &AdministrativeInformation);
    /// Visits a [`Qualifier`].
    fn visit_qualifier(&mut self, that: &Qualifier);
    /// Visits an [`AssetAdministrationShell`].
    fn visit_asset_administration_shell(&mut self, that: &AssetAdministrationShell);
    /// Visits an [`AssetInformation`].
    fn visit_asset_information(&mut self, that: &AssetInformation);
    /// Visits a [`Resource`].
    fn visit_resource(&mut self, that: &Resource);
    /// Visits a [`SpecificAssetId`].
    fn visit_specific_asset_id(&mut self, that: &SpecificAssetId);
    /// Visits a [`Submodel`].
    fn visit_submodel(&mut self, that: &Submodel);
    /// Visits a [`RelationshipElement`].
    fn visit_relationship_element(&mut self, that: &RelationshipElement);
    /// Visits a [`SubmodelElementList`].
    fn visit_submodel_element_list(&mut self, that: &SubmodelElementList);
    /// Visits a [`SubmodelElementCollection`].
    fn visit_submodel_element_collection(&mut self, that: &SubmodelElementCollection);
    /// Visits a [`Property`].
    fn visit_property(&mut self, that: &Property);
    /// Visits a [`MultiLanguageProperty`].
    fn visit_multi_language_property(&mut self, that: &MultiLanguageProperty);
    /// Visits a [`Range`].
    fn visit_range(&mut self, that: &Range);
    /// Visits a [`ReferenceElement`].
    fn visit_reference_element(&mut self, that: &ReferenceElement);
    /// Visits a [`Blob`].
    fn visit_blob(&mut self, that: &Blob);
    /// Visits a [`File`].
    fn visit_file(&mut self, that: &File);
    /// Visits an [`AnnotatedRelationshipElement`].
    fn visit_annotated_relationship_element(&mut self, that: &AnnotatedRelationshipElement);
    /// Visits an [`Entity`].
    fn visit_entity(&mut self, that: &Entity);
    /// Visits an [`EventPayload`].
    fn visit_event_payload(&mut self, that: &EventPayload);
    /// Visits a [`BasicEventElement`].
    fn visit_basic_event_element(&mut self, that: &BasicEventElement);
    /// Visits an [`Operation`].
    fn visit_operation(&mut self, that: &Operation);
    /// Visits an [`OperationVariable`].
    fn visit_operation_variable(&mut self, that: &OperationVariable);
    /// Visits a [`Capability`].
    fn visit_capability(&mut self, that: &Capability);
    /// Visits a [`ConceptDescription`].
    fn visit_concept_description(&mut self, that: &ConceptDescription);
    /// Visits a [`Reference`].
    fn visit_reference(&mut self, that: &Reference);
    /// Visits a [`Key`].
    fn visit_key(&mut self, that: &Key);
    /// Visits a [`LangStringNameType`].
    fn visit_lang_string_name_type(&mut self, that: &LangStringNameType);
    /// Visits a [`LangStringTextType`].
    fn visit_lang_string_text_type(&mut self, that: &LangStringTextType);
    /// Visits an [`Environment`].
    fn visit_environment(&mut self, that: &Environment);
    /// Visits an [`EmbeddedDataSpecification`].
    fn visit_embedded_data_specification(&mut self, that: &EmbeddedDataSpecification);
    /// Visits a [`LevelType`].
    fn visit_level_type(&mut self, that: &LevelType);
    /// Visits a [`ValueReferencePair`].
    fn visit_value_reference_pair(&mut self, that: &ValueReferencePair);
    /// Visits a [`ValueList`].
    fn visit_value_list(&mut self, that: &ValueList);
    /// Visits a [`LangStringPreferredNameTypeIec61360`].
    fn visit_lang_string_preferred_name_type_iec61360(
        &mut self,
        that: &LangStringPreferredNameTypeIec61360,
    );
    /// Visits a [`LangStringShortNameTypeIec61360`].
    fn visit_lang_string_short_name_type_iec61360(
        &mut self,
        that: &LangStringShortNameTypeIec61360,
    );
    /// Visits a [`LangStringDefinitionTypeIec61360`].
    fn visit_lang_string_definition_type_iec61360(
        &mut self,
        that: &LangStringDefinitionTypeIec61360,
    );
    /// Visits a [`DataSpecificationIec61360`].
    fn visit_data_specification_iec61360(&mut self, that: &DataSpecificationIec61360);
}

/// Visitor trait with context for visiting all AAS element types.
#[allow(clippy::module_name_repetitions)]
pub trait VisitorWithContext<C> {
    /// Visits an [`Extension`] with context.
    fn visit_extension_with_context(&mut self, that: &Extension, context: &C);
    /// Visits an [`AdministrativeInformation`] with context.
    fn visit_administrative_information_with_context(
        &mut self,
        that: &AdministrativeInformation,
        context: &C,
    );
    /// Visits a [`Qualifier`] with context.
    fn visit_qualifier_with_context(&mut self, that: &Qualifier, context: &C);
    /// Visits an [`AssetAdministrationShell`] with context.
    fn visit_asset_administration_shell_with_context(
        &mut self,
        that: &AssetAdministrationShell,
        context: &C,
    );
    /// Visits an [`AssetInformation`] with context.
    fn visit_asset_information_with_context(&mut self, that: &AssetInformation, context: &C);
    /// Visits a [`Resource`] with context.
    fn visit_resource_with_context(&mut self, that: &Resource, context: &C);
    /// Visits a [`SpecificAssetId`] with context.
    fn visit_specific_asset_id_with_context(&mut self, that: &SpecificAssetId, context: &C);
    /// Visits a [`Submodel`] with context.
    fn visit_submodel_with_context(&mut self, that: &Submodel, context: &C);
    /// Visits a [`RelationshipElement`] with context.
    fn visit_relationship_element_with_context(&mut self, that: &RelationshipElement, context: &C);
    /// Visits a [`SubmodelElementList`] with context.
    fn visit_submodel_element_list_with_context(&mut self, that: &SubmodelElementList, context: &C);
    /// Visits a [`SubmodelElementCollection`] with context.
    fn visit_submodel_element_collection_with_context(
        &mut self,
        that: &SubmodelElementCollection,
        context: &C,
    );
    /// Visits a [`Property`] with context.
    fn visit_property_with_context(&mut self, that: &Property, context: &C);
    /// Visits a [`MultiLanguageProperty`] with context.
    fn visit_multi_language_property_with_context(
        &mut self,
        that: &MultiLanguageProperty,
        context: &C,
    );
    /// Visits a [`Range`] with context.
    fn visit_range_with_context(&mut self, that: &Range, context: &C);
    /// Visits a [`ReferenceElement`] with context.
    fn visit_reference_element_with_context(&mut self, that: &ReferenceElement, context: &C);
    /// Visits a [`Blob`] with context.
    fn visit_blob_with_context(&mut self, that: &Blob, context: &C);
    /// Visits a [`File`] with context.
    fn visit_file_with_context(&mut self, that: &File, context: &C);
    /// Visits an [`AnnotatedRelationshipElement`] with context.
    fn visit_annotated_relationship_element_with_context(
        &mut self,
        that: &AnnotatedRelationshipElement,
        context: &C,
    );
    /// Visits an [`Entity`] with context.
    fn visit_entity_with_context(&mut self, that: &Entity, context: &C);
    /// Visits an [`EventPayload`] with context.
    fn visit_event_payload_with_context(&mut self, that: &EventPayload, context: &C);
    /// Visits a [`BasicEventElement`] with context.
    fn visit_basic_event_element_with_context(&mut self, that: &BasicEventElement, context: &C);
    /// Visits an [`Operation`] with context.
    fn visit_operation_with_context(&mut self, that: &Operation, context: &C);
    /// Visits an [`OperationVariable`] with context.
    fn visit_operation_variable_with_context(&mut self, that: &OperationVariable, context: &C);
    /// Visits a [`Capability`] with context.
    fn visit_capability_with_context(&mut self, that: &Capability, context: &C);
    /// Visits a [`ConceptDescription`] with context.
    fn visit_concept_description_with_context(&mut self, that: &ConceptDescription, context: &C);
    /// Visits a [`Reference`] with context.
    fn visit_reference_with_context(&mut self, that: &Reference, context: &C);
    /// Visits a [`Key`] with context.
    fn visit_key_with_context(&mut self, that: &Key, context: &C);
    /// Visits a [`LangStringNameType`] with context.
    fn visit_lang_string_name_type_with_context(&mut self, that: &LangStringNameType, context: &C);
    /// Visits a [`LangStringTextType`] with context.
    fn visit_lang_string_text_type_with_context(&mut self, that: &LangStringTextType, context: &C);
    /// Visits an [`Environment`] with context.
    fn visit_environment_with_context(&mut self, that: &Environment, context: &C);
    /// Visits an [`EmbeddedDataSpecification`] with context.
    fn visit_embedded_data_specification_with_context(
        &mut self,
        that: &EmbeddedDataSpecification,
        context: &C,
    );
    /// Visits a [`LevelType`] with context.
    fn visit_level_type_with_context(&mut self, that: &LevelType, context: &C);
    /// Visits a [`ValueReferencePair`] with context.
    fn visit_value_reference_pair_with_context(&mut self, that: &ValueReferencePair, context: &C);
    /// Visits a [`ValueList`] with context.
    fn visit_value_list_with_context(&mut self, that: &ValueList, context: &C);
    /// Visits a [`LangStringPreferredNameTypeIec61360`] with context.
    fn visit_lang_string_preferred_name_type_iec61360_with_context(
        &mut self,
        that: &LangStringPreferredNameTypeIec61360,
        context: &C,
    );
    /// Visits a [`LangStringShortNameTypeIec61360`] with context.
    fn visit_lang_string_short_name_type_iec61360_with_context(
        &mut self,
        that: &LangStringShortNameTypeIec61360,
        context: &C,
    );
    /// Visits a [`LangStringDefinitionTypeIec61360`] with context.
    fn visit_lang_string_definition_type_iec61360_with_context(
        &mut self,
        that: &LangStringDefinitionTypeIec61360,
        context: &C,
    );
    /// Visits a [`DataSpecificationIec61360`] with context.
    fn visit_data_specification_iec61360_with_context(
        &mut self,
        that: &DataSpecificationIec61360,
        context: &C,
    );
}
