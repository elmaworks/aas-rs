//! String constraint verification functions for AAS types.

#![allow(dead_code)]

use super::error::VerificationError;
use super::pattern;
use super::value;

const CONSTRAINT_AAS_D_130: &str = "Constraint AASd-130: An attribute with data type 'string' \
     shall consist of these characters only: \
     ^[\\x09\\x0A\\x0D\\x20-\\uD7FF\\uE000-\\uFFFD\\U00010000-\\U0010FFFF]*$.";

/// Verify Constraint AASd-130 (XML-serializable string).
pub fn verify_xml_serializable_string(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    errors
}

/// Verify non-empty XML-serializable string.
pub fn verify_non_empty_xml_serializable_string(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    errors
}

/// Verify xs:dateTime with UTC timezone.
pub fn verify_date_time_utc(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xs_date_time_utc(that) {
        errors.push(VerificationError::new(
            "The value must match the pattern of xs:dateTime with the time zone fixed to UTC.",
        ));
    }
    if !value::is_xs_date_time_utc(that) {
        errors.push(VerificationError::new(
            "The value must represent a valid xs:dateTime with the time zone fixed to UTC.",
        ));
    }
    errors
}

/// Verify xs:duration.
pub fn verify_duration(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xs_duration(that) {
        errors.push(VerificationError::new(
            "The value must match the pattern of xs:duration.",
        ));
    }
    errors
}

/// Verify blob type (no verification specified).
pub fn verify_blob_type(_that: &[u8]) -> Vec<VerificationError> {
    Vec::new()
}

/// Verify identifier (XML-serializable, non-empty, max 2000 chars).
pub fn verify_identifier(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 2000 {
        errors.push(VerificationError::new(
            "Identifier shall have a maximum length of 2000 characters.",
        ));
    }
    errors
}

/// Verify value type IEC 61360 (XML-serializable, non-empty, max 2000 chars).
pub fn verify_value_type_iec61360(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 2000 {
        errors.push(VerificationError::new(
            "Value type IEC 61360 shall have a maximum length of 2000 characters.",
        ));
    }
    errors
}

/// Verify name type (XML-serializable, non-empty, max 128 chars).
pub fn verify_name_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 128 {
        errors.push(VerificationError::new(
            "Name type shall have a maximum length of 128 characters.",
        ));
    }
    errors
}

/// Verify version type (XML-serializable, non-empty, pattern match, max 4 chars).
pub fn verify_version_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if !pattern::matches_version_type(that) {
        errors.push(VerificationError::new(
            "Version type shall match the version pattern.",
        ));
    }
    if that.len() > 4 {
        errors.push(VerificationError::new(
            "Version type shall have a maximum length of 4 characters.",
        ));
    }
    errors
}

/// Verify revision type (XML-serializable, non-empty, pattern match, max 4 chars).
pub fn verify_revision_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if !pattern::matches_revision_type(that) {
        errors.push(VerificationError::new(
            "Revision type shall match the revision pattern.",
        ));
    }
    if that.len() > 4 {
        errors.push(VerificationError::new(
            "Revision type shall have a maximum length of 4 characters.",
        ));
    }
    errors
}

/// Verify label type (XML-serializable, non-empty, max 64 chars).
pub fn verify_label_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 64 {
        errors.push(VerificationError::new(
            "Label type shall have a maximum length of 64 characters.",
        ));
    }
    errors
}

/// Verify message topic type (XML-serializable, non-empty, max 255 chars).
pub fn verify_message_topic_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 255 {
        errors.push(VerificationError::new(
            "Message topic type shall have a maximum length of 255 characters.",
        ));
    }
    errors
}

/// Verify BCP 47 language tag.
pub fn verify_bcp47_language_tag(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_bcp47(that) {
        errors.push(VerificationError::new(
            "The value must represent a value language tag conformant to BCP 47.",
        ));
    }
    errors
}

/// Verify content type (XML-serializable, non-empty, max 100 chars, MIME type pattern).
pub fn verify_content_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 100 {
        errors.push(VerificationError::new(
            "Content type shall have a maximum length of 100 characters.",
        ));
    }
    if !pattern::matches_mime_type(that) {
        errors.push(VerificationError::new(
            "The value must represent a valid content MIME type according to RFC 2046.",
        ));
    }
    errors
}

/// Verify path type (XML-serializable, non-empty, max 2000 chars, RFC 2396 pattern).
pub fn verify_path_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 2000 {
        errors.push(VerificationError::new(
            "Identifier shall have a maximum length of 2000 characters.",
        ));
    }
    if !pattern::matches_rfc2396(that) {
        errors.push(VerificationError::new(
            "String with max 2048 and min 1 characters conformant to a URI as per RFC 2396.",
        ));
    }
    errors
}

/// Verify qualifier type (XML-serializable, non-empty, max 128 chars).
pub fn verify_qualifier_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 128 {
        errors.push(VerificationError::new(
            "Name type shall have a maximum length of 128 characters.",
        ));
    }
    errors
}

/// Verify value data type (XML-serializable).
pub fn verify_value_data_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    errors
}

/// Verify ID-short type (XML-serializable, non-empty, max 128 chars, ID-short pattern).
pub fn verify_id_short_type(that: &str) -> Vec<VerificationError> {
    let mut errors = Vec::new();
    if !pattern::matches_xml_serializable_string(that) {
        errors.push(VerificationError::new(CONSTRAINT_AAS_D_130));
    }
    if that.is_empty() {
        errors.push(VerificationError::new("The value must not be empty."));
    }
    if that.len() > 128 {
        errors.push(VerificationError::new(
            "Name type shall have a maximum length of 128 characters.",
        ));
    }
    if !pattern::matches_id_short(that) {
        errors.push(VerificationError::new(
            "ID-short of Referables shall only feature letters, digits, \
             underscore (`_`); starting mandatory with a letter. \
             *I.e.* `[a-zA-Z][a-zA-Z0-9_]*`.",
        ));
    }
    errors
}
