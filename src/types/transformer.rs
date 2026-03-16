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
    EventPayload, Extension, Key, LevelType, OperationVariable, Qualifier, Reference,
    SpecificAssetId, ValueList, ValueReferencePair, Resource,
};

/// Transformer trait that transforms AAS element types to `T`.
pub trait Transformer<T> {
    /// Transforms an [`Extension`].
    fn transform_extension(&mut self, that: &Extension) -> T;
    /// Transforms an [`AdministrativeInformation`].
    fn transform_administrative_information(&mut self, that: &AdministrativeInformation) -> T;
    /// Transforms a [`Qualifier`].
    fn transform_qualifier(&mut self, that: &Qualifier) -> T;
    /// Transforms an [`AssetAdministrationShell`].
    fn transform_asset_administration_shell(&mut self, that: &AssetAdministrationShell) -> T;
    /// Transforms an [`AssetInformation`].
    fn transform_asset_information(&mut self, that: &AssetInformation) -> T;
    /// Transforms a [`Resource`].
    fn transform_resource(&mut self, that: &Resource) -> T;
    /// Transforms a [`SpecificAssetId`].
    fn transform_specific_asset_id(&mut self, that: &SpecificAssetId) -> T;
    /// Transforms a [`Submodel`].
    fn transform_submodel(&mut self, that: &Submodel) -> T;
    /// Transforms a [`RelationshipElement`].
    fn transform_relationship_element(&mut self, that: &RelationshipElement) -> T;
    /// Transforms a [`SubmodelElementList`].
    fn transform_submodel_element_list(&mut self, that: &SubmodelElementList) -> T;
    /// Transforms a [`SubmodelElementCollection`].
    fn transform_submodel_element_collection(&mut self, that: &SubmodelElementCollection) -> T;
    /// Transforms a [`Property`].
    fn transform_property(&mut self, that: &Property) -> T;
    /// Transforms a [`MultiLanguageProperty`].
    fn transform_multi_language_property(&mut self, that: &MultiLanguageProperty) -> T;
    /// Transforms a [`Range`].
    fn transform_range(&mut self, that: &Range) -> T;
    /// Transforms a [`ReferenceElement`].
    fn transform_reference_element(&mut self, that: &ReferenceElement) -> T;
    /// Transforms a [`Blob`].
    fn transform_blob(&mut self, that: &Blob) -> T;
    /// Transforms a [`File`].
    fn transform_file(&mut self, that: &File) -> T;
    /// Transforms an [`AnnotatedRelationshipElement`].
    fn transform_annotated_relationship_element(
        &mut self,
        that: &AnnotatedRelationshipElement,
    ) -> T;
    /// Transforms an [`Entity`].
    fn transform_entity(&mut self, that: &Entity) -> T;
    /// Transforms an [`EventPayload`].
    fn transform_event_payload(&mut self, that: &EventPayload) -> T;
    /// Transforms a [`BasicEventElement`].
    fn transform_basic_event_element(&mut self, that: &BasicEventElement) -> T;
    /// Transforms an [`Operation`].
    fn transform_operation(&mut self, that: &Operation) -> T;
    /// Transforms an [`OperationVariable`].
    fn transform_operation_variable(&mut self, that: &OperationVariable) -> T;
    /// Transforms a [`Capability`].
    fn transform_capability(&mut self, that: &Capability) -> T;
    /// Transforms a [`ConceptDescription`].
    fn transform_concept_description(&mut self, that: &ConceptDescription) -> T;
    /// Transforms a [`Reference`].
    fn transform_reference(&mut self, that: &Reference) -> T;
    /// Transforms a [`Key`].
    fn transform_key(&mut self, that: &Key) -> T;
    /// Transforms a [`LangStringNameType`].
    fn transform_lang_string_name_type(&mut self, that: &LangStringNameType) -> T;
    /// Transforms a [`LangStringTextType`].
    fn transform_lang_string_text_type(&mut self, that: &LangStringTextType) -> T;
    /// Transforms an [`Environment`].
    fn transform_environment(&mut self, that: &Environment) -> T;
    /// Transforms an [`EmbeddedDataSpecification`].
    fn transform_embedded_data_specification(&mut self, that: &EmbeddedDataSpecification) -> T;
    /// Transforms a [`LevelType`].
    fn transform_level_type(&mut self, that: &LevelType) -> T;
    /// Transforms a [`ValueReferencePair`].
    fn transform_value_reference_pair(&mut self, that: &ValueReferencePair) -> T;
    /// Transforms a [`ValueList`].
    fn transform_value_list(&mut self, that: &ValueList) -> T;
    /// Transforms a [`LangStringPreferredNameTypeIec61360`].
    fn transform_lang_string_preferred_name_type_iec61360(
        &mut self,
        that: &LangStringPreferredNameTypeIec61360,
    ) -> T;
    /// Transforms a [`LangStringShortNameTypeIec61360`].
    fn transform_lang_string_short_name_type_iec61360(
        &mut self,
        that: &LangStringShortNameTypeIec61360,
    ) -> T;
    /// Transforms a [`LangStringDefinitionTypeIec61360`].
    fn transform_lang_string_definition_type_iec61360(
        &mut self,
        that: &LangStringDefinitionTypeIec61360,
    ) -> T;
    /// Transforms a [`DataSpecificationIec61360`].
    fn transform_data_specification_iec61360(&mut self, that: &DataSpecificationIec61360) -> T;
}

/// Transformer trait with context that transforms AAS element types to `T`.
#[allow(clippy::module_name_repetitions)]
pub trait TransformerWithContext<C, T> {
    /// Transforms an [`Extension`] with context.
    fn transform_extension_with_context(&mut self, that: &Extension, context: &C) -> T;
    /// Transforms an [`AdministrativeInformation`] with context.
    fn transform_administrative_information_with_context(
        &mut self,
        that: &AdministrativeInformation,
        context: &C,
    ) -> T;
    /// Transforms a [`Qualifier`] with context.
    fn transform_qualifier_with_context(&mut self, that: &Qualifier, context: &C) -> T;
    /// Transforms an [`AssetAdministrationShell`] with context.
    fn transform_asset_administration_shell_with_context(
        &mut self,
        that: &AssetAdministrationShell,
        context: &C,
    ) -> T;
    /// Transforms an [`AssetInformation`] with context.
    fn transform_asset_information_with_context(&mut self, that: &AssetInformation, context: &C)
        -> T;
    /// Transforms a [`Resource`] with context.
    fn transform_resource_with_context(&mut self, that: &Resource, context: &C) -> T;
    /// Transforms a [`SpecificAssetId`] with context.
    fn transform_specific_asset_id_with_context(
        &mut self,
        that: &SpecificAssetId,
        context: &C,
    ) -> T;
    /// Transforms a [`Submodel`] with context.
    fn transform_submodel_with_context(&mut self, that: &Submodel, context: &C) -> T;
    /// Transforms a [`RelationshipElement`] with context.
    fn transform_relationship_element_with_context(
        &mut self,
        that: &RelationshipElement,
        context: &C,
    ) -> T;
    /// Transforms a [`SubmodelElementList`] with context.
    fn transform_submodel_element_list_with_context(
        &mut self,
        that: &SubmodelElementList,
        context: &C,
    ) -> T;
    /// Transforms a [`SubmodelElementCollection`] with context.
    fn transform_submodel_element_collection_with_context(
        &mut self,
        that: &SubmodelElementCollection,
        context: &C,
    ) -> T;
    /// Transforms a [`Property`] with context.
    fn transform_property_with_context(&mut self, that: &Property, context: &C) -> T;
    /// Transforms a [`MultiLanguageProperty`] with context.
    fn transform_multi_language_property_with_context(
        &mut self,
        that: &MultiLanguageProperty,
        context: &C,
    ) -> T;
    /// Transforms a [`Range`] with context.
    fn transform_range_with_context(&mut self, that: &Range, context: &C) -> T;
    /// Transforms a [`ReferenceElement`] with context.
    fn transform_reference_element_with_context(
        &mut self,
        that: &ReferenceElement,
        context: &C,
    ) -> T;
    /// Transforms a [`Blob`] with context.
    fn transform_blob_with_context(&mut self, that: &Blob, context: &C) -> T;
    /// Transforms a [`File`] with context.
    fn transform_file_with_context(&mut self, that: &File, context: &C) -> T;
    /// Transforms an [`AnnotatedRelationshipElement`] with context.
    fn transform_annotated_relationship_element_with_context(
        &mut self,
        that: &AnnotatedRelationshipElement,
        context: &C,
    ) -> T;
    /// Transforms an [`Entity`] with context.
    fn transform_entity_with_context(&mut self, that: &Entity, context: &C) -> T;
    /// Transforms an [`EventPayload`] with context.
    fn transform_event_payload_with_context(&mut self, that: &EventPayload, context: &C) -> T;
    /// Transforms a [`BasicEventElement`] with context.
    fn transform_basic_event_element_with_context(
        &mut self,
        that: &BasicEventElement,
        context: &C,
    ) -> T;
    /// Transforms an [`Operation`] with context.
    fn transform_operation_with_context(&mut self, that: &Operation, context: &C) -> T;
    /// Transforms an [`OperationVariable`] with context.
    fn transform_operation_variable_with_context(
        &mut self,
        that: &OperationVariable,
        context: &C,
    ) -> T;
    /// Transforms a [`Capability`] with context.
    fn transform_capability_with_context(&mut self, that: &Capability, context: &C) -> T;
    /// Transforms a [`ConceptDescription`] with context.
    fn transform_concept_description_with_context(
        &mut self,
        that: &ConceptDescription,
        context: &C,
    ) -> T;
    /// Transforms a [`Reference`] with context.
    fn transform_reference_with_context(&mut self, that: &Reference, context: &C) -> T;
    /// Transforms a [`Key`] with context.
    fn transform_key_with_context(&mut self, that: &Key, context: &C) -> T;
    /// Transforms a [`LangStringNameType`] with context.
    fn transform_lang_string_name_type_with_context(
        &mut self,
        that: &LangStringNameType,
        context: &C,
    ) -> T;
    /// Transforms a [`LangStringTextType`] with context.
    fn transform_lang_string_text_type_with_context(
        &mut self,
        that: &LangStringTextType,
        context: &C,
    ) -> T;
    /// Transforms an [`Environment`] with context.
    fn transform_environment_with_context(&mut self, that: &Environment, context: &C) -> T;
    /// Transforms an [`EmbeddedDataSpecification`] with context.
    fn transform_embedded_data_specification_with_context(
        &mut self,
        that: &EmbeddedDataSpecification,
        context: &C,
    ) -> T;
    /// Transforms a [`LevelType`] with context.
    fn transform_level_type_with_context(&mut self, that: &LevelType, context: &C) -> T;
    /// Transforms a [`ValueReferencePair`] with context.
    fn transform_value_reference_pair_with_context(
        &mut self,
        that: &ValueReferencePair,
        context: &C,
    ) -> T;
    /// Transforms a [`ValueList`] with context.
    fn transform_value_list_with_context(&mut self, that: &ValueList, context: &C) -> T;
    /// Transforms a [`LangStringPreferredNameTypeIec61360`] with context.
    fn transform_lang_string_preferred_name_type_iec61360_with_context(
        &mut self,
        that: &LangStringPreferredNameTypeIec61360,
        context: &C,
    ) -> T;
    /// Transforms a [`LangStringShortNameTypeIec61360`] with context.
    fn transform_lang_string_short_name_type_iec61360_with_context(
        &mut self,
        that: &LangStringShortNameTypeIec61360,
        context: &C,
    ) -> T;
    /// Transforms a [`LangStringDefinitionTypeIec61360`] with context.
    fn transform_lang_string_definition_type_iec61360_with_context(
        &mut self,
        that: &LangStringDefinitionTypeIec61360,
        context: &C,
    ) -> T;
    /// Transforms a [`DataSpecificationIec61360`] with context.
    fn transform_data_specification_iec61360_with_context(
        &mut self,
        that: &DataSpecificationIec61360,
        context: &C,
    ) -> T;
}

/// A transformer that returns the default value for all element types.
#[allow(clippy::module_name_repetitions)]
pub struct TransformerWithDefault<T: Default>(pub std::marker::PhantomData<T>);

impl<T: Default> TransformerWithDefault<T> {
    /// Creates a new [`TransformerWithDefault`].
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Default> Default for TransformerWithDefault<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> Transformer<T> for TransformerWithDefault<T> {
    fn transform_extension(&mut self, _that: &Extension) -> T {
        T::default()
    }
    fn transform_administrative_information(&mut self, _that: &AdministrativeInformation) -> T {
        T::default()
    }
    fn transform_qualifier(&mut self, _that: &Qualifier) -> T {
        T::default()
    }
    fn transform_asset_administration_shell(&mut self, _that: &AssetAdministrationShell) -> T {
        T::default()
    }
    fn transform_asset_information(&mut self, _that: &AssetInformation) -> T {
        T::default()
    }
    fn transform_resource(&mut self, _that: &Resource) -> T {
        T::default()
    }
    fn transform_specific_asset_id(&mut self, _that: &SpecificAssetId) -> T {
        T::default()
    }
    fn transform_submodel(&mut self, _that: &Submodel) -> T {
        T::default()
    }
    fn transform_relationship_element(&mut self, _that: &RelationshipElement) -> T {
        T::default()
    }
    fn transform_submodel_element_list(&mut self, _that: &SubmodelElementList) -> T {
        T::default()
    }
    fn transform_submodel_element_collection(&mut self, _that: &SubmodelElementCollection) -> T {
        T::default()
    }
    fn transform_property(&mut self, _that: &Property) -> T {
        T::default()
    }
    fn transform_multi_language_property(&mut self, _that: &MultiLanguageProperty) -> T {
        T::default()
    }
    fn transform_range(&mut self, _that: &Range) -> T {
        T::default()
    }
    fn transform_reference_element(&mut self, _that: &ReferenceElement) -> T {
        T::default()
    }
    fn transform_blob(&mut self, _that: &Blob) -> T {
        T::default()
    }
    fn transform_file(&mut self, _that: &File) -> T {
        T::default()
    }
    fn transform_annotated_relationship_element(
        &mut self,
        _that: &AnnotatedRelationshipElement,
    ) -> T {
        T::default()
    }
    fn transform_entity(&mut self, _that: &Entity) -> T {
        T::default()
    }
    fn transform_event_payload(&mut self, _that: &EventPayload) -> T {
        T::default()
    }
    fn transform_basic_event_element(&mut self, _that: &BasicEventElement) -> T {
        T::default()
    }
    fn transform_operation(&mut self, _that: &Operation) -> T {
        T::default()
    }
    fn transform_operation_variable(&mut self, _that: &OperationVariable) -> T {
        T::default()
    }
    fn transform_capability(&mut self, _that: &Capability) -> T {
        T::default()
    }
    fn transform_concept_description(&mut self, _that: &ConceptDescription) -> T {
        T::default()
    }
    fn transform_reference(&mut self, _that: &Reference) -> T {
        T::default()
    }
    fn transform_key(&mut self, _that: &Key) -> T {
        T::default()
    }
    fn transform_lang_string_name_type(&mut self, _that: &LangStringNameType) -> T {
        T::default()
    }
    fn transform_lang_string_text_type(&mut self, _that: &LangStringTextType) -> T {
        T::default()
    }
    fn transform_environment(&mut self, _that: &Environment) -> T {
        T::default()
    }
    fn transform_embedded_data_specification(&mut self, _that: &EmbeddedDataSpecification) -> T {
        T::default()
    }
    fn transform_level_type(&mut self, _that: &LevelType) -> T {
        T::default()
    }
    fn transform_value_reference_pair(&mut self, _that: &ValueReferencePair) -> T {
        T::default()
    }
    fn transform_value_list(&mut self, _that: &ValueList) -> T {
        T::default()
    }
    fn transform_lang_string_preferred_name_type_iec61360(
        &mut self,
        _that: &LangStringPreferredNameTypeIec61360,
    ) -> T {
        T::default()
    }
    fn transform_lang_string_short_name_type_iec61360(
        &mut self,
        _that: &LangStringShortNameTypeIec61360,
    ) -> T {
        T::default()
    }
    fn transform_lang_string_definition_type_iec61360(
        &mut self,
        _that: &LangStringDefinitionTypeIec61360,
    ) -> T {
        T::default()
    }
    fn transform_data_specification_iec61360(&mut self, _that: &DataSpecificationIec61360) -> T {
        T::default()
    }
}
