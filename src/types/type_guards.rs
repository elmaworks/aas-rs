use super::class::Class;
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

impl Class {
    /// Returns a reference to the inner [`Extension`] if this is `Class::Extension`.
    pub fn as_extension(&self) -> Option<&Extension> {
        if let Class::Extension(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Extension`] if this is `Class::Extension`.
    pub fn as_extension_mut(&mut self) -> Option<&mut Extension> {
        if let Class::Extension(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Extension`.
    pub fn is_extension(&self) -> bool {
        self.as_extension().is_some()
    }

    /// Returns a reference to the inner [`AdministrativeInformation`] if applicable.
    pub fn as_administrative_information(&self) -> Option<&AdministrativeInformation> {
        if let Class::AdministrativeInformation(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`AdministrativeInformation`] if applicable.
    pub fn as_administrative_information_mut(&mut self) -> Option<&mut AdministrativeInformation> {
        if let Class::AdministrativeInformation(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::AdministrativeInformation`.
    pub fn is_administrative_information(&self) -> bool {
        self.as_administrative_information().is_some()
    }

    /// Returns a reference to the inner [`Qualifier`] if applicable.
    pub fn as_qualifier(&self) -> Option<&Qualifier> {
        if let Class::Qualifier(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Qualifier`] if applicable.
    pub fn as_qualifier_mut(&mut self) -> Option<&mut Qualifier> {
        if let Class::Qualifier(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Qualifier`.
    pub fn is_qualifier(&self) -> bool {
        self.as_qualifier().is_some()
    }

    /// Returns a reference to the inner [`AssetAdministrationShell`] if applicable.
    pub fn as_asset_administration_shell(&self) -> Option<&AssetAdministrationShell> {
        if let Class::AssetAdministrationShell(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`AssetAdministrationShell`] if applicable.
    pub fn as_asset_administration_shell_mut(&mut self) -> Option<&mut AssetAdministrationShell> {
        if let Class::AssetAdministrationShell(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::AssetAdministrationShell`.
    pub fn is_asset_administration_shell(&self) -> bool {
        self.as_asset_administration_shell().is_some()
    }

    /// Returns a reference to the inner [`AssetInformation`] if applicable.
    pub fn as_asset_information(&self) -> Option<&AssetInformation> {
        if let Class::AssetInformation(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`AssetInformation`] if applicable.
    pub fn as_asset_information_mut(&mut self) -> Option<&mut AssetInformation> {
        if let Class::AssetInformation(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::AssetInformation`.
    pub fn is_asset_information(&self) -> bool {
        self.as_asset_information().is_some()
    }

    /// Returns a reference to the inner [`Resource`] if applicable.
    pub fn as_resource(&self) -> Option<&Resource> {
        if let Class::Resource(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Resource`] if applicable.
    pub fn as_resource_mut(&mut self) -> Option<&mut Resource> {
        if let Class::Resource(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Resource`.
    pub fn is_resource(&self) -> bool {
        self.as_resource().is_some()
    }

    /// Returns a reference to the inner [`SpecificAssetId`] if applicable.
    pub fn as_specific_asset_id(&self) -> Option<&SpecificAssetId> {
        if let Class::SpecificAssetId(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`SpecificAssetId`] if applicable.
    pub fn as_specific_asset_id_mut(&mut self) -> Option<&mut SpecificAssetId> {
        if let Class::SpecificAssetId(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::SpecificAssetId`.
    pub fn is_specific_asset_id(&self) -> bool {
        self.as_specific_asset_id().is_some()
    }

    /// Returns a reference to the inner [`Submodel`] if applicable.
    pub fn as_submodel(&self) -> Option<&Submodel> {
        if let Class::Submodel(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Submodel`] if applicable.
    pub fn as_submodel_mut(&mut self) -> Option<&mut Submodel> {
        if let Class::Submodel(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Submodel`.
    pub fn is_submodel(&self) -> bool {
        self.as_submodel().is_some()
    }

    /// Returns a reference to the inner [`RelationshipElement`] if applicable.
    pub fn as_relationship_element(&self) -> Option<&RelationshipElement> {
        if let Class::RelationshipElement(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`RelationshipElement`] if applicable.
    pub fn as_relationship_element_mut(&mut self) -> Option<&mut RelationshipElement> {
        if let Class::RelationshipElement(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::RelationshipElement`.
    pub fn is_relationship_element(&self) -> bool {
        self.as_relationship_element().is_some()
    }

    /// Returns a reference to the inner [`SubmodelElementList`] if applicable.
    pub fn as_submodel_element_list(&self) -> Option<&SubmodelElementList> {
        if let Class::SubmodelElementList(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`SubmodelElementList`] if applicable.
    pub fn as_submodel_element_list_mut(&mut self) -> Option<&mut SubmodelElementList> {
        if let Class::SubmodelElementList(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::SubmodelElementList`.
    pub fn is_submodel_element_list(&self) -> bool {
        self.as_submodel_element_list().is_some()
    }

    /// Returns a reference to the inner [`SubmodelElementCollection`] if applicable.
    pub fn as_submodel_element_collection(&self) -> Option<&SubmodelElementCollection> {
        if let Class::SubmodelElementCollection(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`SubmodelElementCollection`] if applicable.
    pub fn as_submodel_element_collection_mut(&mut self) -> Option<&mut SubmodelElementCollection> {
        if let Class::SubmodelElementCollection(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::SubmodelElementCollection`.
    pub fn is_submodel_element_collection(&self) -> bool {
        self.as_submodel_element_collection().is_some()
    }

    /// Returns a reference to the inner [`Property`] if applicable.
    pub fn as_property(&self) -> Option<&Property> {
        if let Class::Property(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Property`] if applicable.
    pub fn as_property_mut(&mut self) -> Option<&mut Property> {
        if let Class::Property(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Property`.
    pub fn is_property(&self) -> bool {
        self.as_property().is_some()
    }

    /// Returns a reference to the inner [`MultiLanguageProperty`] if applicable.
    pub fn as_multi_language_property(&self) -> Option<&MultiLanguageProperty> {
        if let Class::MultiLanguageProperty(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`MultiLanguageProperty`] if applicable.
    pub fn as_multi_language_property_mut(&mut self) -> Option<&mut MultiLanguageProperty> {
        if let Class::MultiLanguageProperty(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::MultiLanguageProperty`.
    pub fn is_multi_language_property(&self) -> bool {
        self.as_multi_language_property().is_some()
    }

    /// Returns a reference to the inner [`Range`] if applicable.
    pub fn as_range(&self) -> Option<&Range> {
        if let Class::Range(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Range`] if applicable.
    pub fn as_range_mut(&mut self) -> Option<&mut Range> {
        if let Class::Range(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Range`.
    pub fn is_range(&self) -> bool {
        self.as_range().is_some()
    }

    /// Returns a reference to the inner [`ReferenceElement`] if applicable.
    pub fn as_reference_element(&self) -> Option<&ReferenceElement> {
        if let Class::ReferenceElement(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`ReferenceElement`] if applicable.
    pub fn as_reference_element_mut(&mut self) -> Option<&mut ReferenceElement> {
        if let Class::ReferenceElement(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::ReferenceElement`.
    pub fn is_reference_element(&self) -> bool {
        self.as_reference_element().is_some()
    }

    /// Returns a reference to the inner [`Blob`] if applicable.
    pub fn as_blob(&self) -> Option<&Blob> {
        if let Class::Blob(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Blob`] if applicable.
    pub fn as_blob_mut(&mut self) -> Option<&mut Blob> {
        if let Class::Blob(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Blob`.
    pub fn is_blob(&self) -> bool {
        self.as_blob().is_some()
    }

    /// Returns a reference to the inner [`File`] if applicable.
    pub fn as_file(&self) -> Option<&File> {
        if let Class::File(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`File`] if applicable.
    pub fn as_file_mut(&mut self) -> Option<&mut File> {
        if let Class::File(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::File`.
    pub fn is_file(&self) -> bool {
        self.as_file().is_some()
    }

    /// Returns a reference to the inner [`AnnotatedRelationshipElement`] if applicable.
    pub fn as_annotated_relationship_element(&self) -> Option<&AnnotatedRelationshipElement> {
        if let Class::AnnotatedRelationshipElement(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`AnnotatedRelationshipElement`] if applicable.
    pub fn as_annotated_relationship_element_mut(
        &mut self,
    ) -> Option<&mut AnnotatedRelationshipElement> {
        if let Class::AnnotatedRelationshipElement(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::AnnotatedRelationshipElement`.
    pub fn is_annotated_relationship_element(&self) -> bool {
        self.as_annotated_relationship_element().is_some()
    }

    /// Returns a reference to the inner [`Entity`] if applicable.
    pub fn as_entity(&self) -> Option<&Entity> {
        if let Class::Entity(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Entity`] if applicable.
    pub fn as_entity_mut(&mut self) -> Option<&mut Entity> {
        if let Class::Entity(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Entity`.
    pub fn is_entity(&self) -> bool {
        self.as_entity().is_some()
    }

    /// Returns a reference to the inner [`EventPayload`] if applicable.
    pub fn as_event_payload(&self) -> Option<&EventPayload> {
        if let Class::EventPayload(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`EventPayload`] if applicable.
    pub fn as_event_payload_mut(&mut self) -> Option<&mut EventPayload> {
        if let Class::EventPayload(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::EventPayload`.
    pub fn is_event_payload(&self) -> bool {
        self.as_event_payload().is_some()
    }

    /// Returns a reference to the inner [`BasicEventElement`] if applicable.
    pub fn as_basic_event_element(&self) -> Option<&BasicEventElement> {
        if let Class::BasicEventElement(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`BasicEventElement`] if applicable.
    pub fn as_basic_event_element_mut(&mut self) -> Option<&mut BasicEventElement> {
        if let Class::BasicEventElement(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::BasicEventElement`.
    pub fn is_basic_event_element(&self) -> bool {
        self.as_basic_event_element().is_some()
    }

    /// Returns a reference to the inner [`Operation`] if applicable.
    pub fn as_operation(&self) -> Option<&Operation> {
        if let Class::Operation(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Operation`] if applicable.
    pub fn as_operation_mut(&mut self) -> Option<&mut Operation> {
        if let Class::Operation(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Operation`.
    pub fn is_operation(&self) -> bool {
        self.as_operation().is_some()
    }

    /// Returns a reference to the inner [`OperationVariable`] if applicable.
    pub fn as_operation_variable(&self) -> Option<&OperationVariable> {
        if let Class::OperationVariable(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`OperationVariable`] if applicable.
    pub fn as_operation_variable_mut(&mut self) -> Option<&mut OperationVariable> {
        if let Class::OperationVariable(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::OperationVariable`.
    pub fn is_operation_variable(&self) -> bool {
        self.as_operation_variable().is_some()
    }

    /// Returns a reference to the inner [`Capability`] if applicable.
    pub fn as_capability(&self) -> Option<&Capability> {
        if let Class::Capability(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Capability`] if applicable.
    pub fn as_capability_mut(&mut self) -> Option<&mut Capability> {
        if let Class::Capability(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Capability`.
    pub fn is_capability(&self) -> bool {
        self.as_capability().is_some()
    }

    /// Returns a reference to the inner [`ConceptDescription`] if applicable.
    pub fn as_concept_description(&self) -> Option<&ConceptDescription> {
        if let Class::ConceptDescription(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`ConceptDescription`] if applicable.
    pub fn as_concept_description_mut(&mut self) -> Option<&mut ConceptDescription> {
        if let Class::ConceptDescription(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::ConceptDescription`.
    pub fn is_concept_description(&self) -> bool {
        self.as_concept_description().is_some()
    }

    /// Returns a reference to the inner [`Reference`] if applicable.
    pub fn as_reference(&self) -> Option<&Reference> {
        if let Class::Reference(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Reference`] if applicable.
    pub fn as_reference_mut(&mut self) -> Option<&mut Reference> {
        if let Class::Reference(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Reference`.
    pub fn is_reference(&self) -> bool {
        self.as_reference().is_some()
    }

    /// Returns a reference to the inner [`Key`] if applicable.
    pub fn as_key(&self) -> Option<&Key> {
        if let Class::Key(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Key`] if applicable.
    pub fn as_key_mut(&mut self) -> Option<&mut Key> {
        if let Class::Key(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Key`.
    pub fn is_key(&self) -> bool {
        self.as_key().is_some()
    }

    /// Returns a reference to the inner [`LangStringNameType`] if applicable.
    pub fn as_lang_string_name_type(&self) -> Option<&LangStringNameType> {
        if let Class::LangStringNameType(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`LangStringNameType`] if applicable.
    pub fn as_lang_string_name_type_mut(&mut self) -> Option<&mut LangStringNameType> {
        if let Class::LangStringNameType(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::LangStringNameType`.
    pub fn is_lang_string_name_type(&self) -> bool {
        self.as_lang_string_name_type().is_some()
    }

    /// Returns a reference to the inner [`LangStringTextType`] if applicable.
    pub fn as_lang_string_text_type(&self) -> Option<&LangStringTextType> {
        if let Class::LangStringTextType(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`LangStringTextType`] if applicable.
    pub fn as_lang_string_text_type_mut(&mut self) -> Option<&mut LangStringTextType> {
        if let Class::LangStringTextType(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::LangStringTextType`.
    pub fn is_lang_string_text_type(&self) -> bool {
        self.as_lang_string_text_type().is_some()
    }

    /// Returns a reference to the inner [`Environment`] if applicable.
    pub fn as_environment(&self) -> Option<&Environment> {
        if let Class::Environment(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`Environment`] if applicable.
    pub fn as_environment_mut(&mut self) -> Option<&mut Environment> {
        if let Class::Environment(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::Environment`.
    pub fn is_environment(&self) -> bool {
        self.as_environment().is_some()
    }

    /// Returns a reference to the inner [`EmbeddedDataSpecification`] if applicable.
    pub fn as_embedded_data_specification(&self) -> Option<&EmbeddedDataSpecification> {
        if let Class::EmbeddedDataSpecification(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`EmbeddedDataSpecification`] if applicable.
    pub fn as_embedded_data_specification_mut(&mut self) -> Option<&mut EmbeddedDataSpecification> {
        if let Class::EmbeddedDataSpecification(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::EmbeddedDataSpecification`.
    pub fn is_embedded_data_specification(&self) -> bool {
        self.as_embedded_data_specification().is_some()
    }

    /// Returns a reference to the inner [`LevelType`] if applicable.
    pub fn as_level_type(&self) -> Option<&LevelType> {
        if let Class::LevelType(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`LevelType`] if applicable.
    pub fn as_level_type_mut(&mut self) -> Option<&mut LevelType> {
        if let Class::LevelType(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::LevelType`.
    pub fn is_level_type(&self) -> bool {
        self.as_level_type().is_some()
    }

    /// Returns a reference to the inner [`ValueReferencePair`] if applicable.
    pub fn as_value_reference_pair(&self) -> Option<&ValueReferencePair> {
        if let Class::ValueReferencePair(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`ValueReferencePair`] if applicable.
    pub fn as_value_reference_pair_mut(&mut self) -> Option<&mut ValueReferencePair> {
        if let Class::ValueReferencePair(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::ValueReferencePair`.
    pub fn is_value_reference_pair(&self) -> bool {
        self.as_value_reference_pair().is_some()
    }

    /// Returns a reference to the inner [`ValueList`] if applicable.
    pub fn as_value_list(&self) -> Option<&ValueList> {
        if let Class::ValueList(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`ValueList`] if applicable.
    pub fn as_value_list_mut(&mut self) -> Option<&mut ValueList> {
        if let Class::ValueList(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::ValueList`.
    pub fn is_value_list(&self) -> bool {
        self.as_value_list().is_some()
    }

    /// Returns a reference to the inner [`LangStringPreferredNameTypeIec61360`] if applicable.
    pub fn as_lang_string_preferred_name_type_iec61360(
        &self,
    ) -> Option<&LangStringPreferredNameTypeIec61360> {
        if let Class::LangStringPreferredNameTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the inner [`LangStringPreferredNameTypeIec61360`] if applicable.
    pub fn as_lang_string_preferred_name_type_iec61360_mut(
        &mut self,
    ) -> Option<&mut LangStringPreferredNameTypeIec61360> {
        if let Class::LangStringPreferredNameTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns `true` if this is `Class::LangStringPreferredNameTypeIec61360`.
    pub fn is_lang_string_preferred_name_type_iec61360(&self) -> bool {
        self.as_lang_string_preferred_name_type_iec61360().is_some()
    }

    /// Returns a reference to the inner [`LangStringShortNameTypeIec61360`] if applicable.
    pub fn as_lang_string_short_name_type_iec61360(
        &self,
    ) -> Option<&LangStringShortNameTypeIec61360> {
        if let Class::LangStringShortNameTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the inner [`LangStringShortNameTypeIec61360`] if applicable.
    pub fn as_lang_string_short_name_type_iec61360_mut(
        &mut self,
    ) -> Option<&mut LangStringShortNameTypeIec61360> {
        if let Class::LangStringShortNameTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns `true` if this is `Class::LangStringShortNameTypeIec61360`.
    pub fn is_lang_string_short_name_type_iec61360(&self) -> bool {
        self.as_lang_string_short_name_type_iec61360().is_some()
    }

    /// Returns a reference to the inner [`LangStringDefinitionTypeIec61360`] if applicable.
    pub fn as_lang_string_definition_type_iec61360(
        &self,
    ) -> Option<&LangStringDefinitionTypeIec61360> {
        if let Class::LangStringDefinitionTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the inner [`LangStringDefinitionTypeIec61360`] if applicable.
    pub fn as_lang_string_definition_type_iec61360_mut(
        &mut self,
    ) -> Option<&mut LangStringDefinitionTypeIec61360> {
        if let Class::LangStringDefinitionTypeIec61360(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Returns `true` if this is `Class::LangStringDefinitionTypeIec61360`.
    pub fn is_lang_string_definition_type_iec61360(&self) -> bool {
        self.as_lang_string_definition_type_iec61360().is_some()
    }

    /// Returns a reference to the inner [`DataSpecificationIec61360`] if applicable.
    pub fn as_data_specification_iec61360(&self) -> Option<&DataSpecificationIec61360> {
        if let Class::DataSpecificationIec61360(x) = self { Some(x) } else { None }
    }

    /// Returns a mutable reference to the inner [`DataSpecificationIec61360`] if applicable.
    pub fn as_data_specification_iec61360_mut(&mut self) -> Option<&mut DataSpecificationIec61360> {
        if let Class::DataSpecificationIec61360(x) = self { Some(x) } else { None }
    }

    /// Returns `true` if this is `Class::DataSpecificationIec61360`.
    pub fn is_data_specification_iec61360(&self) -> bool {
        self.as_data_specification_iec61360().is_some()
    }

    /// Returns `true` if this class implements the `IReferable` interface.
    pub fn is_referable(&self) -> bool {
        matches!(
            self,
            Class::AssetAdministrationShell(_)
                | Class::Submodel(_)
                | Class::ConceptDescription(_)
                | Class::RelationshipElement(_)
                | Class::SubmodelElementList(_)
                | Class::SubmodelElementCollection(_)
                | Class::Property(_)
                | Class::MultiLanguageProperty(_)
                | Class::Range(_)
                | Class::ReferenceElement(_)
                | Class::Blob(_)
                | Class::File(_)
                | Class::AnnotatedRelationshipElement(_)
                | Class::Entity(_)
                | Class::BasicEventElement(_)
                | Class::Operation(_)
                | Class::Capability(_)
        )
    }

    /// Returns `true` if this class implements the `IIdentifiable` interface.
    pub fn is_identifiable(&self) -> bool {
        matches!(
            self,
            Class::AssetAdministrationShell(_)
                | Class::Submodel(_)
                | Class::ConceptDescription(_)
        )
    }

    /// Returns `true` if this class implements the `ISubmodelElement` interface.
    pub fn is_submodel_element(&self) -> bool {
        matches!(
            self,
            Class::RelationshipElement(_)
                | Class::SubmodelElementList(_)
                | Class::SubmodelElementCollection(_)
                | Class::Property(_)
                | Class::MultiLanguageProperty(_)
                | Class::Range(_)
                | Class::ReferenceElement(_)
                | Class::Blob(_)
                | Class::File(_)
                | Class::AnnotatedRelationshipElement(_)
                | Class::Entity(_)
                | Class::BasicEventElement(_)
                | Class::Operation(_)
                | Class::Capability(_)
        )
    }

    /// Returns `true` if this class implements the `IHasSemantics` interface.
    pub fn is_has_semantics(&self) -> bool {
        matches!(
            self,
            Class::Extension(_)
                | Class::Qualifier(_)
                | Class::SpecificAssetId(_)
                | Class::AssetAdministrationShell(_)
                | Class::Submodel(_)
                | Class::RelationshipElement(_)
                | Class::SubmodelElementList(_)
                | Class::SubmodelElementCollection(_)
                | Class::Property(_)
                | Class::MultiLanguageProperty(_)
                | Class::Range(_)
                | Class::ReferenceElement(_)
                | Class::Blob(_)
                | Class::File(_)
                | Class::AnnotatedRelationshipElement(_)
                | Class::Entity(_)
                | Class::BasicEventElement(_)
                | Class::Operation(_)
                | Class::Capability(_)
        )
    }
}
