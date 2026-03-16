use super::class::Class;
use super::enums::{AasSubmodelElements, DataTypeDefXsd, Direction, EntityType, StateOfEvent};
use super::structs_lang::{LangStringNameType, LangStringTextType};
use super::structs_support::{
    Extension, OperationVariable, Qualifier, Reference, SpecificAssetId,
};

/// An embedded data specification.
#[derive(Debug, Clone)]
pub struct EmbeddedDataSpecification {
    pub data_specification: Reference,
    pub data_specification_content: Box<Class>,
}

impl EmbeddedDataSpecification {
    /// Creates a new [`EmbeddedDataSpecification`] with the given reference and content.
    pub fn new(data_specification: Reference, data_specification_content: Box<Class>) -> Self {
        Self {
            data_specification,
            data_specification_content,
        }
    }
}

/// Administrative information of an identifiable element.
#[derive(Debug, Clone)]
pub struct AdministrativeInformation {
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub creator: Option<Reference>,
    pub template_id: Option<String>,
}

impl AdministrativeInformation {
    /// Creates a new [`AdministrativeInformation`] with all optional fields unset.
    pub fn new() -> Self {
        Self {
            embedded_data_specifications: None,
            version: None,
            revision: None,
            creator: None,
            template_id: None,
        }
    }
}

impl Default for AdministrativeInformation {
    fn default() -> Self {
        Self::new()
    }
}

/// A property submodel element.
#[derive(Debug, Clone)]
pub struct Property {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value_type: DataTypeDefXsd,
    pub value: Option<String>,
    pub value_id: Option<Reference>,
}

impl Property {
    /// Creates a new [`Property`] with the given value type.
    pub fn new(value_type: DataTypeDefXsd) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value_type,
            value: None,
            value_id: None,
        }
    }
}

/// A multi-language property submodel element.
#[derive(Debug, Clone)]
pub struct MultiLanguageProperty {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value: Option<Vec<LangStringTextType>>,
    pub value_id: Option<Reference>,
}

impl MultiLanguageProperty {
    /// Creates a new [`MultiLanguageProperty`].
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
            value_id: None,
        }
    }
}

impl Default for MultiLanguageProperty {
    fn default() -> Self {
        Self::new()
    }
}

/// A range submodel element.
#[derive(Debug, Clone)]
pub struct Range {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value_type: DataTypeDefXsd,
    pub min: Option<String>,
    pub max: Option<String>,
}

impl Range {
    /// Creates a new [`Range`] with the given value type.
    pub fn new(value_type: DataTypeDefXsd) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value_type,
            min: None,
            max: None,
        }
    }
}

/// A reference element submodel element.
#[derive(Debug, Clone)]
pub struct ReferenceElement {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value: Option<Reference>,
}

impl ReferenceElement {
    /// Creates a new [`ReferenceElement`].
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
        }
    }
}

impl Default for ReferenceElement {
    fn default() -> Self {
        Self::new()
    }
}

/// A blob submodel element.
#[derive(Debug, Clone)]
pub struct Blob {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value: Option<Vec<u8>>,
    pub content_type: String,
}

impl Blob {
    /// Creates a new [`Blob`] with the given content type.
    pub fn new(content_type: String) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
            content_type,
        }
    }
}

/// A file submodel element.
#[derive(Debug, Clone)]
pub struct File {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value: Option<String>,
    pub content_type: String,
}

impl File {
    /// Creates a new [`File`] with the given content type.
    pub fn new(content_type: String) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
            content_type,
        }
    }
}

/// A submodel element list.
#[derive(Debug, Clone)]
pub struct SubmodelElementList {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub order_relevant: Option<bool>,
    pub semantic_id_list_element: Option<Reference>,
    pub type_value_list_element: AasSubmodelElements,
    pub value_type_list_element: Option<DataTypeDefXsd>,
    pub value: Option<Vec<Class>>,
}

impl SubmodelElementList {
    /// Creates a new [`SubmodelElementList`] with the given type value list element.
    pub fn new(type_value_list_element: AasSubmodelElements) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            order_relevant: None,
            semantic_id_list_element: None,
            type_value_list_element,
            value_type_list_element: None,
            value: None,
        }
    }
}

/// A submodel element collection.
#[derive(Debug, Clone)]
pub struct SubmodelElementCollection {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub value: Option<Vec<Class>>,
}

impl SubmodelElementCollection {
    /// Creates a new [`SubmodelElementCollection`].
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            value: None,
        }
    }
}

impl Default for SubmodelElementCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// A relationship element.
#[derive(Debug, Clone)]
pub struct RelationshipElement {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub first: Reference,
    pub second: Reference,
}

impl RelationshipElement {
    /// Creates a new [`RelationshipElement`] with the given first and second references.
    pub fn new(first: Reference, second: Reference) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            first,
            second,
        }
    }
}

/// An annotated relationship element.
#[derive(Debug, Clone)]
pub struct AnnotatedRelationshipElement {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub first: Reference,
    pub second: Reference,
    pub annotations: Option<Vec<Class>>,
}

impl AnnotatedRelationshipElement {
    /// Creates a new [`AnnotatedRelationshipElement`] with the given first and second references.
    pub fn new(first: Reference, second: Reference) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            first,
            second,
            annotations: None,
        }
    }
}

/// An entity submodel element.
#[derive(Debug, Clone)]
pub struct Entity {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub statements: Option<Vec<Class>>,
    pub entity_type: EntityType,
    pub global_asset_id: Option<String>,
    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,
}

impl Entity {
    /// Creates a new [`Entity`] with the given entity type.
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            statements: None,
            entity_type,
            global_asset_id: None,
            specific_asset_ids: None,
        }
    }
}

/// A basic event element.
#[derive(Debug, Clone)]
pub struct BasicEventElement {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub observed: Reference,
    pub direction: Direction,
    pub state: StateOfEvent,
    pub message_topic: Option<String>,
    pub message_broker: Option<Reference>,
    pub last_update: Option<String>,
    pub min_interval: Option<String>,
    pub max_interval: Option<String>,
}

impl BasicEventElement {
    /// Creates a new [`BasicEventElement`] with the given observed reference, direction, and state.
    pub fn new(observed: Reference, direction: Direction, state: StateOfEvent) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            observed,
            direction,
            state,
            message_topic: None,
            message_broker: None,
            last_update: None,
            min_interval: None,
            max_interval: None,
        }
    }
}

/// An operation submodel element.
#[derive(Debug, Clone)]
pub struct Operation {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub input_variables: Option<Vec<OperationVariable>>,
    pub output_variables: Option<Vec<OperationVariable>>,
    pub inoutput_variables: Option<Vec<OperationVariable>>,
}

impl Operation {
    /// Creates a new [`Operation`].
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            input_variables: None,
            output_variables: None,
            inoutput_variables: None,
        }
    }
}

impl Default for Operation {
    fn default() -> Self {
        Self::new()
    }
}

/// A capability submodel element.
#[derive(Debug, Clone)]
pub struct Capability {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
}

impl Capability {
    /// Creates a new [`Capability`].
    pub fn new() -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
        }
    }
}

impl Default for Capability {
    fn default() -> Self {
        Self::new()
    }
}
