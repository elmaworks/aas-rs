//! Deserialize AAS classes from `serde_json::Value`.

#![allow(clippy::too_many_lines)]

use serde_json::Value;

use crate::stringification;
use crate::types::class::Class;
use crate::{
    AasSubmodelElements, AssetKind, DataTypeDefXsd, DataTypeIec61360, Direction, EntityType,
    KeyTypes, ModellingKind, QualifierKind, ReferenceTypes, StateOfEvent,
};
use crate::{
    AdministrativeInformation, AnnotatedRelationshipElement, AssetAdministrationShell,
    AssetInformation, BasicEventElement, Blob, Capability, ConceptDescription,
    DataSpecificationIec61360, EmbeddedDataSpecification, Entity, Environment, EventPayload,
    Extension, File, Key, LangStringDefinitionTypeIec61360, LangStringNameType,
    LangStringPreferredNameTypeIec61360, LangStringShortNameTypeIec61360, LangStringTextType,
    LevelType, MultiLanguageProperty, Operation, OperationVariable, Property, Qualifier, Range,
    Reference, ReferenceElement, RelationshipElement, Resource, SpecificAssetId, Submodel,
    SubmodelElementCollection, SubmodelElementList, ValueList, ValueReferencePair,
};

use super::error::DeserializationError;

// ── Primitive helpers ─────────────────────────────────────────────────────────

fn bool_from_jsonable(v: &Value) -> Result<bool, DeserializationError> {
    v.as_bool().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a boolean, but got null")
        } else {
            DeserializationError::new(format!("Expected a boolean, but got {}", json_type_name(v)))
        }
    })
}

fn str_from_jsonable(v: &Value) -> Result<String, DeserializationError> {
    v.as_str()
        .map(std::borrow::ToOwned::to_owned)
        .ok_or_else(|| {
            if v.is_null() {
                DeserializationError::new("Expected a string, but got null")
            } else {
                DeserializationError::new(format!(
                    "Expected a string, but got: {}",
                    json_type_name(v)
                ))
            }
        })
}

/// Parse a string from a JSON value for enum dispatch.
///
/// Unlike [`str_from_jsonable`], null values produce `"but got: object"` to match
/// TypeScript's `typeof null === "object"` behavior.
fn enum_str_from_jsonable(v: &Value) -> Result<String, DeserializationError> {
    v.as_str()
        .map(std::borrow::ToOwned::to_owned)
        .ok_or_else(|| {
            // TypeScript's typeof null === "object", so report "object" for both null and object types
            let type_name = if v.is_null() {
                "object"
            } else {
                json_type_name(v)
            };
            DeserializationError::new(format!("Expected a string, but got: {type_name}"))
        })
}

fn bytes_from_jsonable(v: &Value) -> Result<Vec<u8>, DeserializationError> {
    if v.is_null() {
        return Err(DeserializationError::new(
            "Expected a base64-encoded string, but got null",
        ));
    }
    let s = v.as_str().ok_or_else(|| {
        DeserializationError::new(format!(
            "Expected a base64-encoded string, but got: {}",
            json_type_name(v)
        ))
    })?;
    crate::common::base64_decode(s)
        .map_err(|e| DeserializationError::new(format!("Invalid base64 encoding: {e}")))
}

fn int64_from_jsonable(v: &Value) -> Result<i64, DeserializationError> {
    v.as_i64().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected an integer, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected an integer, but got: {}",
                json_type_name(v)
            ))
        }
    })
}

fn float64_from_jsonable(v: &Value) -> Result<f64, DeserializationError> {
    v.as_f64().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a float, but got null")
        } else {
            DeserializationError::new(format!("Expected a float, but got: {}", json_type_name(v)))
        }
    })
}

/// Returns the JSON type name for error messages.
///
/// Note: TypeScript's `typeof [] === "object"`, so arrays map to `"object"`.
/// Null is handled separately (TypeScript shows "null" as a separate type in these messages).
fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "object",
        Value::Object(_) => "object",
    }
}

// ── Enum helpers ──────────────────────────────────────────────────────────────

/// Deserialize a [`ModellingKind`] from a JSON value.
pub fn modelling_kind_from_jsonable(v: &Value) -> Result<ModellingKind, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::modelling_kind_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of ModellingKind: {s}"
        ))
    })
}

/// Deserialize a [`QualifierKind`] from a JSON value.
pub fn qualifier_kind_from_jsonable(v: &Value) -> Result<QualifierKind, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::qualifier_kind_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of QualifierKind: {s}"
        ))
    })
}

/// Deserialize an [`AssetKind`] from a JSON value.
pub fn asset_kind_from_jsonable(v: &Value) -> Result<AssetKind, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::asset_kind_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of AssetKind: {s}"
        ))
    })
}

/// Deserialize an [`AasSubmodelElements`] from a JSON value.
pub fn aas_submodel_elements_from_jsonable(
    v: &Value,
) -> Result<AasSubmodelElements, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::aas_submodel_elements_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of AasSubmodelElements: {s}"
        ))
    })
}

/// Deserialize an [`EntityType`] from a JSON value.
pub fn entity_type_from_jsonable(v: &Value) -> Result<EntityType, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::entity_type_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of EntityType: {s}"
        ))
    })
}

/// Deserialize a [`Direction`] from a JSON value.
pub fn direction_from_jsonable(v: &Value) -> Result<Direction, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::direction_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of Direction: {s}"
        ))
    })
}

/// Deserialize a [`StateOfEvent`] from a JSON value.
pub fn state_of_event_from_jsonable(v: &Value) -> Result<StateOfEvent, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::state_of_event_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of StateOfEvent: {s}"
        ))
    })
}

/// Deserialize a [`ReferenceTypes`] from a JSON value.
pub fn reference_types_from_jsonable(v: &Value) -> Result<ReferenceTypes, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::reference_types_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of ReferenceTypes: {s}"
        ))
    })
}

/// Deserialize a [`KeyTypes`] from a JSON value.
pub fn key_types_from_jsonable(v: &Value) -> Result<KeyTypes, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::key_types_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of KeyTypes: {s}"
        ))
    })
}

/// Deserialize a [`DataTypeDefXsd`] from a JSON value.
pub fn data_type_def_xsd_from_jsonable(v: &Value) -> Result<DataTypeDefXsd, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::data_type_def_xsd_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of DataTypeDefXsd: {s}"
        ))
    })
}

/// Deserialize a [`DataTypeIec61360`] from a JSON value.
pub fn data_type_iec61360_from_jsonable(
    v: &Value,
) -> Result<DataTypeIec61360, DeserializationError> {
    let s = enum_str_from_jsonable(v)?;
    stringification::data_type_iec_61360_from_str(&s).ok_or_else(|| {
        DeserializationError::new(format!(
            "Not a valid string representation of a literal of DataTypeIec61360: {s}"
        ))
    })
}

// Keep internal alias for use within this module.
fn data_type_iec_61360_from_jsonable(v: &Value) -> Result<DataTypeIec61360, DeserializationError> {
    data_type_iec61360_from_jsonable(v)
}

// ── Struct parsers ────────────────────────────────────────────────────────────

/// Deserialize a [`Key`] from a JSON value.
pub fn key_from_jsonable(v: &Value) -> Result<Key, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;

    let mut type_: Option<KeyTypes> = None;
    let mut value: Option<String> = None;

    for (key, val) in obj {
        match key.as_str() {
            "type" => {
                type_ = Some(key_types_from_jsonable(val).map_err(|e| e.prepend_property("type"))?);
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            _ => {}
        }
    }

    Ok(Key {
        type_: type_
            .ok_or_else(|| DeserializationError::new("The required property 'type' is missing"))?,
        value: value
            .ok_or_else(|| DeserializationError::new("The required property 'value' is missing"))?,
    })
}

/// Deserialize a [`Reference`] from a JSON value.
pub fn reference_from_jsonable(v: &Value) -> Result<Reference, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;

    let mut type_: Option<ReferenceTypes> = None;
    let mut referred_semantic_id: Option<Box<Reference>> = None;
    let mut keys: Option<Vec<Key>> = None;

    for (key, val) in obj {
        match key.as_str() {
            "type" => {
                type_ = Some(
                    reference_types_from_jsonable(val).map_err(|e| e.prepend_property("type"))?,
                );
            }
            "referredSemanticId" => {
                referred_semantic_id = Some(Box::new(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("referredSemanticId"))?,
                ));
            }
            "keys" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("keys")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        key_from_jsonable(item)
                            .map_err(|e| e.prepend_index(i).prepend_property("keys"))?,
                    );
                }
                keys = Some(result);
            }
            _ => {}
        }
    }

    Ok(Reference {
        type_: type_
            .ok_or_else(|| DeserializationError::new("The required property 'type' is missing"))?,
        referred_semantic_id,
        keys: keys
            .ok_or_else(|| DeserializationError::new("The required property 'keys' is missing"))?,
    })
}

fn lang_string_name_type_from_jsonable(
    v: &Value,
) -> Result<LangStringNameType, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut language: Option<String> = None;
    let mut text: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "language" => {
                language =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("language"))?);
            }
            "text" => {
                text = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("text"))?);
            }
            _ => {}
        }
    }
    Ok(LangStringNameType {
        language: language.ok_or_else(|| {
            DeserializationError::new("The required property 'language' is missing")
        })?,
        text: text
            .ok_or_else(|| DeserializationError::new("The required property 'text' is missing"))?,
    })
}

fn lang_string_text_type_from_jsonable(
    v: &Value,
) -> Result<LangStringTextType, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut language: Option<String> = None;
    let mut text: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "language" => {
                language =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("language"))?);
            }
            "text" => {
                text = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("text"))?);
            }
            _ => {}
        }
    }
    Ok(LangStringTextType {
        language: language.ok_or_else(|| {
            DeserializationError::new("The required property 'language' is missing")
        })?,
        text: text
            .ok_or_else(|| DeserializationError::new("The required property 'text' is missing"))?,
    })
}

fn lang_string_preferred_name_type_iec61360_from_jsonable(
    v: &Value,
) -> Result<LangStringPreferredNameTypeIec61360, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut language: Option<String> = None;
    let mut text: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "language" => {
                language =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("language"))?);
            }
            "text" => {
                text = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("text"))?);
            }
            _ => {}
        }
    }
    Ok(LangStringPreferredNameTypeIec61360 {
        language: language.ok_or_else(|| {
            DeserializationError::new("The required property 'language' is missing")
        })?,
        text: text
            .ok_or_else(|| DeserializationError::new("The required property 'text' is missing"))?,
    })
}

fn lang_string_short_name_type_iec61360_from_jsonable(
    v: &Value,
) -> Result<LangStringShortNameTypeIec61360, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut language: Option<String> = None;
    let mut text: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "language" => {
                language =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("language"))?);
            }
            "text" => {
                text = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("text"))?);
            }
            _ => {}
        }
    }
    Ok(LangStringShortNameTypeIec61360 {
        language: language.ok_or_else(|| {
            DeserializationError::new("The required property 'language' is missing")
        })?,
        text: text
            .ok_or_else(|| DeserializationError::new("The required property 'text' is missing"))?,
    })
}

fn lang_string_definition_type_iec61360_from_jsonable(
    v: &Value,
) -> Result<LangStringDefinitionTypeIec61360, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut language: Option<String> = None;
    let mut text: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "language" => {
                language =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("language"))?);
            }
            "text" => {
                text = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("text"))?);
            }
            _ => {}
        }
    }
    Ok(LangStringDefinitionTypeIec61360 {
        language: language.ok_or_else(|| {
            DeserializationError::new("The required property 'language' is missing")
        })?,
        text: text
            .ok_or_else(|| DeserializationError::new("The required property 'text' is missing"))?,
    })
}

fn resource_from_jsonable(v: &Value) -> Result<Resource, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut path: Option<String> = None;
    let mut content_type: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "path" => {
                path = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("path"))?);
            }
            "contentType" => {
                content_type =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("contentType"))?);
            }
            _ => {}
        }
    }
    Ok(Resource {
        path: path
            .ok_or_else(|| DeserializationError::new("The required property 'path' is missing"))?,
        content_type,
    })
}

fn level_type_from_jsonable(v: &Value) -> Result<LevelType, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut min: Option<bool> = None;
    let mut nom: Option<bool> = None;
    let mut typ: Option<bool> = None;
    let mut max: Option<bool> = None;
    for (k, val) in obj {
        match k.as_str() {
            "min" => {
                min = Some(bool_from_jsonable(val).map_err(|e| e.prepend_property("min"))?);
            }
            "nom" => {
                nom = Some(bool_from_jsonable(val).map_err(|e| e.prepend_property("nom"))?);
            }
            "typ" => {
                typ = Some(bool_from_jsonable(val).map_err(|e| e.prepend_property("typ"))?);
            }
            "max" => {
                max = Some(bool_from_jsonable(val).map_err(|e| e.prepend_property("max"))?);
            }
            _ => {}
        }
    }
    Ok(LevelType {
        min: min
            .ok_or_else(|| DeserializationError::new("The required property 'min' is missing"))?,
        nom: nom
            .ok_or_else(|| DeserializationError::new("The required property 'nom' is missing"))?,
        typ: typ
            .ok_or_else(|| DeserializationError::new("The required property 'typ' is missing"))?,
        max: max
            .ok_or_else(|| DeserializationError::new("The required property 'max' is missing"))?,
    })
}

fn value_reference_pair_from_jsonable(
    v: &Value,
) -> Result<ValueReferencePair, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut value: Option<String> = None;
    let mut value_id: Option<Reference> = None;
    for (k, val) in obj {
        match k.as_str() {
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "valueId" => {
                value_id =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("valueId"))?);
            }
            _ => {}
        }
    }
    Ok(ValueReferencePair {
        value: value
            .ok_or_else(|| DeserializationError::new("The required property 'value' is missing"))?,
        value_id: value_id.ok_or_else(|| {
            DeserializationError::new("The required property 'valueId' is missing")
        })?,
    })
}

fn value_list_from_jsonable(v: &Value) -> Result<ValueList, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut value_reference_pairs: Option<Vec<ValueReferencePair>> = None;
    for (k, val) in obj {
        if k == "valueReferencePairs" {
            let arr = val.as_array().ok_or_else(|| {
                let msg = if val.is_null() {
                    "Expected an iterable, but got null".to_string()
                } else {
                    format!("Expected an iterable, but got: {}", json_type_name(val))
                };
                DeserializationError::new(msg).prepend_property("valueReferencePairs")
            })?;
            let mut result = Vec::with_capacity(arr.len());
            for (i, item) in arr.iter().enumerate() {
                result.push(
                    value_reference_pair_from_jsonable(item)
                        .map_err(|e| e.prepend_index(i).prepend_property("valueReferencePairs"))?,
                );
            }
            value_reference_pairs = Some(result);
        }
    }
    Ok(ValueList {
        value_reference_pairs: value_reference_pairs.ok_or_else(|| {
            DeserializationError::new("The required property 'valueReferencePairs' is missing")
        })?,
    })
}

fn extension_from_jsonable(v: &Value) -> Result<Extension, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut semantic_id: Option<Reference> = None;
    let mut supplemental_semantic_ids: Option<Vec<Reference>> = None;
    let mut name: Option<String> = None;
    let mut value_type: Option<DataTypeDefXsd> = None;
    let mut value: Option<String> = None;
    let mut refers_to: Option<Vec<Reference>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "semanticId" => {
                semantic_id = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("semanticId"))?,
                );
            }
            "supplementalSemanticIds" => {
                supplemental_semantic_ids = Some(parse_ref_array(val, "supplementalSemanticIds")?);
            }
            "name" => {
                name = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("name"))?);
            }
            "valueType" => {
                value_type = Some(
                    data_type_def_xsd_from_jsonable(val)
                        .map_err(|e| e.prepend_property("valueType"))?,
                );
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "refersTo" => {
                refers_to = Some(parse_ref_array(val, "refersTo")?);
            }
            _ => {}
        }
    }
    Ok(Extension {
        semantic_id,
        supplemental_semantic_ids,
        name: name
            .ok_or_else(|| DeserializationError::new("The required property 'name' is missing"))?,
        value_type,
        value,
        refers_to,
    })
}

fn qualifier_from_jsonable(v: &Value) -> Result<Qualifier, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut semantic_id: Option<Reference> = None;
    let mut supplemental_semantic_ids: Option<Vec<Reference>> = None;
    let mut kind: Option<QualifierKind> = None;
    let mut type_: Option<String> = None;
    let mut value_type: Option<DataTypeDefXsd> = None;
    let mut value: Option<String> = None;
    let mut value_id: Option<Reference> = None;
    for (k, val) in obj {
        match k.as_str() {
            "semanticId" => {
                semantic_id = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("semanticId"))?,
                );
            }
            "supplementalSemanticIds" => {
                supplemental_semantic_ids = Some(parse_ref_array(val, "supplementalSemanticIds")?);
            }
            "kind" => {
                kind = Some(
                    qualifier_kind_from_jsonable(val).map_err(|e| e.prepend_property("kind"))?,
                );
            }
            "type" => {
                type_ = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("type"))?);
            }
            "valueType" => {
                value_type = Some(
                    data_type_def_xsd_from_jsonable(val)
                        .map_err(|e| e.prepend_property("valueType"))?,
                );
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "valueId" => {
                value_id =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("valueId"))?);
            }
            _ => {}
        }
    }
    Ok(Qualifier {
        semantic_id,
        supplemental_semantic_ids,
        kind,
        type_: type_
            .ok_or_else(|| DeserializationError::new("The required property 'type' is missing"))?,
        value_type: value_type.ok_or_else(|| {
            DeserializationError::new("The required property 'valueType' is missing")
        })?,
        value,
        value_id,
    })
}

fn specific_asset_id_from_jsonable(v: &Value) -> Result<SpecificAssetId, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut semantic_id: Option<Reference> = None;
    let mut supplemental_semantic_ids: Option<Vec<Reference>> = None;
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;
    let mut external_subject_id: Option<Reference> = None;
    for (k, val) in obj {
        match k.as_str() {
            "semanticId" => {
                semantic_id = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("semanticId"))?,
                );
            }
            "supplementalSemanticIds" => {
                supplemental_semantic_ids = Some(parse_ref_array(val, "supplementalSemanticIds")?);
            }
            "name" => {
                name = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("name"))?);
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "externalSubjectId" => {
                external_subject_id = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("externalSubjectId"))?,
                );
            }
            _ => {}
        }
    }
    Ok(SpecificAssetId {
        semantic_id,
        supplemental_semantic_ids,
        name: name
            .ok_or_else(|| DeserializationError::new("The required property 'name' is missing"))?,
        value: value
            .ok_or_else(|| DeserializationError::new("The required property 'value' is missing"))?,
        external_subject_id,
    })
}

fn administrative_information_from_jsonable(
    v: &Value,
) -> Result<AdministrativeInformation, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>> = None;
    let mut version: Option<String> = None;
    let mut revision: Option<String> = None;
    let mut creator: Option<Reference> = None;
    let mut template_id: Option<String> = None;
    for (k, val) in obj {
        match k.as_str() {
            "embeddedDataSpecifications" => {
                embedded_data_specifications = Some(parse_embedded_data_spec_array(
                    val,
                    "embeddedDataSpecifications",
                )?);
            }
            "version" => {
                version = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("version"))?);
            }
            "revision" => {
                revision =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("revision"))?);
            }
            "creator" => {
                creator =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("creator"))?);
            }
            "templateId" => {
                template_id =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("templateId"))?);
            }
            _ => {}
        }
    }
    Ok(AdministrativeInformation {
        embedded_data_specifications,
        version,
        revision,
        creator,
        template_id,
    })
}

fn embedded_data_specification_from_jsonable(
    v: &Value,
) -> Result<EmbeddedDataSpecification, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut data_specification: Option<Reference> = None;
    let mut data_specification_content: Option<Box<Class>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "dataSpecification" => {
                data_specification = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("dataSpecification"))?,
                );
            }
            "dataSpecificationContent" => {
                data_specification_content = Some(Box::new(
                    data_spec_content_from_jsonable(val)
                        .map_err(|e| e.prepend_property("dataSpecificationContent"))?,
                ));
            }
            _ => {}
        }
    }
    Ok(EmbeddedDataSpecification {
        data_specification: data_specification.ok_or_else(|| {
            DeserializationError::new("The required property 'dataSpecification' is missing")
        })?,
        data_specification_content: data_specification_content.ok_or_else(|| {
            DeserializationError::new("The required property 'dataSpecificationContent' is missing")
        })?,
    })
}

fn data_specification_iec61360_from_jsonable(
    v: &Value,
) -> Result<DataSpecificationIec61360, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut preferred_name: Option<Vec<LangStringPreferredNameTypeIec61360>> = None;
    let mut short_name: Option<Vec<LangStringShortNameTypeIec61360>> = None;
    let mut unit: Option<String> = None;
    let mut unit_id: Option<Reference> = None;
    let mut source_of_definition: Option<String> = None;
    let mut symbol: Option<String> = None;
    let mut data_type: Option<DataTypeIec61360> = None;
    let mut definition: Option<Vec<LangStringDefinitionTypeIec61360>> = None;
    let mut value_format: Option<String> = None;
    let mut value_list: Option<ValueList> = None;
    let mut value: Option<String> = None;
    let mut level_type: Option<LevelType> = None;
    for (k, val) in obj {
        match k.as_str() {
            "preferredName" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("preferredName")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        lang_string_preferred_name_type_iec61360_from_jsonable(item)
                            .map_err(|e| e.prepend_index(i).prepend_property("preferredName"))?,
                    );
                }
                preferred_name = Some(result);
            }
            "shortName" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("shortName")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        lang_string_short_name_type_iec61360_from_jsonable(item)
                            .map_err(|e| e.prepend_index(i).prepend_property("shortName"))?,
                    );
                }
                short_name = Some(result);
            }
            "unit" => {
                unit = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("unit"))?);
            }
            "unitId" => {
                unit_id =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("unitId"))?);
            }
            "sourceOfDefinition" => {
                source_of_definition = Some(
                    str_from_jsonable(val).map_err(|e| e.prepend_property("sourceOfDefinition"))?,
                );
            }
            "symbol" => {
                symbol = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("symbol"))?);
            }
            "dataType" => {
                data_type = Some(
                    data_type_iec_61360_from_jsonable(val)
                        .map_err(|e| e.prepend_property("dataType"))?,
                );
            }
            "definition" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("definition")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        lang_string_definition_type_iec61360_from_jsonable(item)
                            .map_err(|e| e.prepend_index(i).prepend_property("definition"))?,
                    );
                }
                definition = Some(result);
            }
            "valueFormat" => {
                value_format =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("valueFormat"))?);
            }
            "valueList" => {
                value_list = Some(
                    value_list_from_jsonable(val).map_err(|e| e.prepend_property("valueList"))?,
                );
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "levelType" => {
                level_type = Some(
                    level_type_from_jsonable(val).map_err(|e| e.prepend_property("levelType"))?,
                );
            }
            _ => {}
        }
    }
    Ok(DataSpecificationIec61360 {
        preferred_name: preferred_name.ok_or_else(|| {
            DeserializationError::new("The required property 'preferredName' is missing")
        })?,
        short_name,
        unit,
        unit_id,
        source_of_definition,
        symbol,
        data_type,
        definition,
        value_format,
        value_list,
        value,
        level_type,
    })
}

// ── Helper array parsers ──────────────────────────────────────────────────────

fn parse_ref_array(v: &Value, prop: &'static str) -> Result<Vec<Reference>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            reference_from_jsonable(item).map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_extension_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<Extension>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            extension_from_jsonable(item).map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_lang_name_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<LangStringNameType>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            lang_string_name_type_from_jsonable(item)
                .map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_lang_text_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<LangStringTextType>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            lang_string_text_type_from_jsonable(item)
                .map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_qualifier_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<Qualifier>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            qualifier_from_jsonable(item).map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_embedded_data_spec_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<EmbeddedDataSpecification>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            embedded_data_specification_from_jsonable(item)
                .map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_class_array(v: &Value, prop: &'static str) -> Result<Vec<Class>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            class_from_jsonable(item).map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_specific_asset_id_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<SpecificAssetId>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            specific_asset_id_from_jsonable(item)
                .map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn parse_operation_variable_array(
    v: &Value,
    prop: &'static str,
) -> Result<Vec<OperationVariable>, DeserializationError> {
    let arr = v.as_array().ok_or_else(|| {
        let msg = if v.is_null() {
            "Expected an iterable, but got null".to_string()
        } else {
            format!("Expected an iterable, but got: {}", json_type_name(v))
        };
        DeserializationError::new(msg).prepend_property(prop)
    })?;
    let mut result = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        result.push(
            operation_variable_from_jsonable(item)
                .map_err(|e| e.prepend_index(i).prepend_property(prop))?,
        );
    }
    Ok(result)
}

fn operation_variable_from_jsonable(v: &Value) -> Result<OperationVariable, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut value: Option<Box<Class>> = None;
    for (k, val) in obj {
        if k == "value" {
            value = Some(Box::new(
                class_from_jsonable(val).map_err(|e| e.prepend_property("value"))?,
            ));
        }
    }
    Ok(OperationVariable {
        value: value
            .ok_or_else(|| DeserializationError::new("The required property 'value' is missing"))?,
    })
}

/// Deserialize an [`EventPayload`] from a JSON value.
pub fn event_payload_from_jsonable(v: &Value) -> Result<EventPayload, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut source: Option<Reference> = None;
    let mut source_semantic_id: Option<Reference> = None;
    let mut observable_reference: Option<Reference> = None;
    let mut observable_semantic_id: Option<Reference> = None;
    let mut topic: Option<String> = None;
    let mut subject_id: Option<Reference> = None;
    let mut time_stamp: Option<String> = None;
    let mut payload: Option<Vec<u8>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "source" => {
                source =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("source"))?);
            }
            "sourceSemanticId" => {
                source_semantic_id = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("sourceSemanticId"))?,
                );
            }
            "observableReference" => {
                observable_reference = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("observableReference"))?,
                );
            }
            "observableSemanticId" => {
                observable_semantic_id = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("observableSemanticId"))?,
                );
            }
            "topic" => {
                topic = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("topic"))?);
            }
            "subjectId" => {
                subject_id = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("subjectId"))?,
                );
            }
            "timeStamp" => {
                time_stamp =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("timeStamp"))?);
            }
            "payload" => {
                payload =
                    Some(bytes_from_jsonable(val).map_err(|e| e.prepend_property("payload"))?);
            }
            _ => {}
        }
    }
    Ok(EventPayload {
        source: source.ok_or_else(|| {
            DeserializationError::new("The required property 'source' is missing")
        })?,
        source_semantic_id,
        observable_reference: observable_reference.ok_or_else(|| {
            DeserializationError::new("The required property 'observableReference' is missing")
        })?,
        observable_semantic_id,
        topic,
        subject_id,
        time_stamp: time_stamp.ok_or_else(|| {
            DeserializationError::new("The required property 'timeStamp' is missing")
        })?,
        payload,
    })
}

// ── Core struct parsers ───────────────────────────────────────────────────────

fn asset_information_from_jsonable(v: &Value) -> Result<AssetInformation, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut asset_kind: Option<AssetKind> = None;
    let mut global_asset_id: Option<String> = None;
    let mut specific_asset_ids: Option<Vec<SpecificAssetId>> = None;
    let mut asset_type: Option<String> = None;
    let mut default_thumbnail: Option<Resource> = None;
    for (k, val) in obj {
        match k.as_str() {
            "assetKind" => {
                asset_kind = Some(
                    asset_kind_from_jsonable(val).map_err(|e| e.prepend_property("assetKind"))?,
                );
            }
            "globalAssetId" => {
                global_asset_id =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("globalAssetId"))?);
            }
            "specificAssetIds" => {
                specific_asset_ids = Some(parse_specific_asset_id_array(val, "specificAssetIds")?);
            }
            "assetType" => {
                asset_type =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("assetType"))?);
            }
            "defaultThumbnail" => {
                default_thumbnail = Some(
                    resource_from_jsonable(val)
                        .map_err(|e| e.prepend_property("defaultThumbnail"))?,
                );
            }
            _ => {}
        }
    }
    Ok(AssetInformation {
        asset_kind: asset_kind.ok_or_else(|| {
            DeserializationError::new("The required property 'assetKind' is missing")
        })?,
        global_asset_id,
        specific_asset_ids,
        asset_type,
        default_thumbnail,
    })
}

fn asset_administration_shell_from_jsonable(
    v: &Value,
) -> Result<AssetAdministrationShell, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut extensions: Option<Vec<Extension>> = None;
    let mut category: Option<String> = None;
    let mut id_short: Option<String> = None;
    let mut display_name: Option<Vec<LangStringNameType>> = None;
    let mut description: Option<Vec<LangStringTextType>> = None;
    let mut administration: Option<AdministrativeInformation> = None;
    let mut id: Option<String> = None;
    let mut embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>> = None;
    let mut derived_from: Option<Reference> = None;
    let mut asset_information: Option<AssetInformation> = None;
    let mut submodels: Option<Vec<Reference>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "extensions" => {
                extensions = Some(parse_extension_array(val, "extensions")?);
            }
            "category" => {
                category =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("category"))?);
            }
            "idShort" => {
                id_short = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("idShort"))?);
            }
            "displayName" => {
                display_name = Some(parse_lang_name_array(val, "displayName")?);
            }
            "description" => {
                description = Some(parse_lang_text_array(val, "description")?);
            }
            "administration" => {
                administration = Some(
                    administrative_information_from_jsonable(val)
                        .map_err(|e| e.prepend_property("administration"))?,
                );
            }
            "id" => {
                id = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("id"))?);
            }
            "embeddedDataSpecifications" => {
                embedded_data_specifications = Some(parse_embedded_data_spec_array(
                    val,
                    "embeddedDataSpecifications",
                )?);
            }
            "derivedFrom" => {
                derived_from = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("derivedFrom"))?,
                );
            }
            "assetInformation" => {
                asset_information = Some(
                    asset_information_from_jsonable(val)
                        .map_err(|e| e.prepend_property("assetInformation"))?,
                );
            }
            "submodels" => {
                submodels = Some(parse_ref_array(val, "submodels")?);
            }
            _ => {}
        }
    }
    let id =
        id.ok_or_else(|| DeserializationError::new("The required property 'id' is missing"))?;
    let asset_information = asset_information.ok_or_else(|| {
        DeserializationError::new("The required property 'assetInformation' is missing")
    })?;
    let model_type = obj
        .get("modelType")
        .ok_or_else(|| DeserializationError::new("The required property 'modelType' is missing"))?;
    let model_type_str = model_type.as_str().ok_or_else(|| {
        DeserializationError::new(format!(
            "Expected the property modelType to be a string, but got: {}",
            json_type_name(model_type)
        ))
    })?;
    if model_type_str != "AssetAdministrationShell" {
        return Err(DeserializationError::new(format!(
            "Expected model type 'AssetAdministrationShell', but got: {model_type_str}"
        )));
    }
    Ok(AssetAdministrationShell {
        extensions,
        category,
        id_short,
        display_name,
        description,
        administration,
        id,
        embedded_data_specifications,
        derived_from,
        asset_information,
        submodels,
    })
}

fn submodel_from_jsonable(v: &Value) -> Result<Submodel, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut extensions: Option<Vec<Extension>> = None;
    let mut category: Option<String> = None;
    let mut id_short: Option<String> = None;
    let mut display_name: Option<Vec<LangStringNameType>> = None;
    let mut description: Option<Vec<LangStringTextType>> = None;
    let mut administration: Option<AdministrativeInformation> = None;
    let mut id: Option<String> = None;
    let mut kind: Option<ModellingKind> = None;
    let mut semantic_id: Option<Reference> = None;
    let mut supplemental_semantic_ids: Option<Vec<Reference>> = None;
    let mut qualifiers: Option<Vec<Qualifier>> = None;
    let mut embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>> = None;
    let mut submodel_elements: Option<Vec<Class>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "extensions" => {
                extensions = Some(parse_extension_array(val, "extensions")?);
            }
            "category" => {
                category =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("category"))?);
            }
            "idShort" => {
                id_short = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("idShort"))?);
            }
            "displayName" => {
                display_name = Some(parse_lang_name_array(val, "displayName")?);
            }
            "description" => {
                description = Some(parse_lang_text_array(val, "description")?);
            }
            "administration" => {
                administration = Some(
                    administrative_information_from_jsonable(val)
                        .map_err(|e| e.prepend_property("administration"))?,
                );
            }
            "id" => {
                id = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("id"))?);
            }
            "kind" => {
                kind = Some(
                    modelling_kind_from_jsonable(val).map_err(|e| e.prepend_property("kind"))?,
                );
            }
            "semanticId" => {
                semantic_id = Some(
                    reference_from_jsonable(val).map_err(|e| e.prepend_property("semanticId"))?,
                );
            }
            "supplementalSemanticIds" => {
                supplemental_semantic_ids = Some(parse_ref_array(val, "supplementalSemanticIds")?);
            }
            "qualifiers" => {
                qualifiers = Some(parse_qualifier_array(val, "qualifiers")?);
            }
            "embeddedDataSpecifications" => {
                embedded_data_specifications = Some(parse_embedded_data_spec_array(
                    val,
                    "embeddedDataSpecifications",
                )?);
            }
            "submodelElements" => {
                submodel_elements = Some(parse_class_array(val, "submodelElements")?);
            }
            _ => {}
        }
    }
    let id =
        id.ok_or_else(|| DeserializationError::new("The required property 'id' is missing"))?;
    let model_type = obj
        .get("modelType")
        .ok_or_else(|| DeserializationError::new("The required property 'modelType' is missing"))?;
    let model_type_str = model_type.as_str().ok_or_else(|| {
        DeserializationError::new(format!(
            "Expected the property modelType to be a string, but got: {}",
            json_type_name(model_type)
        ))
    })?;
    if model_type_str != "Submodel" {
        return Err(DeserializationError::new(format!(
            "Expected model type 'Submodel', but got: {model_type_str}"
        )));
    }
    Ok(Submodel {
        extensions,
        category,
        id_short,
        display_name,
        description,
        administration,
        id,
        kind,
        semantic_id,
        supplemental_semantic_ids,
        qualifiers,
        embedded_data_specifications,
        submodel_elements,
    })
}

fn concept_description_from_jsonable(
    v: &Value,
) -> Result<ConceptDescription, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut extensions: Option<Vec<Extension>> = None;
    let mut category: Option<String> = None;
    let mut id_short: Option<String> = None;
    let mut display_name: Option<Vec<LangStringNameType>> = None;
    let mut description: Option<Vec<LangStringTextType>> = None;
    let mut administration: Option<AdministrativeInformation> = None;
    let mut id: Option<String> = None;
    let mut embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>> = None;
    let mut is_case_of: Option<Vec<Reference>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "extensions" => {
                extensions = Some(parse_extension_array(val, "extensions")?);
            }
            "category" => {
                category =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("category"))?);
            }
            "idShort" => {
                id_short = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("idShort"))?);
            }
            "displayName" => {
                display_name = Some(parse_lang_name_array(val, "displayName")?);
            }
            "description" => {
                description = Some(parse_lang_text_array(val, "description")?);
            }
            "administration" => {
                administration = Some(
                    administrative_information_from_jsonable(val)
                        .map_err(|e| e.prepend_property("administration"))?,
                );
            }
            "id" => {
                id = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("id"))?);
            }
            "embeddedDataSpecifications" => {
                embedded_data_specifications = Some(parse_embedded_data_spec_array(
                    val,
                    "embeddedDataSpecifications",
                )?);
            }
            "isCaseOf" => {
                is_case_of = Some(parse_ref_array(val, "isCaseOf")?);
            }
            _ => {}
        }
    }
    let id =
        id.ok_or_else(|| DeserializationError::new("The required property 'id' is missing"))?;
    let model_type = obj
        .get("modelType")
        .ok_or_else(|| DeserializationError::new("The required property 'modelType' is missing"))?;
    let model_type_str = model_type.as_str().ok_or_else(|| {
        DeserializationError::new(format!(
            "Expected the property modelType to be a string, but got: {}",
            json_type_name(model_type)
        ))
    })?;
    if model_type_str != "ConceptDescription" {
        return Err(DeserializationError::new(format!(
            "Expected model type 'ConceptDescription', but got: {model_type_str}"
        )));
    }
    Ok(ConceptDescription {
        extensions,
        category,
        id_short,
        display_name,
        description,
        administration,
        id,
        embedded_data_specifications,
        is_case_of,
    })
}

fn environment_from_jsonable(v: &Value) -> Result<Environment, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut asset_administration_shells: Option<Vec<AssetAdministrationShell>> = None;
    let mut submodels: Option<Vec<Submodel>> = None;
    let mut concept_descriptions: Option<Vec<ConceptDescription>> = None;
    for (k, val) in obj {
        match k.as_str() {
            "assetAdministrationShells" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("assetAdministrationShells")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(asset_administration_shell_from_jsonable(item).map_err(|e| {
                        e.prepend_index(i)
                            .prepend_property("assetAdministrationShells")
                    })?);
                }
                asset_administration_shells = Some(result);
            }
            "submodels" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("submodels")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        submodel_from_jsonable(item)
                            .map_err(|e| e.prepend_index(i).prepend_property("submodels"))?,
                    );
                }
                submodels = Some(result);
            }
            "conceptDescriptions" => {
                let arr = val.as_array().ok_or_else(|| {
                    let msg = if val.is_null() {
                        "Expected an iterable, but got null".to_string()
                    } else {
                        format!("Expected an iterable, but got: {}", json_type_name(val))
                    };
                    DeserializationError::new(msg).prepend_property("conceptDescriptions")
                })?;
                let mut result = Vec::with_capacity(arr.len());
                for (i, item) in arr.iter().enumerate() {
                    result.push(
                        concept_description_from_jsonable(item).map_err(|e| {
                            e.prepend_index(i).prepend_property("conceptDescriptions")
                        })?,
                    );
                }
                concept_descriptions = Some(result);
            }
            _ => {}
        }
    }
    Ok(Environment {
        asset_administration_shells,
        submodels,
        concept_descriptions,
    })
}

// ── SME common field parser ───────────────────────────────────────────────────

struct SmeCommonFields {
    extensions: Option<Vec<Extension>>,
    category: Option<String>,
    id_short: Option<String>,
    display_name: Option<Vec<LangStringNameType>>,
    description: Option<Vec<LangStringTextType>>,
    semantic_id: Option<Reference>,
    supplemental_semantic_ids: Option<Vec<Reference>>,
    qualifiers: Option<Vec<Qualifier>>,
    embedded_data_specifications: Option<Vec<EmbeddedDataSpecification>>,
}

fn parse_sme_common_field(
    k: &str,
    val: &Value,
    fields: &mut SmeCommonFields,
) -> Result<bool, DeserializationError> {
    match k {
        "extensions" => {
            fields.extensions = Some(parse_extension_array(val, "extensions")?);
            Ok(true)
        }
        "category" => {
            fields.category =
                Some(str_from_jsonable(val).map_err(|e| e.prepend_property("category"))?);
            Ok(true)
        }
        "idShort" => {
            fields.id_short =
                Some(str_from_jsonable(val).map_err(|e| e.prepend_property("idShort"))?);
            Ok(true)
        }
        "displayName" => {
            fields.display_name = Some(parse_lang_name_array(val, "displayName")?);
            Ok(true)
        }
        "description" => {
            fields.description = Some(parse_lang_text_array(val, "description")?);
            Ok(true)
        }
        "semanticId" => {
            fields.semantic_id =
                Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("semanticId"))?);
            Ok(true)
        }
        "supplementalSemanticIds" => {
            fields.supplemental_semantic_ids =
                Some(parse_ref_array(val, "supplementalSemanticIds")?);
            Ok(true)
        }
        "qualifiers" => {
            fields.qualifiers = Some(parse_qualifier_array(val, "qualifiers")?);
            Ok(true)
        }
        "embeddedDataSpecifications" => {
            fields.embedded_data_specifications = Some(parse_embedded_data_spec_array(
                val,
                "embeddedDataSpecifications",
            )?);
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn new_sme_fields() -> SmeCommonFields {
    SmeCommonFields {
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

// ── SME concrete parsers ──────────────────────────────────────────────────────

fn property_from_jsonable(v: &Value) -> Result<Property, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value_type: Option<DataTypeDefXsd> = None;
    let mut value: Option<String> = None;
    let mut value_id: Option<Reference> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "valueType" => {
                value_type = Some(
                    data_type_def_xsd_from_jsonable(val)
                        .map_err(|e| e.prepend_property("valueType"))?,
                );
            }
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "valueId" => {
                value_id =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("valueId"))?);
            }
            _ => {}
        }
    }
    Ok(Property {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value_type: value_type.ok_or_else(|| {
            DeserializationError::new("The required property 'valueType' is missing")
        })?,
        value,
        value_id,
    })
}

fn multi_language_property_from_jsonable(
    v: &Value,
) -> Result<MultiLanguageProperty, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value: Option<Vec<LangStringTextType>> = None;
    let mut value_id: Option<Reference> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "value" => {
                value = Some(parse_lang_text_array(val, "value")?);
            }
            "valueId" => {
                value_id =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("valueId"))?);
            }
            _ => {}
        }
    }
    Ok(MultiLanguageProperty {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value,
        value_id,
    })
}

fn range_from_jsonable(v: &Value) -> Result<Range, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value_type: Option<DataTypeDefXsd> = None;
    let mut min: Option<String> = None;
    let mut max: Option<String> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "valueType" => {
                value_type = Some(
                    data_type_def_xsd_from_jsonable(val)
                        .map_err(|e| e.prepend_property("valueType"))?,
                );
            }
            "min" => {
                min = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("min"))?);
            }
            "max" => {
                max = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("max"))?);
            }
            _ => {}
        }
    }
    Ok(Range {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value_type: value_type.ok_or_else(|| {
            DeserializationError::new("The required property 'valueType' is missing")
        })?,
        min,
        max,
    })
}

fn reference_element_from_jsonable(v: &Value) -> Result<ReferenceElement, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value: Option<Reference> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        if k == "value" {
            value = Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
        }
    }
    Ok(ReferenceElement {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value,
    })
}

fn blob_from_jsonable(v: &Value) -> Result<Blob, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "value" => {
                value = Some(bytes_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "contentType" => {
                content_type =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("contentType"))?);
            }
            _ => {}
        }
    }
    Ok(Blob {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value,
        content_type: content_type.ok_or_else(|| {
            DeserializationError::new("The required property 'contentType' is missing")
        })?,
    })
}

fn file_from_jsonable(v: &Value) -> Result<File, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value: Option<String> = None;
    let mut content_type: Option<String> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "value" => {
                value = Some(str_from_jsonable(val).map_err(|e| e.prepend_property("value"))?);
            }
            "contentType" => {
                content_type =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("contentType"))?);
            }
            _ => {}
        }
    }
    Ok(File {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value,
        content_type: content_type.ok_or_else(|| {
            DeserializationError::new("The required property 'contentType' is missing")
        })?,
    })
}

fn relationship_element_from_jsonable(
    v: &Value,
) -> Result<RelationshipElement, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut first: Option<Reference> = None;
    let mut second: Option<Reference> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "first" => {
                first =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("first"))?);
            }
            "second" => {
                second =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("second"))?);
            }
            _ => {}
        }
    }
    Ok(RelationshipElement {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        first: first
            .ok_or_else(|| DeserializationError::new("The required property 'first' is missing"))?,
        second: second.ok_or_else(|| {
            DeserializationError::new("The required property 'second' is missing")
        })?,
    })
}

fn annotated_relationship_element_from_jsonable(
    v: &Value,
) -> Result<AnnotatedRelationshipElement, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut first: Option<Reference> = None;
    let mut second: Option<Reference> = None;
    let mut annotations: Option<Vec<Class>> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "first" => {
                first =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("first"))?);
            }
            "second" => {
                second =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("second"))?);
            }
            "annotations" => {
                annotations = Some(parse_class_array(val, "annotations")?);
            }
            _ => {}
        }
    }
    Ok(AnnotatedRelationshipElement {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        first: first
            .ok_or_else(|| DeserializationError::new("The required property 'first' is missing"))?,
        second: second.ok_or_else(|| {
            DeserializationError::new("The required property 'second' is missing")
        })?,
        annotations,
    })
}

fn submodel_element_list_from_jsonable(
    v: &Value,
) -> Result<SubmodelElementList, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut order_relevant: Option<bool> = None;
    let mut semantic_id_list_element: Option<Reference> = None;
    let mut type_value_list_element: Option<AasSubmodelElements> = None;
    let mut value_type_list_element: Option<DataTypeDefXsd> = None;
    let mut value: Option<Vec<Class>> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "orderRelevant" => {
                order_relevant =
                    Some(bool_from_jsonable(val).map_err(|e| e.prepend_property("orderRelevant"))?);
            }
            "semanticIdListElement" => {
                semantic_id_list_element = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("semanticIdListElement"))?,
                );
            }
            "typeValueListElement" => {
                type_value_list_element = Some(
                    aas_submodel_elements_from_jsonable(val)
                        .map_err(|e| e.prepend_property("typeValueListElement"))?,
                );
            }
            "valueTypeListElement" => {
                value_type_list_element = Some(
                    data_type_def_xsd_from_jsonable(val)
                        .map_err(|e| e.prepend_property("valueTypeListElement"))?,
                );
            }
            "value" => {
                value = Some(parse_class_array(val, "value")?);
            }
            _ => {}
        }
    }
    Ok(SubmodelElementList {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        order_relevant,
        semantic_id_list_element,
        type_value_list_element: type_value_list_element.ok_or_else(|| {
            DeserializationError::new("The required property 'typeValueListElement' is missing")
        })?,
        value_type_list_element,
        value,
    })
}

fn submodel_element_collection_from_jsonable(
    v: &Value,
) -> Result<SubmodelElementCollection, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut value: Option<Vec<Class>> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        if k == "value" {
            value = Some(parse_class_array(val, "value")?);
        }
    }
    Ok(SubmodelElementCollection {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        value,
    })
}

fn entity_from_jsonable(v: &Value) -> Result<Entity, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut statements: Option<Vec<Class>> = None;
    let mut entity_type: Option<EntityType> = None;
    let mut global_asset_id: Option<String> = None;
    let mut specific_asset_ids: Option<Vec<SpecificAssetId>> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "statements" => {
                statements = Some(parse_class_array(val, "statements")?);
            }
            "entityType" => {
                entity_type = Some(
                    entity_type_from_jsonable(val).map_err(|e| e.prepend_property("entityType"))?,
                );
            }
            "globalAssetId" => {
                global_asset_id =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("globalAssetId"))?);
            }
            "specificAssetIds" => {
                specific_asset_ids = Some(parse_specific_asset_id_array(val, "specificAssetIds")?);
            }
            _ => {}
        }
    }
    Ok(Entity {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        statements,
        entity_type: entity_type.ok_or_else(|| {
            DeserializationError::new("The required property 'entityType' is missing")
        })?,
        global_asset_id,
        specific_asset_ids,
    })
}

fn basic_event_element_from_jsonable(v: &Value) -> Result<BasicEventElement, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut observed: Option<Reference> = None;
    let mut direction: Option<Direction> = None;
    let mut state: Option<StateOfEvent> = None;
    let mut message_topic: Option<String> = None;
    let mut message_broker: Option<Reference> = None;
    let mut last_update: Option<String> = None;
    let mut min_interval: Option<String> = None;
    let mut max_interval: Option<String> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "observed" => {
                observed =
                    Some(reference_from_jsonable(val).map_err(|e| e.prepend_property("observed"))?);
            }
            "direction" => {
                direction = Some(
                    direction_from_jsonable(val).map_err(|e| e.prepend_property("direction"))?,
                );
            }
            "state" => {
                state = Some(
                    state_of_event_from_jsonable(val).map_err(|e| e.prepend_property("state"))?,
                );
            }
            "messageTopic" => {
                message_topic =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("messageTopic"))?);
            }
            "messageBroker" => {
                message_broker = Some(
                    reference_from_jsonable(val)
                        .map_err(|e| e.prepend_property("messageBroker"))?,
                );
            }
            "lastUpdate" => {
                last_update =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("lastUpdate"))?);
            }
            "minInterval" => {
                min_interval =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("minInterval"))?);
            }
            "maxInterval" => {
                max_interval =
                    Some(str_from_jsonable(val).map_err(|e| e.prepend_property("maxInterval"))?);
            }
            _ => {}
        }
    }
    Ok(BasicEventElement {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        observed: observed.ok_or_else(|| {
            DeserializationError::new("The required property 'observed' is missing")
        })?,
        direction: direction.ok_or_else(|| {
            DeserializationError::new("The required property 'direction' is missing")
        })?,
        state: state
            .ok_or_else(|| DeserializationError::new("The required property 'state' is missing"))?,
        message_topic,
        message_broker,
        last_update,
        min_interval,
        max_interval,
    })
}

fn operation_from_jsonable(v: &Value) -> Result<Operation, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    let mut input_variables: Option<Vec<OperationVariable>> = None;
    let mut output_variables: Option<Vec<OperationVariable>> = None;
    let mut inoutput_variables: Option<Vec<OperationVariable>> = None;
    for (k, val) in obj {
        if parse_sme_common_field(k, val, &mut sme)? {
            continue;
        }
        match k.as_str() {
            "inputVariables" => {
                input_variables = Some(parse_operation_variable_array(val, "inputVariables")?);
            }
            "outputVariables" => {
                output_variables = Some(parse_operation_variable_array(val, "outputVariables")?);
            }
            "inoutputVariables" => {
                inoutput_variables =
                    Some(parse_operation_variable_array(val, "inoutputVariables")?);
            }
            _ => {}
        }
    }
    Ok(Operation {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
        input_variables,
        output_variables,
        inoutput_variables,
    })
}

fn capability_from_jsonable(v: &Value) -> Result<Capability, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let mut sme = new_sme_fields();
    for (k, val) in obj {
        parse_sme_common_field(k, val, &mut sme)?;
    }
    Ok(Capability {
        extensions: sme.extensions,
        category: sme.category,
        id_short: sme.id_short,
        display_name: sme.display_name,
        description: sme.description,
        semantic_id: sme.semantic_id,
        supplemental_semantic_ids: sme.supplemental_semantic_ids,
        qualifiers: sme.qualifiers,
        embedded_data_specifications: sme.embedded_data_specifications,
    })
}

// ── Data specification content dispatcher ────────────────────────────────────

/// Deserialize an [`IDataSpecificationContent`] variant from a JSON value.
fn data_spec_content_from_jsonable(v: &Value) -> Result<Class, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let model_type_val = obj
        .get("modelType")
        .ok_or_else(|| DeserializationError::new("The required property modelType is missing"))?;
    let model_type_str = model_type_val
        .as_str()
        .ok_or_else(|| DeserializationError::new("Expected a string for modelType"))?;

    match model_type_str {
        "DataSpecificationIec61360" => {
            data_specification_iec61360_from_jsonable(v).map(Class::DataSpecificationIec61360)
        }
        other => Err(DeserializationError::new(format!(
            "Unexpected model type for IDataSpecificationContent: {other}"
        ))),
    }
}

// ── Public dispatch: class_from_jsonable ──────────────────────────────────────

/// Deserialize any [`Class`] variant from a JSON value.
///
/// Reads the `"modelType"` field to determine which concrete type to parse.
pub fn class_from_jsonable(v: &Value) -> Result<Class, DeserializationError> {
    let obj = v.as_object().ok_or_else(|| {
        if v.is_null() {
            DeserializationError::new("Expected a JSON object, but got null")
        } else {
            DeserializationError::new(format!(
                "Expected a JSON object, but got: {}",
                json_type_name(v)
            ))
        }
    })?;
    let model_type_val = obj
        .get("modelType")
        .ok_or_else(|| DeserializationError::new("The required property modelType is missing"))?;
    let model_type_str = model_type_val
        .as_str()
        .ok_or_else(|| DeserializationError::new("Expected a string for modelType"))?;

    match model_type_str {
        "Extension" => extension_from_jsonable(v).map(Class::Extension),
        "AdministrativeInformation" => {
            administrative_information_from_jsonable(v).map(Class::AdministrativeInformation)
        }
        "Qualifier" => qualifier_from_jsonable(v).map(Class::Qualifier),
        "AssetAdministrationShell" => {
            asset_administration_shell_from_jsonable(v).map(Class::AssetAdministrationShell)
        }
        "AssetInformation" => asset_information_from_jsonable(v).map(Class::AssetInformation),
        "Resource" => resource_from_jsonable(v).map(Class::Resource),
        "SpecificAssetId" => specific_asset_id_from_jsonable(v).map(Class::SpecificAssetId),
        "Submodel" => submodel_from_jsonable(v).map(Class::Submodel),
        "RelationshipElement" => {
            relationship_element_from_jsonable(v).map(Class::RelationshipElement)
        }
        "SubmodelElementList" => {
            submodel_element_list_from_jsonable(v).map(Class::SubmodelElementList)
        }
        "SubmodelElementCollection" => {
            submodel_element_collection_from_jsonable(v).map(Class::SubmodelElementCollection)
        }
        "Property" => property_from_jsonable(v).map(Class::Property),
        "MultiLanguageProperty" => {
            multi_language_property_from_jsonable(v).map(Class::MultiLanguageProperty)
        }
        "Range" => range_from_jsonable(v).map(Class::Range),
        "ReferenceElement" => reference_element_from_jsonable(v).map(Class::ReferenceElement),
        "Blob" => blob_from_jsonable(v).map(Class::Blob),
        "File" => file_from_jsonable(v).map(Class::File),
        "AnnotatedRelationshipElement" => {
            annotated_relationship_element_from_jsonable(v).map(Class::AnnotatedRelationshipElement)
        }
        "Entity" => entity_from_jsonable(v).map(Class::Entity),
        "EventPayload" => event_payload_from_jsonable(v).map(Class::EventPayload),
        "BasicEventElement" => basic_event_element_from_jsonable(v).map(Class::BasicEventElement),
        "Operation" => operation_from_jsonable(v).map(Class::Operation),
        "OperationVariable" => operation_variable_from_jsonable(v).map(Class::OperationVariable),
        "Capability" => capability_from_jsonable(v).map(Class::Capability),
        "ConceptDescription" => concept_description_from_jsonable(v).map(Class::ConceptDescription),
        "Reference" => reference_from_jsonable(v).map(Class::Reference),
        "Key" => key_from_jsonable(v).map(Class::Key),
        "LangStringNameType" => {
            lang_string_name_type_from_jsonable(v).map(Class::LangStringNameType)
        }
        "LangStringTextType" => {
            lang_string_text_type_from_jsonable(v).map(Class::LangStringTextType)
        }
        "Environment" => environment_from_jsonable(v).map(Class::Environment),
        "EmbeddedDataSpecification" => {
            embedded_data_specification_from_jsonable(v).map(Class::EmbeddedDataSpecification)
        }
        "LevelType" => level_type_from_jsonable(v).map(Class::LevelType),
        "ValueReferencePair" => {
            value_reference_pair_from_jsonable(v).map(Class::ValueReferencePair)
        }
        "ValueList" => value_list_from_jsonable(v).map(Class::ValueList),
        "LangStringPreferredNameTypeIec61360" => {
            lang_string_preferred_name_type_iec61360_from_jsonable(v)
                .map(Class::LangStringPreferredNameTypeIec61360)
        }
        "LangStringShortNameTypeIec61360" => lang_string_short_name_type_iec61360_from_jsonable(v)
            .map(Class::LangStringShortNameTypeIec61360),
        "LangStringDefinitionTypeIec61360" => lang_string_definition_type_iec61360_from_jsonable(v)
            .map(Class::LangStringDefinitionTypeIec61360),
        "DataSpecificationIec61360" => {
            data_specification_iec61360_from_jsonable(v).map(Class::DataSpecificationIec61360)
        }
        other => Err(DeserializationError::new(format!(
            "Unexpected model type for ISubmodelElement: {other}"
        ))),
    }
}

/// Deserialize an [`Environment`] from a JSON value (public entry point).
pub fn environment_from_jsonable_value(v: &Value) -> Result<Environment, DeserializationError> {
    environment_from_jsonable(v)
}

// Suppress unused warning for int64/float64 helpers when not referenced elsewhere
#[allow(dead_code)]
fn _use_int64(v: &Value) -> Result<i64, DeserializationError> {
    int64_from_jsonable(v)
}

#[allow(dead_code)]
fn _use_float64(v: &Value) -> Result<f64, DeserializationError> {
    float64_from_jsonable(v)
}
