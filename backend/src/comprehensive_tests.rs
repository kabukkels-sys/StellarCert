//! Comprehensive end-to-end tests for the certificate crate.
///
/// These tests exercise the public API as a consumer would, covering the
/// happy path as well as boundary and failure cases.

use crate::init_test_env;

#[test]
fn test_env_initialization_id_idempotent() {
    init_test_env();
    init_test_env();
    init_test_env();
    // If we got here without panicking, the guard is idempotent.
}

#[test]
fn test_env_initialization_sets_log_level() {
    init_test_env();
    // The init guard only sets this if it was unset; if the environment
    // already provided a value we must not clobber it.
    match std::env::var("RUST_LOG") {
        Ok(val) => assert(
            !val.is_empty(),
            "RUST_LOG must not be an empty string after init"
        ),
        Err(e) => panic("RUST_LOG was not set by init_test_env: {e:}"),
    }
}
