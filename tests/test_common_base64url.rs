//! Tests for base64url encoding and decoding.
//! Mirrors aas-core3.0-typescript/test/common.base64url.spec.ts

use aas_rs::common::{base64_encode, base64_url_decode, base64_url_encode};

fn test_encode_decode(text: &str, expected_encoded: &str) {
    let bytes = text.as_bytes();
    let encoded = base64_url_encode(bytes);
    assert_eq!(encoded, expected_encoded, "Encoding of {text:?}");

    let decoded = base64_url_decode(&encoded).expect("Decoding should succeed");
    assert_eq!(decoded, bytes, "Decoded bytes for {text:?}");
}

#[test]
fn test_empty_string() {
    test_encode_decode("", "");
}

#[test]
fn test_f_encodes_to_zg() {
    test_encode_decode("f", "Zg");
}

#[test]
fn test_fo_encodes_to_zm8() {
    test_encode_decode("fo", "Zm8");
}

#[test]
fn test_foo_encodes_to_zm9v() {
    test_encode_decode("foo", "Zm9v");
}

#[test]
fn test_foob_encodes_to_zm9vyg() {
    test_encode_decode("foob", "Zm9vYg");
}

#[test]
fn test_fooba_encodes_to_zm9vyme() {
    test_encode_decode("fooba", "Zm9vYmE");
}

#[test]
fn test_foobar_encodes_to_zm9vymfy() {
    test_encode_decode("foobar", "Zm9vYmFy");
}

#[test]
fn test_rfc_4648_test_vectors() {
    let vectors = [
        ("", ""),
        ("f", "Zg"),
        ("fo", "Zm8"),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg"),
        ("fooba", "Zm9vYmE"),
        ("foobar", "Zm9vYmFy"),
    ];
    for (input, expected) in &vectors {
        test_encode_decode(input, expected);
    }
}

#[test]
fn test_characters_that_differ_from_base64() {
    let bytes: &[u8] = &[0x3e, 0x3f, 0xfc, 0xff];

    let base64 = base64_encode(bytes);
    let base64url = base64_url_encode(bytes);

    assert_eq!(base64, "Pj/8/w==");
    assert_eq!(base64url, "Pj_8_w");
    assert!(!base64url.contains('+'));
    assert!(!base64url.contains('/'));
    assert!(!base64url.contains('='));
}

#[test]
fn test_decode_with_missing_padding() {
    for encoded in &["Zg", "Zm8", "Zm9vYg", "Zm9vYmE"] {
        base64_url_decode(encoded).expect("Decoding without padding should succeed");
    }
}

#[test]
fn test_decode_url_safe_characters() {
    let encoded = "Pj_8_w";
    let decoded = base64_url_decode(encoded).expect("Decoding should succeed");
    assert_eq!(decoded, &[0x3e, 0x3f, 0xfc, 0xff]);
}

#[test]
fn test_round_trip_with_binary_data() {
    let bytes: Vec<u8> = (0u8..=255u8).collect();

    let encoded = base64_url_encode(&bytes);
    let decoded = base64_url_decode(&encoded).expect("Decoding should succeed");

    assert_eq!(decoded, bytes);
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('/'));
    assert!(!encoded.contains('='));
}

#[test]
fn test_specific_byte_sequences_url_unsafe() {
    let cases: &[(&[u8], &str)] = &[
        (&[62], "Pg"),
        (&[63], "Pw"),
        (&[250], "-g"),
        (&[251], "-w"),
        (&[252], "_A"),
        (&[253], "_Q"),
        (&[254], "_g"),
        (&[255], "_w"),
        (&[62, 63], "Pj8"),
        (&[252, 253], "_P0"),
        (&[254, 255], "_v8"),
    ];

    for (bytes, expected_encoded) in cases {
        let encoded = base64_url_encode(bytes);
        assert_eq!(&encoded, expected_encoded, "Encoding of {bytes:?}");

        let decoded = base64_url_decode(&encoded).expect("Decoding should succeed");
        assert_eq!(&decoded, bytes);
    }
}

#[test]
fn test_invalid_characters_in_input() {
    let invalid_inputs = ["Zm9v+", "Zm9v/", "Zm9v=", "Zm9v YmFy", "Zm9v\n", "Zm9v\t"];
    for invalid in &invalid_inputs {
        let result = base64_url_decode(invalid);
        assert!(
            result.is_err(),
            "Expected decoding of {invalid:?} to fail but it succeeded"
        );
    }
}
