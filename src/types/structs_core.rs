use super::class::Class;
use super::enums::{AssetKind, ModellingKind};
use super::structs_elements::{AdministrativeInformation, EmbeddedDataSpecification};
use super::structs_lang::{LangStringNameType, LangStringTextType};
use super::structs_support::{Extension, Qualifier, Reference, Resource, SpecificAssetId};

/// The environment root element that groups shells, submodels, and concept descriptions.
#[derive(Debug, Clone)]
pub struct Environment {
    pub asset_administration_shells: Option<Vec<AssetAdministrationShell>>,
    pub submodels: Option<Vec<Submodel>>,
    pub concept_descriptions: Option<Vec<ConceptDescription>>,
}

impl Environment {
    /// Creates a new [`Environment`] with all optional fields unset.
    pub fn new() -> Self {
        Self {
            asset_administration_shells: None,
            submodels: None,
            concept_descriptions: None,
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// An Asset Administration Shell.
#[derive(Debug, Clone)]
pub struct AssetAdministrationShell {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub administration: Option<AdministrativeInformation>,
    pub id: String,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub derived_from: Option<Reference>,
    pub asset_information: AssetInformation,
    pub submodels: Option<Vec<Reference>>,
}

impl AssetAdministrationShell {
    /// Creates a new [`AssetAdministrationShell`] with the given ID and asset information.
    pub fn new(id: String, asset_information: AssetInformation) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            administration: None,
            id,
            embedded_data_specifications: None,
            derived_from: None,
            asset_information,
            submodels: None,
        }
    }
}

/// Asset information for an Asset Administration Shell.
#[derive(Debug, Clone)]
pub struct AssetInformation {
    pub asset_kind: AssetKind,
    pub global_asset_id: Option<String>,
    pub specific_asset_ids: Option<Vec<SpecificAssetId>>,
    pub asset_type: Option<String>,
    pub default_thumbnail: Option<Resource>,
}

impl AssetInformation {
    /// Creates a new [`AssetInformation`] with the given asset kind.
    pub fn new(asset_kind: AssetKind) -> Self {
        Self {
            asset_kind,
            global_asset_id: None,
            specific_asset_ids: None,
            asset_type: None,
            default_thumbnail: None,
        }
    }
}

/// A submodel.
#[derive(Debug, Clone)]
pub struct Submodel {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub administration: Option<AdministrativeInformation>,
    pub id: String,
    pub kind: Option<ModellingKind>,
    pub semantic_id: Option<Reference>,
    pub supplemental_semantic_ids: Option<Vec<Reference>>,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub submodel_elements: Option<Vec<Class>>,
}

impl Submodel {
    /// Creates a new [`Submodel`] with the given ID.
    pub fn new(id: String) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            administration: None,
            id,
            kind: None,
            semantic_id: None,
            supplemental_semantic_ids: None,
            qualifiers: None,
            embedded_data_specifications: None,
            submodel_elements: None,
        }
    }
}

/// A concept description.
#[derive(Debug, Clone)]
pub struct ConceptDescription {
    pub extensions: Option<Vec<Extension>>,
    pub category: Option<String>,
    pub id_short: Option<String>,
    pub display_name: Option<Vec<LangStringNameType>>,
    pub description: Option<Vec<LangStringTextType>>,
    pub administration: Option<AdministrativeInformation>,
    pub id: String,
    pub embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
    pub is_case_of: Option<Vec<Reference>>,
}

impl ConceptDescription {
    /// Creates a new [`ConceptDescription`] with the given ID.
    pub fn new(id: String) -> Self {
        Self {
            extensions: None,
            category: None,
            id_short: None,
            display_name: None,
            description: None,
            administration: None,
            id,
            embedded_data_specifications: None,
            is_case_of: None,
        }
    }
}
