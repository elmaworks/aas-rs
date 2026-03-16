use super::enums::{DataTypeDefXsd, KeyTypes, QualifierKind, ReferenceTypes};

/// A reference to an element.
#[derive(Debug, Clone)]
pub struct Reference {
    pub type_: ReferenceTypes,
    pub referred_semantic_id: Option<Box<Reference>>,
    pub keys: Vec<Key>,
}

impl Reference {
    /// Creates a new [`Reference`] with the given type and keys.
    pub fn new(type_: ReferenceTypes, keys: Vec<Key>) -> Self {
        Self {
            type_,
            referred_semantic_id: None,
            keys,
        }
    }
}

/// A key in a reference.
#[derive(Debug, Clone)]
pub struct Key {
    pub type_: KeyTypes,
    pub value: String,
}

impl Key {
    /// Creates a new [`Key`] with the given type and value.
    pub fn new(type_: KeyTypes, value: String) -> Self {
        Self { type_, value }
    }
}

/// An extension of an element.
#[derive(Debug, Clone)]
pub struct Extension {
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub name: String,
    pub value_type: Option<DataTypeDefXsd>,
    pub value: Option<String>,
    pub refers_to: Option<Vec<Reference>>,
}

impl Extension {
    /// Creates a new [`Extension`] with the given name.
    pub fn new(name: String) -> Self {
        Self {
            semantic_id: None,
            supplemental_semantic_ids: None,
            name,
            value_type: None,
            value: None,
            refers_to: None,
        }
    }
}

/// A qualifier of an element.
#[derive(Debug, Clone)]
pub struct Qualifier {
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub kind: Option<QualifierKind>,
    pub type_: String,
    pub value_type: DataTypeDefXsd,
    pub value: Option<String>,
    pub value_id: Option<Reference>,
}

impl Qualifier {
    /// Creates a new [`Qualifier`] with the given type and value type.
    pub fn new(type_: String, value_type: DataTypeDefXsd) -> Self {
        Self {
            semantic_id: None,
            supplemental_semantic_ids: None,
            kind: None,
            type_,
            value_type,
            value: None,
            value_id: None,
        }
    }
}

/// A resource (e.g., a thumbnail image).
#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
    pub content_type: Option<String>,
}

impl Resource {
    /// Creates a new [`Resource`] with the given path.
    pub fn new(path: String) -> Self {
        Self {
            path,
            content_type: None,
        }
    }
}

/// A specific asset ID.
#[derive(Debug, Clone)]
pub struct SpecificAssetId {
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub name: String,
    pub value: String,
    pub external_subject_id: Option<Reference>,
}

impl SpecificAssetId {
    /// Creates a new [`SpecificAssetId`] with the given name and value.
    pub fn new(name: String, value: String) -> Self {
        Self {
            semantic_id: None,
            supplemental_semantic_ids: None,
            name,
            value,
            external_subject_id: None,
        }
    }
}

/// Represents a level type with min, nom, typ, max flags.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct LevelType {
    pub min: bool,
    pub nom: bool,
    pub typ: bool,
    pub max: bool,
}

impl LevelType {
    /// Creates a new [`LevelType`] with the given flags.
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn new(min: bool, nom: bool, typ: bool, max: bool) -> Self {
        Self { min, nom, typ, max }
    }
}

/// A pair of a value and its reference.
#[derive(Debug, Clone)]
pub struct ValueReferencePair {
    pub value: String,
    pub value_id: Reference,
}

impl ValueReferencePair {
    /// Creates a new [`ValueReferencePair`] with the given value and value ID.
    pub fn new(value: String, value_id: Reference) -> Self {
        Self { value, value_id }
    }
}

/// A list of value reference pairs.
#[derive(Debug, Clone)]
pub struct ValueList {
    pub value_reference_pairs: Vec<ValueReferencePair>,
}

impl ValueList {
    /// Creates a new [`ValueList`] with the given value reference pairs.
    pub fn new(value_reference_pairs: Vec<ValueReferencePair>) -> Self {
        Self { value_reference_pairs }
    }
}

/// An event payload.
#[derive(Debug, Clone)]
pub struct EventPayload {
    pub source: Reference,
    pub source_semantic_id: Option<Reference>,
    pub observable_reference: Reference,
    pub observable_semantic_id: Option<Reference>,
    pub topic: Option<String>,
    pub subject_id: Option<Reference>,
    pub time_stamp: String,
    pub payload: Option<Vec<u8>>,
}

impl EventPayload {
    /// Creates a new [`EventPayload`] with the given source, observable reference, and timestamp.
    pub fn new(source: Reference, observable_reference: Reference, time_stamp: String) -> Self {
        Self {
            source,
            source_semantic_id: None,
            observable_reference,
            observable_semantic_id: None,
            topic: None,
            subject_id: None,
            time_stamp,
            payload: None,
        }
    }
}

/// A variable in an operation.
#[derive(Debug, Clone)]
pub struct OperationVariable {
    pub value: Box<super::class::Class>,
}

impl OperationVariable {
    /// Creates a new [`OperationVariable`] with the given value.
    pub fn new(value: Box<super::class::Class>) -> Self {
        Self { value }
    }
}
