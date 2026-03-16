//! Verifier functions for all AAS model classes.

use crate::types::class::Class;
use crate::Reference;
use crate::{
    AnnotatedRelationshipElement, AssetAdministrationShell, AssetInformation, BasicEventElement,
    Blob, Capability, ConceptDescription, DataSpecificationIec61360, DataTypeDefXsd, Direction,
    EmbeddedDataSpecification, Entity, EntityType, Environment, EventPayload, Extension, File, Key,
    KeyTypes, LangStringDefinitionTypeIec61360, LangStringNameType,
    LangStringPreferredNameTypeIec61360, LangStringShortNameTypeIec61360, LangStringTextType,
    LevelType, MultiLanguageProperty, Operation, OperationVariable, Property, Qualifier, Range,
    ReferenceElement, ReferenceTypes, RelationshipElement, Resource, SpecificAssetId, Submodel,
    SubmodelElementCollection, SubmodelElementList, ValueList, ValueReferencePair,
};

use super::error::VerificationError;
use super::rules::{
    data_specification_iec61360s_for_document_have_appropriate_data_type,
    data_specification_iec61360s_for_property_or_value_have_appropriate_data_type,
    data_specification_iec61360s_for_reference_have_appropriate_data_type,
    data_specification_iec61360s_have_data_type,
    data_specification_iec61360s_have_definition_at_least_in_english,
    data_specification_iec61360s_have_value, extension_names_are_unique, id_shorts_are_unique,
    id_shorts_of_variables_are_unique, is_model_reference_to, is_model_reference_to_referable,
    lang_strings_have_unique_languages, properties_or_ranges_have_value_type,
    qualifier_types_are_unique, reference_key_values_equal, reference_keys_valid,
    submodel_element_is_of_type, submodel_elements_have_identical_semantic_ids,
};
use super::string_constraints::{
    verify_bcp47_language_tag, verify_blob_type, verify_content_type, verify_date_time_utc,
    verify_duration, verify_id_short_type, verify_identifier, verify_label_type,
    verify_message_topic_type, verify_name_type, verify_non_empty_xml_serializable_string,
    verify_path_type, verify_qualifier_type, verify_revision_type, verify_value_data_type,
    verify_value_type_iec61360, verify_version_type,
};
use super::value::value_consistent_with_xsd_type;

use super::pattern::is_bcp47_for_english;
use crate::constants::VALID_CATEGORIES_FOR_DATA_ELEMENT;

// ---- Helper macros ----------------------------------------------------------

macro_rules! push_prop {
    ($errors:expr, $source:expr, $prop:literal) => {
        for mut err in $source {
            err.prepend_property($prop);
            $errors.push(err);
        }
    };
}

// ---- Shared SME field verification helpers ----------------------------------

fn verify_sme_constraints(
    extensions: &Option<Vec<Extension>>,
    description: &Option<Vec<LangStringTextType>>,
    display_name: &Option<Vec<LangStringNameType>>,
    supplemental_semantic_ids: &Option<Vec<Reference>>,
    semantic_id: &Option<Reference>,
    qualifiers: &Option<Vec<Qualifier>>,
    embedded_data_specifications: &Option<Vec<EmbeddedDataSpecification>>,
    errors: &mut Vec<VerificationError>,
) {
    if extensions.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Extensions must be either not set or have at least one item.",
        ));
    }
    if extensions
        .as_ref()
        .map_or(false, |v| !extension_names_are_unique(v))
    {
        errors.push(VerificationError::new(
            "Constraint AASd-077: The name of an extension within Has-Extensions needs to be unique.",
        ));
    }
    if description.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Description must be either not set or have at least one item.",
        ));
    }
    if description
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Description must specify unique languages.",
        ));
    }
    if display_name.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Display name must be either not set or have at least one item.",
        ));
    }
    if display_name
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Display name must specify unique languages.",
        ));
    }
    if supplemental_semantic_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Supplemental semantic IDs must be either not set or have at least one item.",
        ));
    }
    if supplemental_semantic_ids.is_some() && semantic_id.is_none() {
        errors.push(VerificationError::new(
            "Constraint AASd-118: If there are supplemental semantic IDs defined then there shall be also a main semantic ID.",
        ));
    }
    if qualifiers.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Qualifiers must be either not set or have at least one item.",
        ));
    }
    if qualifiers
        .as_ref()
        .map_or(false, |v| !qualifier_types_are_unique(v))
    {
        errors.push(VerificationError::new(
            "Constraint AASd-021: Every qualifiable can only have one qualifier with the same type.",
        ));
    }
    if embedded_data_specifications
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Embedded data specifications must be either not set or have at least one item.",
        ));
    }
}

fn verify_sme_string_fields(
    category: &Option<String>,
    id_short: &Option<String>,
    errors: &mut Vec<VerificationError>,
) {
    if let Some(cat) = category {
        for mut err in verify_name_type(cat) {
            err.prepend_property("category");
            errors.push(err);
        }
    }
    if let Some(id) = id_short {
        for mut err in verify_id_short_type(id) {
            err.prepend_property("idShort");
            errors.push(err);
        }
    }
}

fn recurse_sme_common(
    extensions: &Option<Vec<Extension>>,
    display_name: &Option<Vec<LangStringNameType>>,
    description: &Option<Vec<LangStringTextType>>,
    semantic_id: &Option<Reference>,
    supplemental_semantic_ids: &Option<Vec<Reference>>,
    qualifiers: &Option<Vec<Qualifier>>,
    embedded_data_specifications: &Option<Vec<EmbeddedDataSpecification>>,
    recurse: bool,
    errors: &mut Vec<VerificationError>,
) {
    if !recurse {
        return;
    }
    if let Some(exts) = extensions {
        for (idx, item) in exts.iter().enumerate() {
            for mut err in verify_extension(item, recurse) {
                err.prepend_index(idx);
                err.prepend_property("extensions");
                errors.push(err);
            }
        }
    }
    if let Some(names) = display_name {
        for (idx, item) in names.iter().enumerate() {
            for mut err in verify_lang_string_name_type(item) {
                err.prepend_index(idx);
                err.prepend_property("displayName");
                errors.push(err);
            }
        }
    }
    if let Some(descs) = description {
        for (idx, item) in descs.iter().enumerate() {
            for mut err in verify_lang_string_text_type(item) {
                err.prepend_index(idx);
                err.prepend_property("description");
                errors.push(err);
            }
        }
    }
    if let Some(sid) = semantic_id {
        push_prop!(errors, verify_reference(sid, recurse), "semanticId");
    }
    if let Some(ssids) = supplemental_semantic_ids {
        for (idx, item) in ssids.iter().enumerate() {
            for mut err in verify_reference(item, recurse) {
                err.prepend_index(idx);
                err.prepend_property("supplementalSemanticIds");
                errors.push(err);
            }
        }
    }
    if let Some(quals) = qualifiers {
        for (idx, item) in quals.iter().enumerate() {
            for mut err in verify_qualifier(item, recurse) {
                err.prepend_index(idx);
                err.prepend_property("qualifiers");
                errors.push(err);
            }
        }
    }
    if let Some(eds) = embedded_data_specifications {
        for (idx, item) in eds.iter().enumerate() {
            for mut err in verify_embedded_data_specification(item, recurse) {
                err.prepend_index(idx);
                err.prepend_property("embeddedDataSpecifications");
                errors.push(err);
            }
        }
    }
}

// ---- Public verify entry point ----------------------------------------------

/// Verify an AAS model class instance and return all errors found.
pub fn verify_class(that: &Class, recurse: bool) -> Vec<VerificationError> {
    match that {
        Class::Extension(x) => verify_extension(x, recurse),
        Class::AdministrativeInformation(x) => verify_administrative_information(x, recurse),
        Class::Qualifier(x) => verify_qualifier(x, recurse),
        Class::AssetAdministrationShell(x) => verify_asset_administration_shell(x, recurse),
        Class::AssetInformation(x) => verify_asset_information(x, recurse),
        Class::Resource(x) => verify_resource(x),
        Class::SpecificAssetId(x) => verify_specific_asset_id(x, recurse),
        Class::Submodel(x) => verify_submodel(x, recurse),
        Class::RelationshipElement(x) => verify_relationship_element(x, recurse),
        Class::SubmodelElementList(x) => verify_submodel_element_list(x, recurse),
        Class::SubmodelElementCollection(x) => verify_submodel_element_collection(x, recurse),
        Class::Property(x) => verify_property(x, recurse),
        Class::MultiLanguageProperty(x) => verify_multi_language_property(x, recurse),
        Class::Range(x) => verify_range(x, recurse),
        Class::ReferenceElement(x) => verify_reference_element(x, recurse),
        Class::Blob(x) => verify_blob(x, recurse),
        Class::File(x) => verify_file(x, recurse),
        Class::AnnotatedRelationshipElement(x) => verify_annotated_relationship_element(x, recurse),
        Class::Entity(x) => verify_entity(x, recurse),
        Class::EventPayload(x) => verify_event_payload(x, recurse),
        Class::BasicEventElement(x) => verify_basic_event_element(x, recurse),
        Class::Operation(x) => verify_operation(x, recurse),
        Class::OperationVariable(x) => verify_operation_variable(x, recurse),
        Class::Capability(x) => verify_capability(x, recurse),
        Class::ConceptDescription(x) => verify_concept_description(x, recurse),
        Class::Reference(x) => verify_reference(x, recurse),
        Class::Key(x) => verify_key(x),
        Class::LangStringNameType(x) => verify_lang_string_name_type(x),
        Class::LangStringTextType(x) => verify_lang_string_text_type(x),
        Class::Environment(x) => verify_environment(x, recurse),
        Class::EmbeddedDataSpecification(x) => verify_embedded_data_specification(x, recurse),
        Class::LevelType(x) => verify_level_type(x),
        Class::ValueReferencePair(x) => verify_value_reference_pair(x, recurse),
        Class::ValueList(x) => verify_value_list(x, recurse),
        Class::LangStringPreferredNameTypeIec61360(x) => {
            verify_lang_string_preferred_name_type_iec61360(x)
        }
        Class::LangStringShortNameTypeIec61360(x) => verify_lang_string_short_name_type_iec61360(x),
        Class::LangStringDefinitionTypeIec61360(x) => {
            verify_lang_string_definition_type_iec61360(x)
        }
        Class::DataSpecificationIec61360(x) => verify_data_specification_iec61360(x, recurse),
    }
}

// ---- Individual class verification functions --------------------------------

fn verify_extension(that: &Extension, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .supplemental_semantic_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Supplemental semantic IDs must be either not set or have at least one item.",
        ));
    }
    if that.supplemental_semantic_ids.is_some() && that.semantic_id.is_none() {
        errors.push(VerificationError::new(
            "Constraint AASd-118: If there are supplemental semantic IDs defined then there shall be also a main semantic ID.",
        ));
    }
    if that
        .refers_to
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Refers-to must be either not set or have at least one item.",
        ));
    }
    if let (Some(value), value_type) = (
        &that.value,
        that.value_type.unwrap_or(DataTypeDefXsd::String),
    ) {
        if !value_consistent_with_xsd_type(value, value_type) {
            errors.push(VerificationError::new(
                "The value must match the value type.",
            ));
        }
    }

    push_prop!(errors, verify_name_type(&that.name), "name");
    if let Some(v) = &that.value {
        push_prop!(errors, verify_value_data_type(v), "value");
    }

    if recurse {
        if let Some(sid) = &that.semantic_id {
            push_prop!(errors, verify_reference(sid, recurse), "semanticId");
        }
        if let Some(ssids) = &that.supplemental_semantic_ids {
            for (idx, item) in ssids.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("supplementalSemanticIds");
                    errors.push(err);
                }
            }
        }
        if let Some(refs) = &that.refers_to {
            for (idx, item) in refs.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("refersTo");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_administrative_information(
    that: &crate::AdministrativeInformation,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .embedded_data_specifications
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Embedded data specifications must be either not set or have at least one item.",
        ));
    }
    if that.revision.is_some() && that.version.is_none() {
        errors.push(VerificationError::new(
            "Constraint AASd-005: If version is not specified then also revision shall be unspecified. This means, a revision requires a version. If there is no version there is no revision either. Revision is optional.",
        ));
    }

    if let Some(v) = &that.version {
        push_prop!(errors, verify_version_type(v), "version");
    }
    if let Some(r) = &that.revision {
        push_prop!(errors, verify_revision_type(r), "revision");
    }
    if let Some(t) = &that.template_id {
        push_prop!(errors, verify_identifier(t), "templateId");
    }

    if recurse {
        if let Some(eds) = &that.embedded_data_specifications {
            for (idx, item) in eds.iter().enumerate() {
                for mut err in verify_embedded_data_specification(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("embeddedDataSpecifications");
                    errors.push(err);
                }
            }
        }
        if let Some(c) = &that.creator {
            push_prop!(errors, verify_reference(c, recurse), "creator");
        }
    }
    errors
}

fn verify_qualifier(that: &Qualifier, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .supplemental_semantic_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Supplemental semantic IDs must be either not set or have at least one item.",
        ));
    }
    if that.supplemental_semantic_ids.is_some() && that.semantic_id.is_none() {
        errors.push(VerificationError::new(
            "Constraint AASd-118: If there are supplemental semantic IDs defined then there shall be also a main semantic ID.",
        ));
    }
    if let Some(v) = &that.value {
        if !value_consistent_with_xsd_type(v, that.value_type) {
            errors.push(VerificationError::new(
                "Constraint AASd-020: The value shall be consistent to the data type as defined in value type.",
            ));
        }
    }

    push_prop!(errors, verify_qualifier_type(&that.type_), "type");
    if let Some(v) = &that.value {
        push_prop!(errors, verify_value_data_type(v), "value");
    }

    if recurse {
        if let Some(sid) = &that.semantic_id {
            push_prop!(errors, verify_reference(sid, recurse), "semanticId");
        }
        if let Some(ssids) = &that.supplemental_semantic_ids {
            for (idx, item) in ssids.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("supplementalSemanticIds");
                    errors.push(err);
                }
            }
        }
        if let Some(vid) = &that.value_id {
            push_prop!(errors, verify_reference(vid, recurse), "valueId");
        }
    }
    errors
}

fn verify_asset_administration_shell(
    that: &AssetAdministrationShell,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .extensions
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Extensions must be either not set or have at least one item.",
        ));
    }
    if that
        .extensions
        .as_ref()
        .map_or(false, |v| !extension_names_are_unique(v))
    {
        errors.push(VerificationError::new(
            "Constraint AASd-077: The name of an extension within Has-Extensions needs to be unique.",
        ));
    }
    if that
        .description
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Description must be either not set or have at least one item.",
        ));
    }
    if that
        .description
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Description must specify unique languages.",
        ));
    }
    if that
        .display_name
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Display name must be either not set or have at least one item.",
        ));
    }
    if that
        .display_name
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Display name must specify unique languages.",
        ));
    }
    if that
        .embedded_data_specifications
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Embedded data specifications must be either not set or have at least one item.",
        ));
    }
    if that
        .submodels
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Submodels must be either not set or have at least one item.",
        ));
    }
    if let Some(df) = &that.derived_from {
        if !is_model_reference_to(df, KeyTypes::AssetAdministrationShell) {
            errors.push(VerificationError::new(
                "Derived-from must be a model reference to an asset administration shell.",
            ));
        }
    }
    if let Some(sms) = &that.submodels {
        if !sms
            .iter()
            .all(|r| is_model_reference_to(r, KeyTypes::Submodel))
        {
            errors.push(VerificationError::new(
                "All submodels must be model references to a submodel.",
            ));
        }
    }

    if let Some(cat) = &that.category {
        push_prop!(errors, verify_name_type(cat), "category");
    }
    if let Some(id_short) = &that.id_short {
        push_prop!(errors, verify_id_short_type(id_short), "idShort");
    }
    push_prop!(errors, verify_identifier(&that.id), "id");

    if recurse {
        if let Some(exts) = &that.extensions {
            for (idx, item) in exts.iter().enumerate() {
                for mut err in verify_extension(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("extensions");
                    errors.push(err);
                }
            }
        }
        if let Some(dn) = &that.display_name {
            for (idx, item) in dn.iter().enumerate() {
                for mut err in verify_lang_string_name_type(item) {
                    err.prepend_index(idx);
                    err.prepend_property("displayName");
                    errors.push(err);
                }
            }
        }
        if let Some(desc) = &that.description {
            for (idx, item) in desc.iter().enumerate() {
                for mut err in verify_lang_string_text_type(item) {
                    err.prepend_index(idx);
                    err.prepend_property("description");
                    errors.push(err);
                }
            }
        }
        if let Some(adm) = &that.administration {
            push_prop!(
                errors,
                verify_administrative_information(adm, recurse),
                "administration"
            );
        }
        if let Some(eds) = &that.embedded_data_specifications {
            for (idx, item) in eds.iter().enumerate() {
                for mut err in verify_embedded_data_specification(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("embeddedDataSpecifications");
                    errors.push(err);
                }
            }
        }
        if let Some(df) = &that.derived_from {
            push_prop!(errors, verify_reference(df, recurse), "derivedFrom");
        }
        push_prop!(
            errors,
            verify_asset_information(&that.asset_information, recurse),
            "assetInformation"
        );
        if let Some(sms) = &that.submodels {
            for (idx, item) in sms.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("submodels");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_asset_information(that: &AssetInformation, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    // AASd-116: globalAssetId reserved key
    if let Some(specific_ids) = &that.specific_asset_ids {
        let valid = specific_ids.iter().all(|sid| {
            sid.name != "globalAssetId"
                || (that.global_asset_id.is_some()
                    && sid.name == "globalAssetId"
                    && Some(&sid.value) == that.global_asset_id.as_ref())
        });
        if !valid {
            errors.push(VerificationError::new(
                "Constraint AASd-116: ``globalAssetId`` is a reserved key. If used as value for the name of specific asset ID then the value of specific asset ID shall be identical to the global asset ID.",
            ));
        }
    }

    // AASd-131: either globalAssetId or specificAssetIds
    let has_global = that.global_asset_id.is_some();
    let has_specific = that
        .specific_asset_ids
        .as_ref()
        .map_or(false, |v| !v.is_empty());
    if !has_global && !has_specific {
        errors.push(VerificationError::new(
            "Constraint AASd-131: Either the global asset ID shall be defined or at least one specific asset ID.",
        ));
    }

    if that
        .specific_asset_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Specific asset IDs must be either not set or have at least one item.",
        ));
    }

    if let Some(gid) = &that.global_asset_id {
        push_prop!(errors, verify_identifier(gid), "globalAssetId");
    }
    if let Some(at) = &that.asset_type {
        push_prop!(errors, verify_identifier(at), "assetType");
    }

    if recurse {
        if let Some(sids) = &that.specific_asset_ids {
            for (idx, item) in sids.iter().enumerate() {
                for mut err in verify_specific_asset_id(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("specificAssetIds");
                    errors.push(err);
                }
            }
        }
        if let Some(thumb) = &that.default_thumbnail {
            push_prop!(errors, verify_resource(thumb), "defaultThumbnail");
        }
    }
    errors
}

fn verify_resource(that: &Resource) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    push_prop!(errors, verify_path_type(&that.path), "path");
    if let Some(ct) = &that.content_type {
        push_prop!(errors, verify_content_type(ct), "contentType");
    }
    errors
}

fn verify_specific_asset_id(that: &SpecificAssetId, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .supplemental_semantic_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Supplemental semantic IDs must be either not set or have at least one item.",
        ));
    }
    if that.supplemental_semantic_ids.is_some() && that.semantic_id.is_none() {
        errors.push(VerificationError::new(
            "Constraint AASd-118: If there are supplemental semantic IDs defined then there shall be also a main semantic ID.",
        ));
    }
    if let Some(ext_id) = &that.external_subject_id {
        if ext_id.type_ != ReferenceTypes::ExternalReference {
            errors.push(VerificationError::new(
                "Constraint AASd-133: External subject ID shall be an external reference.",
            ));
        }
    }

    push_prop!(errors, verify_label_type(&that.name), "name");
    push_prop!(errors, verify_identifier(&that.value), "value");

    if recurse {
        if let Some(sid) = &that.semantic_id {
            push_prop!(errors, verify_reference(sid, recurse), "semanticId");
        }
        if let Some(ssids) = &that.supplemental_semantic_ids {
            for (idx, item) in ssids.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("supplementalSemanticIds");
                    errors.push(err);
                }
            }
        }
        if let Some(ext) = &that.external_subject_id {
            push_prop!(errors, verify_reference(ext, recurse), "externalSubjectId");
        }
    }
    errors
}

fn verify_submodel(that: &Submodel, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if that
        .submodel_elements
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Submodel elements must be either not set or have at least one item.",
        ));
    }
    if let Some(smes) = &that.submodel_elements {
        if !smes.iter().all(|e| get_id_short_class(e).is_some()) {
            errors.push(VerificationError::new(
                "ID-shorts need to be defined for all the items of submodel elements according to AASd-117.",
            ));
        }
        if !id_shorts_are_unique(smes) {
            errors.push(VerificationError::new(
                "ID-shorts of the submodel elements must be unique.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    push_prop!(errors, verify_identifier(&that.id), "id");

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(adm) = &that.administration {
            push_prop!(
                errors,
                verify_administrative_information(adm, recurse),
                "administration"
            );
        }
        if let Some(smes) = &that.submodel_elements {
            for (idx, item) in smes.iter().enumerate() {
                for mut err in verify_class(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("submodelElements");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn get_id_short_class(cls: &Class) -> Option<&str> {
    match cls {
        Class::RelationshipElement(x) => x.id_short.as_deref(),
        Class::SubmodelElementList(x) => x.id_short.as_deref(),
        Class::SubmodelElementCollection(x) => x.id_short.as_deref(),
        Class::Property(x) => x.id_short.as_deref(),
        Class::MultiLanguageProperty(x) => x.id_short.as_deref(),
        Class::Range(x) => x.id_short.as_deref(),
        Class::ReferenceElement(x) => x.id_short.as_deref(),
        Class::Blob(x) => x.id_short.as_deref(),
        Class::File(x) => x.id_short.as_deref(),
        Class::AnnotatedRelationshipElement(x) => x.id_short.as_deref(),
        Class::Entity(x) => x.id_short.as_deref(),
        Class::BasicEventElement(x) => x.id_short.as_deref(),
        Class::Operation(x) => x.id_short.as_deref(),
        Class::Capability(x) => x.id_short.as_deref(),
        _ => None,
    }
}

fn verify_relationship_element(
    that: &RelationshipElement,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        push_prop!(errors, verify_reference(&that.first, recurse), "first");
        push_prop!(errors, verify_reference(&that.second, recurse), "second");
    }
    errors
}

fn verify_submodel_element_list(
    that: &SubmodelElementList,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if that.value.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Value must be either not set or have at least one item.",
        ));
    }

    // AASd-107
    if let (Some(vals), Some(sem_list_elem)) = (&that.value, &that.semantic_id_list_element) {
        let all_ok = vals.iter().all(|child| {
            get_semantic_id_class(child)
                .map_or(true, |sid| reference_key_values_equal(sid, sem_list_elem))
        });
        if !all_ok {
            errors.push(VerificationError::new(
                "Constraint AASd-107: If a first level child element has a semantic ID it shall be identical to semantic ID list element.",
            ));
        }
    }

    // AASd-114
    if let Some(vals) = &that.value {
        if !submodel_elements_have_identical_semantic_ids(vals) {
            errors.push(VerificationError::new(
                "Constraint AASd-114: If two first level child elements have a semantic ID then they shall be identical.",
            ));
        }
    }

    // AASd-108
    if let Some(vals) = &that.value {
        if !vals
            .iter()
            .all(|e| submodel_element_is_of_type(e, that.type_value_list_element))
        {
            errors.push(VerificationError::new(
                "Constraint AASd-108: All first level child elements shall have the same submodel element type as specified in type value list element.",
            ));
        }
    }

    // AASd-109
    let is_property_or_range = that.type_value_list_element == crate::AasSubmodelElements::Property
        || that.type_value_list_element == crate::AasSubmodelElements::Range;
    if is_property_or_range {
        let ok = match (that.value_type_list_element, &that.value) {
            (None, _) => false,
            (Some(vt), Some(vals)) => properties_or_ranges_have_value_type(vals, vt),
            (Some(_), None) => true,
        };
        if !ok {
            errors.push(VerificationError::new(
                "Constraint AASd-109: If type value list element is equal to Property or Range value type list element shall be set and all first level child elements shall have the value type as specified in value type list element.",
            ));
        }
    }

    // AASd-120: children must not have idShort
    if let Some(vals) = &that.value {
        if !vals.iter().all(|e| get_id_short_class(e).is_none()) {
            errors.push(VerificationError::new(
                "Constraint AASd-120: ID-short of submodel elements being a direct child of a  Submodel element list shall not be specified.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(sem_id) = &that.semantic_id_list_element {
            push_prop!(
                errors,
                verify_reference(sem_id, recurse),
                "semanticIdListElement"
            );
        }
        if let Some(vals) = &that.value {
            for (idx, item) in vals.iter().enumerate() {
                for mut err in verify_class(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("value");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn get_semantic_id_class(cls: &Class) -> Option<&Reference> {
    match cls {
        Class::RelationshipElement(x) => x.semantic_id.as_ref(),
        Class::SubmodelElementList(x) => x.semantic_id.as_ref(),
        Class::SubmodelElementCollection(x) => x.semantic_id.as_ref(),
        Class::Property(x) => x.semantic_id.as_ref(),
        Class::MultiLanguageProperty(x) => x.semantic_id.as_ref(),
        Class::Range(x) => x.semantic_id.as_ref(),
        Class::ReferenceElement(x) => x.semantic_id.as_ref(),
        Class::Blob(x) => x.semantic_id.as_ref(),
        Class::File(x) => x.semantic_id.as_ref(),
        Class::AnnotatedRelationshipElement(x) => x.semantic_id.as_ref(),
        Class::Entity(x) => x.semantic_id.as_ref(),
        Class::BasicEventElement(x) => x.semantic_id.as_ref(),
        Class::Operation(x) => x.semantic_id.as_ref(),
        Class::Capability(x) => x.semantic_id.as_ref(),
        _ => None,
    }
}

fn verify_submodel_element_collection(
    that: &SubmodelElementCollection,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if that.value.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Value must be either not set or have at least one item.",
        ));
    }
    if let Some(vals) = &that.value {
        if !vals.iter().all(|e| get_id_short_class(e).is_some()) {
            errors.push(VerificationError::new(
                "ID-shorts need to be defined for all the items of value according to AASd-117.",
            ));
        }
        if !id_shorts_are_unique(vals) {
            errors.push(VerificationError::new(
                "ID-shorts of the value must be unique.",
            ));
        }
    }
    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(vals) = &that.value {
            for (idx, item) in vals.iter().enumerate() {
                for mut err in verify_class(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("value");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_property(that: &Property, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    // AASd-090: category must be in valid categories for data element
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }
    if let Some(v) = &that.value {
        if !value_consistent_with_xsd_type(v, that.value_type) {
            errors.push(VerificationError::new(
                "Value must be consistent with the value type.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    if let Some(v) = &that.value {
        push_prop!(errors, verify_value_data_type(v), "value");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(vid) = &that.value_id {
            push_prop!(errors, verify_reference(vid, recurse), "valueId");
        }
    }
    errors
}

fn verify_multi_language_property(
    that: &MultiLanguageProperty,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }
    if that
        .value
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Value must specify unique languages.",
        ));
    }
    if that.value.as_ref().map_or(false, std::vec::Vec::is_empty) {
        errors.push(VerificationError::new(
            "Value must be either not set or have at least one item.",
        ));
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(vals) = &that.value {
            for (idx, item) in vals.iter().enumerate() {
                for mut err in verify_lang_string_text_type(item) {
                    err.prepend_index(idx);
                    err.prepend_property("value");
                    errors.push(err);
                }
            }
        }
        if let Some(vid) = &that.value_id {
            push_prop!(errors, verify_reference(vid, recurse), "valueId");
        }
    }
    errors
}

fn verify_range(that: &Range, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }
    if let Some(max) = &that.max {
        if !value_consistent_with_xsd_type(max, that.value_type) {
            errors.push(VerificationError::new(
                "Max must be consistent with the value type.",
            ));
        }
    }
    if let Some(min) = &that.min {
        if !value_consistent_with_xsd_type(min, that.value_type) {
            errors.push(VerificationError::new(
                "Min must be consistent with the value type.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    if let Some(min) = &that.min {
        push_prop!(errors, verify_value_data_type(min), "min");
    }
    if let Some(max) = &that.max {
        push_prop!(errors, verify_value_data_type(max), "max");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
    }
    errors
}

fn verify_reference_element(that: &ReferenceElement, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }
    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(v) = &that.value {
            push_prop!(errors, verify_reference(v, recurse), "value");
        }
    }
    errors
}

fn verify_blob(that: &Blob, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    push_prop!(
        errors,
        verify_content_type(&that.content_type),
        "contentType"
    );
    if let Some(v) = &that.value {
        push_prop!(errors, verify_blob_type(v), "value");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
    }
    errors
}

fn verify_file(that: &File, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if let Some(cat) = &that.category {
        if !VALID_CATEGORIES_FOR_DATA_ELEMENT.contains(cat.as_str()) {
            errors.push(VerificationError::new(
                "Constraint AASd-090: For data elements category shall be one of the following values: CONSTANT, PARAMETER or VARIABLE.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    push_prop!(
        errors,
        verify_content_type(&that.content_type),
        "contentType"
    );
    if let Some(v) = &that.value {
        push_prop!(errors, verify_path_type(v), "value");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
    }
    errors
}

fn verify_annotated_relationship_element(
    that: &AnnotatedRelationshipElement,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if that
        .annotations
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Annotations must be either not set or have at least one item.",
        ));
    }
    if let Some(anns) = &that.annotations {
        if !anns.iter().all(|e| get_id_short_class(e).is_some()) {
            errors.push(VerificationError::new(
                "ID-shorts need to be defined for all the items of annotations according to AASd-117.",
            ));
        }
    }
    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        push_prop!(errors, verify_reference(&that.first, recurse), "first");
        push_prop!(errors, verify_reference(&that.second, recurse), "second");
        if let Some(anns) = &that.annotations {
            for (idx, item) in anns.iter().enumerate() {
                for mut err in verify_class(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("annotations");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_entity(that: &Entity, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    if that
        .statements
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Statements must be either not set or have at least one item.",
        ));
    }
    if let Some(stmts) = &that.statements {
        if !stmts.iter().all(|e| get_id_short_class(e).is_some()) {
            errors.push(VerificationError::new(
                "ID-shorts need to be defined for all the items of statements according to AASd-117.",
            ));
        }
    }

    // AASd-014
    let is_self_managed = that.entity_type == EntityType::SelfManagedEntity;
    let has_global = that.global_asset_id.is_some();
    let has_specific = that
        .specific_asset_ids
        .as_ref()
        .map_or(false, |v| !v.is_empty());
    let constraint_ok = if is_self_managed {
        (has_global && !has_specific) || (!has_global && has_specific)
    } else {
        !has_global && !has_specific
    };
    if !constraint_ok {
        errors.push(VerificationError::new(
            "Constraint AASd-014: Either the attribute global asset ID or specific asset ID must be set if entity type is set to self-managed entity. They are not existing otherwise.",
        ));
    }
    if that
        .specific_asset_ids
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Specific asset IDs must be either not set or have at least one item.",
        ));
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    if let Some(gid) = &that.global_asset_id {
        push_prop!(errors, verify_identifier(gid), "globalAssetId");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(stmts) = &that.statements {
            for (idx, item) in stmts.iter().enumerate() {
                for mut err in verify_class(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("statements");
                    errors.push(err);
                }
            }
        }
        if let Some(sids) = &that.specific_asset_ids {
            for (idx, item) in sids.iter().enumerate() {
                for mut err in verify_specific_asset_id(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("specificAssetIds");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_event_payload(that: &EventPayload, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    let source_ok = is_model_reference_to(&that.source, KeyTypes::EventElement)
        || is_model_reference_to(&that.source, KeyTypes::BasicEventElement);
    if !source_ok {
        errors.push(VerificationError::new(
            "Source must be a model reference to an Event element.",
        ));
    }
    if !is_model_reference_to_referable(&that.observable_reference) {
        errors.push(VerificationError::new(
            "Observable reference must be a model reference to a referable.",
        ));
    }

    if let Some(topic) = &that.topic {
        push_prop!(errors, verify_message_topic_type(topic), "topic");
    }
    push_prop!(errors, verify_date_time_utc(&that.time_stamp), "timeStamp");
    if let Some(payload) = &that.payload {
        push_prop!(errors, verify_blob_type(payload), "payload");
    }

    if recurse {
        push_prop!(errors, verify_reference(&that.source, recurse), "source");
        if let Some(sid) = &that.source_semantic_id {
            push_prop!(errors, verify_reference(sid, recurse), "sourceSemanticId");
        }
        push_prop!(
            errors,
            verify_reference(&that.observable_reference, recurse),
            "observableReference"
        );
        if let Some(oid) = &that.observable_semantic_id {
            push_prop!(
                errors,
                verify_reference(oid, recurse),
                "observableSemanticId"
            );
        }
        if let Some(subj) = &that.subject_id {
            push_prop!(errors, verify_reference(subj, recurse), "subjectId");
        }
    }
    errors
}

fn verify_basic_event_element(that: &BasicEventElement, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );

    if that.direction == Direction::Input && that.max_interval.is_some() {
        errors.push(VerificationError::new(
            "Max. interval is not applicable for input direction.",
        ));
    }
    if !is_model_reference_to_referable(&that.observed) {
        errors.push(VerificationError::new(
            "Observed must be a model reference to a referable.",
        ));
    }
    if let Some(mb) = &that.message_broker {
        if !is_model_reference_to_referable(mb) {
            errors.push(VerificationError::new(
                "Message broker must be a model reference to a referable.",
            ));
        }
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);
    if let Some(mt) = &that.message_topic {
        push_prop!(errors, verify_message_topic_type(mt), "messageTopic");
    }
    if let Some(lu) = &that.last_update {
        push_prop!(errors, verify_date_time_utc(lu), "lastUpdate");
    }
    if let Some(mi) = &that.min_interval {
        push_prop!(errors, verify_duration(mi), "minInterval");
    }
    if let Some(ma) = &that.max_interval {
        push_prop!(errors, verify_duration(ma), "maxInterval");
    }

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        push_prop!(
            errors,
            verify_reference(&that.observed, recurse),
            "observed"
        );
        if let Some(mb) = &that.message_broker {
            push_prop!(errors, verify_reference(mb, recurse), "messageBroker");
        }
    }
    errors
}

fn verify_operation(that: &Operation, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );

    if !id_shorts_of_variables_are_unique(
        that.input_variables.as_deref(),
        that.output_variables.as_deref(),
        that.inoutput_variables.as_deref(),
    ) {
        errors.push(VerificationError::new(
            "Constraint AASd-134: For an Operation the ID-short of all values of input, output and in/output variables shall be unique.",
        ));
    }
    if that
        .input_variables
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Input variables must be either not set or have at least one item.",
        ));
    }
    if that
        .output_variables
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Output variables must be either not set or have at least one item.",
        ));
    }
    if that
        .inoutput_variables
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Inoutput variables must be either not set or have at least one item.",
        ));
    }

    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
        if let Some(ivs) = &that.input_variables {
            for (idx, item) in ivs.iter().enumerate() {
                for mut err in verify_operation_variable(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("inputVariables");
                    errors.push(err);
                }
            }
        }
        if let Some(ovs) = &that.output_variables {
            for (idx, item) in ovs.iter().enumerate() {
                for mut err in verify_operation_variable(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("outputVariables");
                    errors.push(err);
                }
            }
        }
        if let Some(iovs) = &that.inoutput_variables {
            for (idx, item) in iovs.iter().enumerate() {
                for mut err in verify_operation_variable(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("inoutputVariables");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_operation_variable(that: &OperationVariable, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if get_id_short_class(&that.value).is_none() {
        errors.push(VerificationError::new(
            "Value must have the ID-short specified according to Constraint AASd-117.",
        ));
    }

    if recurse {
        push_prop!(errors, verify_class(&that.value, recurse), "value");
    }
    errors
}

fn verify_capability(that: &Capability, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    verify_sme_constraints(
        &that.extensions,
        &that.description,
        &that.display_name,
        &that.supplemental_semantic_ids,
        &that.semantic_id,
        &that.qualifiers,
        &that.embedded_data_specifications,
        &mut errors,
    );
    verify_sme_string_fields(&that.category, &that.id_short, &mut errors);

    if recurse {
        recurse_sme_common(
            &that.extensions,
            &that.display_name,
            &that.description,
            &that.semantic_id,
            &that.supplemental_semantic_ids,
            &that.qualifiers,
            &that.embedded_data_specifications,
            recurse,
            &mut errors,
        );
    }
    errors
}

fn verify_concept_description(that: &ConceptDescription, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .extensions
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Extensions must be either not set or have at least one item.",
        ));
    }
    if that
        .extensions
        .as_ref()
        .map_or(false, |v| !extension_names_are_unique(v))
    {
        errors.push(VerificationError::new(
            "Constraint AASd-077: The name of an extension within Has-Extensions needs to be unique.",
        ));
    }
    if that
        .description
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Description must be either not set or have at least one item.",
        ));
    }
    if that
        .description
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Description must specify unique languages.",
        ));
    }
    if that
        .display_name
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Display name must be either not set or have at least one item.",
        ));
    }
    if that
        .display_name
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Display name must specify unique languages.",
        ));
    }
    if that
        .embedded_data_specifications
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Embedded data specifications must be either not set or have at least one item.",
        ));
    }
    if that
        .is_case_of
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Is-case-of must be either not set or have at least one item.",
        ));
    }

    // AASc-3a-008
    if let Some(eds) = &that.embedded_data_specifications {
        if !data_specification_iec61360s_have_definition_at_least_in_english(eds)
            && !data_specification_iec61360s_have_value(eds)
        {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-008: For a concept description using data specification template IEC 61360, the definition is mandatory and shall be defined at least in English. Exception: The concept description describes a value.",
            ));
        }
    }

    // AASc-3a-007
    if let (Some(cat), Some(eds)) = (&that.category, &that.embedded_data_specifications) {
        if cat == "QUALIFIER_TYPE" && !data_specification_iec61360s_have_data_type(eds) {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-007: For a concept description with category QUALIFIER_TYPE using data specification IEC 61360, the data type of the data specification is mandatory and shall be defined.",
            ));
        }
        if cat == "DOCUMENT"
            && !data_specification_iec61360s_for_document_have_appropriate_data_type(eds)
        {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-006: For a concept description with category DOCUMENT using data specification IEC 61360, the data type of the data specification shall be one of: FILE, BLOB, HTML.",
            ));
        }
        if cat == "REFERENCE"
            && !data_specification_iec61360s_for_reference_have_appropriate_data_type(eds)
        {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-005: For a concept description with category REFERENCE using data specification IEC 61360, the data type of the data specification shall be one of: STRING, IRI, IRDI.",
            ));
        }
        if (cat == "PROPERTY" || cat == "VALUE")
            && !data_specification_iec61360s_for_property_or_value_have_appropriate_data_type(eds)
        {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-004: For a concept description with category PROPERTY or VALUE using data specification IEC 61360, the data type of the data specification is mandatory and shall be one of: DATE, STRING, STRING_TRANSLATABLE, INTEGER_MEASURE, INTEGER_COUNT, INTEGER_CURRENCY, REAL_MEASURE, REAL_COUNT, REAL_CURRENCY, BOOLEAN, RATIONAL, RATIONAL_MEASURE, TIME, TIMESTAMP.",
            ));
        }
    }

    if let Some(cat) = &that.category {
        push_prop!(errors, verify_name_type(cat), "category");
    }
    if let Some(id_short) = &that.id_short {
        push_prop!(errors, verify_id_short_type(id_short), "idShort");
    }
    push_prop!(errors, verify_identifier(&that.id), "id");

    if recurse {
        if let Some(exts) = &that.extensions {
            for (idx, item) in exts.iter().enumerate() {
                for mut err in verify_extension(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("extensions");
                    errors.push(err);
                }
            }
        }
        if let Some(dn) = &that.display_name {
            for (idx, item) in dn.iter().enumerate() {
                for mut err in verify_lang_string_name_type(item) {
                    err.prepend_index(idx);
                    err.prepend_property("displayName");
                    errors.push(err);
                }
            }
        }
        if let Some(desc) = &that.description {
            for (idx, item) in desc.iter().enumerate() {
                for mut err in verify_lang_string_text_type(item) {
                    err.prepend_index(idx);
                    err.prepend_property("description");
                    errors.push(err);
                }
            }
        }
        if let Some(adm) = &that.administration {
            push_prop!(
                errors,
                verify_administrative_information(adm, recurse),
                "administration"
            );
        }
        if let Some(eds) = &that.embedded_data_specifications {
            for (idx, item) in eds.iter().enumerate() {
                for mut err in verify_embedded_data_specification(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("embeddedDataSpecifications");
                    errors.push(err);
                }
            }
        }
        if let Some(ico) = &that.is_case_of {
            for (idx, item) in ico.iter().enumerate() {
                for mut err in verify_reference(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("isCaseOf");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_reference(that: &Reference, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that.keys.is_empty() {
        errors.push(VerificationError::new(
            "Keys must contain at least one item.",
        ));
    }

    let (c121, c122, c123, c124, c125, c126, c127, c128) = reference_keys_valid(that);
    if !c121 {
        errors.push(VerificationError::new(
            "Constraint AASd-121: For References the value of type of the first key of keys shall be one of Globally Identifiables.",
        ));
    }
    if !c122 {
        errors.push(VerificationError::new(
            "Constraint AASd-122: For external references the value of type of the first key of keys shall be one of Generic Globally Identifiables.",
        ));
    }
    if !c123 {
        errors.push(VerificationError::new(
            "Constraint AASd-123: For model references the value of type of the first key of keys shall be one of AAS identifiables.",
        ));
    }
    if !c124 {
        errors.push(VerificationError::new(
            "Constraint AASd-124: For external references the last key of keys shall be either one of Generic Globally Identifiables or one of Generic Fragment Keys.",
        ));
    }
    if !c125 {
        errors.push(VerificationError::new(
            "Constraint AASd-125: For model references with more than one key in keys the value of type of each of the keys following the first key of keys shall be one of Fragment Keys.",
        ));
    }
    if !c126 {
        errors.push(VerificationError::new(
            "Constraint AASd-126: For model references with more than one key in keys the value of type of the last key in the reference key chain may be one of Generic Fragment Keys or no key at all shall have a value out of Generic Fragment Keys.",
        ));
    }
    if !c127 {
        errors.push(VerificationError::new(
            "Constraint AASd-127: For model references, with more than one key in keys a key with type Fragment Reference shall be preceded by a key with type File or Blob.",
        ));
    }
    if !c128 {
        errors.push(VerificationError::new(
            "Constraint AASd-128: For model references, the value of a key preceded by a key with type Submodel element list is an integer number denoting the position in the array of the submodel element list.",
        ));
    }

    if recurse {
        for (idx, key) in that.keys.iter().enumerate() {
            for mut err in verify_key(key) {
                err.prepend_index(idx);
                err.prepend_property("keys");
                errors.push(err);
            }
        }
        if let Some(rsid) = &that.referred_semantic_id {
            push_prop!(
                errors,
                verify_reference(rsid, recurse),
                "referredSemanticId"
            );
        }
    }
    errors
}

fn verify_key(that: &Key) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    push_prop!(errors, verify_identifier(&that.value), "value");
    errors
}

fn verify_lang_string_name_type(that: &LangStringNameType) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.text.chars().count() > 128 {
        errors.push(VerificationError::new(
            "String shall have a maximum length of 128 characters.",
        ));
    }
    push_prop!(
        errors,
        verify_bcp47_language_tag(&that.language),
        "language"
    );
    push_prop!(
        errors,
        verify_non_empty_xml_serializable_string(&that.text),
        "text"
    );
    errors
}

fn verify_lang_string_text_type(that: &LangStringTextType) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.text.chars().count() > 1023 {
        errors.push(VerificationError::new(
            "String shall have a maximum length of 1023 characters.",
        ));
    }
    push_prop!(
        errors,
        verify_bcp47_language_tag(&that.language),
        "language"
    );
    push_prop!(
        errors,
        verify_non_empty_xml_serializable_string(&that.text),
        "text"
    );
    errors
}

fn verify_environment(that: &Environment, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if that
        .concept_descriptions
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Concept descriptions must be either not set or have at least one item.",
        ));
    }
    if that
        .submodels
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Submodels must be either not set or have at least one item.",
        ));
    }
    if that
        .asset_administration_shells
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Asset administration shells must be either not set or have at least one item.",
        ));
    }

    if recurse {
        if let Some(aas_list) = &that.asset_administration_shells {
            for (idx, item) in aas_list.iter().enumerate() {
                for mut err in verify_asset_administration_shell(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("assetAdministrationShells");
                    errors.push(err);
                }
            }
        }
        if let Some(sms) = &that.submodels {
            for (idx, item) in sms.iter().enumerate() {
                for mut err in verify_submodel(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("submodels");
                    errors.push(err);
                }
            }
        }
        if let Some(cds) = &that.concept_descriptions {
            for (idx, item) in cds.iter().enumerate() {
                for mut err in verify_concept_description(item, recurse) {
                    err.prepend_index(idx);
                    err.prepend_property("conceptDescriptions");
                    errors.push(err);
                }
            }
        }
    }
    errors
}

fn verify_embedded_data_specification(
    that: &EmbeddedDataSpecification,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    if recurse {
        push_prop!(
            errors,
            verify_reference(&that.data_specification, recurse),
            "dataSpecification"
        );
        push_prop!(
            errors,
            verify_class(&that.data_specification_content, recurse),
            "dataSpecificationContent"
        );
    }
    errors
}

fn verify_level_type(_that: &LevelType) -> Vec<VerificationError> {
    Vec::new()
}

fn verify_value_reference_pair(that: &ValueReferencePair, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    push_prop!(errors, verify_value_type_iec61360(&that.value), "value");
    if recurse {
        push_prop!(errors, verify_reference(&that.value_id, recurse), "valueId");
    }
    errors
}

fn verify_value_list(that: &ValueList, recurse: bool) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.value_reference_pairs.is_empty() {
        errors.push(VerificationError::new(
            "Value reference pair types must contain at least one item.",
        ));
    }
    if recurse {
        for (idx, item) in that.value_reference_pairs.iter().enumerate() {
            for mut err in verify_value_reference_pair(item, recurse) {
                err.prepend_index(idx);
                err.prepend_property("valueReferencePairs");
                errors.push(err);
            }
        }
    }
    errors
}

fn verify_lang_string_preferred_name_type_iec61360(
    that: &LangStringPreferredNameTypeIec61360,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.text.chars().count() > 255 {
        errors.push(VerificationError::new(
            "String shall have a maximum length of 255 characters.",
        ));
    }
    push_prop!(
        errors,
        verify_bcp47_language_tag(&that.language),
        "language"
    );
    push_prop!(
        errors,
        verify_non_empty_xml_serializable_string(&that.text),
        "text"
    );
    errors
}

fn verify_lang_string_short_name_type_iec61360(
    that: &LangStringShortNameTypeIec61360,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.text.chars().count() > 18 {
        errors.push(VerificationError::new(
            "String shall have a maximum length of 18 characters.",
        ));
    }
    push_prop!(
        errors,
        verify_bcp47_language_tag(&that.language),
        "language"
    );
    push_prop!(
        errors,
        verify_non_empty_xml_serializable_string(&that.text),
        "text"
    );
    errors
}

fn verify_lang_string_definition_type_iec61360(
    that: &LangStringDefinitionTypeIec61360,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if that.text.chars().count() > 1023 {
        errors.push(VerificationError::new(
            "String shall have a maximum length of 1023 characters.",
        ));
    }
    push_prop!(
        errors,
        verify_bcp47_language_tag(&that.language),
        "language"
    );
    push_prop!(
        errors,
        verify_non_empty_xml_serializable_string(&that.text),
        "text"
    );
    errors
}

fn verify_data_specification_iec61360(
    that: &DataSpecificationIec61360,
    recurse: bool,
) -> Vec<VerificationError> {
    let mut errors = Vec::new();

    // AASc-3a-010: value and valueList cannot both be set
    if that.value.is_some() && that.value_list.is_some() {
        errors.push(VerificationError::new(
            "Constraint AASc-3a-010: If value is not empty then value list shall be empty and vice versa.",
        ));
    }

    // AASc-3a-009: data types with unit must have unit or unit_id
    use crate::constants::IEC_61360_DATA_TYPES_WITH_UNIT;
    if let Some(dt) = that.data_type {
        if IEC_61360_DATA_TYPES_WITH_UNIT.contains(&dt)
            && that.unit.is_none()
            && that.unit_id.is_none()
        {
            errors.push(VerificationError::new(
                "Constraint AASc-3a-009: If data type is a an integer, real or rational with a measure or currency, unit or unit ID shall be defined.",
            ));
        }
    }

    if that
        .definition
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Definition must be either not set or have at least one item.",
        ));
    }
    if that
        .definition
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Definition must specify unique languages.",
        ));
    }
    if that
        .short_name
        .as_ref()
        .map_or(false, std::vec::Vec::is_empty)
    {
        errors.push(VerificationError::new(
            "Short name must be either not set or have at least one item.",
        ));
    }
    if that
        .short_name
        .as_ref()
        .map_or(false, |v| !lang_strings_have_unique_languages(v))
    {
        errors.push(VerificationError::new(
            "Short name must specify unique languages.",
        ));
    }
    if that.preferred_name.is_empty() {
        errors.push(VerificationError::new(
            "Preferred name must have at least one item.",
        ));
    }
    if !lang_strings_have_unique_languages(&that.preferred_name) {
        errors.push(VerificationError::new(
            "Preferred name must specify unique languages.",
        ));
    }
    if !that
        .preferred_name
        .iter()
        .any(|ls| is_bcp47_for_english(&ls.language))
    {
        errors.push(VerificationError::new(
            "Constraint AASc-3a-002: preferred name shall be provided at least in English.",
        ));
    }

    if let Some(u) = &that.unit {
        push_prop!(errors, verify_non_empty_xml_serializable_string(u), "unit");
    }
    if let Some(s) = &that.source_of_definition {
        push_prop!(
            errors,
            verify_non_empty_xml_serializable_string(s),
            "sourceOfDefinition"
        );
    }
    if let Some(sym) = &that.symbol {
        push_prop!(
            errors,
            verify_non_empty_xml_serializable_string(sym),
            "symbol"
        );
    }
    if let Some(vf) = &that.value_format {
        push_prop!(
            errors,
            verify_non_empty_xml_serializable_string(vf),
            "valueFormat"
        );
    }
    if let Some(v) = &that.value {
        push_prop!(errors, verify_value_type_iec61360(v), "value");
    }

    if recurse {
        for (idx, item) in that.preferred_name.iter().enumerate() {
            for mut err in verify_lang_string_preferred_name_type_iec61360(item) {
                err.prepend_index(idx);
                err.prepend_property("preferredName");
                errors.push(err);
            }
        }
        if let Some(sn) = &that.short_name {
            for (idx, item) in sn.iter().enumerate() {
                for mut err in verify_lang_string_short_name_type_iec61360(item) {
                    err.prepend_index(idx);
                    err.prepend_property("shortName");
                    errors.push(err);
                }
            }
        }
        if let Some(uid) = &that.unit_id {
            push_prop!(errors, verify_reference(uid, recurse), "unitId");
        }
        if let Some(def) = &that.definition {
            for (idx, item) in def.iter().enumerate() {
                for mut err in verify_lang_string_definition_type_iec61360(item) {
                    err.prepend_index(idx);
                    err.prepend_property("definition");
                    errors.push(err);
                }
            }
        }
        if let Some(vl) = &that.value_list {
            push_prop!(errors, verify_value_list(vl, recurse), "valueList");
        }
        if let Some(lt) = &that.level_type {
            push_prop!(errors, verify_level_type(lt), "levelType");
        }
    }
    errors
}
