// This code has been ported from aas-core3.0-typescript/src/common.ts.

use base64::{engine::general_purpose, Engine as _};

/// Encode a byte slice as a standard base64 string (with padding).
///
/// # Examples
///
/// ```
/// let encoded = aas_rs::common::base64_encode(b"hello");
/// assert_eq!(encoded, "aGVsbG8=");
/// ```
pub fn base64_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

/// Decode a standard base64-encoded string into bytes.
///
/// Returns `Ok(bytes)` on success, or `Err(message)` if the input is not
/// valid base64.
///
/// # Examples
///
/// ```
/// let bytes = aas_rs::common::base64_decode("aGVsbG8=").unwrap();
/// assert_eq!(bytes, b"hello");
/// ```
pub fn base64_decode(text: &str) -> Result<Vec<u8>, String> {
    general_purpose::STANDARD
        .decode(text)
        .map_err(|e| format!("Invalid base64 encoding: {e}"))
}

/// Encode a byte slice as a base64url string (no padding, URL-safe alphabet).
///
/// The characters `+` and `/` are replaced with `-` and `_`, and padding
/// (`=`) is omitted so the result can be used inside URIs without escaping.
///
/// # Examples
///
/// ```
/// let encoded = aas_rs::common::base64_url_encode(b"hello");
/// assert_eq!(encoded, "aGVsbG8");
/// ```
pub fn base64_url_encode(bytes: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Decode a base64url-encoded string into bytes.
///
/// Accepts both padded and unpadded inputs and the URL-safe alphabet
/// (`-` and `_` instead of `+` and `/`).
///
/// Returns `Ok(bytes)` on success, or `Err(message)` if the input is not
/// valid base64url.
///
/// # Examples
///
/// ```
/// let bytes = aas_rs::common::base64_url_decode("aGVsbG8").unwrap();
/// assert_eq!(bytes, b"hello");
/// ```
pub fn base64_url_decode(text: &str) -> Result<Vec<u8>, String> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(text)
        .map_err(|e| format!("Invalid base64url encoding: {e}"))
}
