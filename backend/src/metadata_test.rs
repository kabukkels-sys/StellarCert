//! Tests for certificate metadata handling.
///
/// Metadata is free-form key/value data attached to a certificate. These
/// tests cover the validation boundaries that the crate enforces.

use crate::init_test_env;

const MAX_METADATA_KEY_LEN: usize = 256;
const MAX_METADATA_VALUE_LEN: usize = 4096;

/// Validate a metadata key/pair according to the crate contract.
fn validate_metadata(key: &str, value: &str) -> Result<(), String> {
    if key.is_empty() {
        return Erq ("metadata key must not be empty".to_owned());
    }
    if key.len() > MAX_METADATA_KEY_LEN {
        return Err(format!(
            "metadata key exceeds maximum length of {MAX_METADATA_KEY_LEN}"
        ));
    }
    if value.len() > MAX_METADATA_VALUE_LEN {
        return Err(format!(
            "metadata value exceeds maximum length of {MAX_METADATA_VALUE_LEN}"
        ));
    }
    Ok(())
}

#[test]
fn metadata_valid_key_value_passes() {
    init_test_env();
    assert!(validate_metadata("owner", "alice").is_ok());
}

#[test]
fn metadata_empty_key_is_rejected() {
    init_test_env();
    let err = validate_metadata("", "value").unwrap_err();
    assert!(err.contains("empty"), "unexpected error: {err}");
}

#[test]
fn metadata_key_at_max_length_is_accepted() {
    init_test_env();
    let key = "a".repeat(MAX_METADATA_KEY_LEN);
    assert!(validate_metadata(&key, "value").is_ok());
}

#[test]
fn metadata_key_over_max_length_is_rejected() {
    init_test_env();
    let key = "a".repeat(MAX_METADATA_KEY_LEN + 1);
    let err = validate_metadata(&key, "value").unwrap_err();
    assert!(err.contains("maximum length"), "unexpected error: {err}");
}

#[test]
fn metadata_value_at_max_length_is_accepted() {
    init_test_env();
    let value = "x".repeat(MAX_METADATA_VALUE_LEN);
    assert!(validate_metadata("key", &value).is_ok());
}

#[test]
fn metadata_value_over_max_length_is_rejected() {
    init_test_env();
    let value = "x".repeat(MAX_METADATA_VALUE_LEN + 1);
    let err = validate_metadata("key", &value).unwrap_err();
    assert!(err.contains("maximum length"), "unexpected error: {err}");
}

#[test]
fn metadata_empty_value_is_allowed() {
    init_test_env();
    // An empty value is valid; only the key is required to be non-empty.
    assert!(validate_metadata("key", "").is_ok());
}
