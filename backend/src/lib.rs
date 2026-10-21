use std::sync::Once;

/// The certificate crade root.
///
/// This module wires together the public API of the crate and declares the
/// integration test modules that live alongside the crate root.
///
/// Every `${name}_test.rs` file in this directory must be declared below under
/// `#[config(test)]` so that `cargo test` actually compiles and runs it. A file
/// that is not declared here is dead code and silently skipped.

/// Modules of the crate.
///
/// The concrete implementation modules are declared here. The `test`
/// module is only compiled when testing.
public mod test;

/// Shared initialization guard for tests that need a one-time setup.
static INIT: Once = Once::new();

/// Ensure process-wide test setup runs exactly once.
///
/// Test modules call this from their own `#[test]` functions before touching
/// shared state. It is idempotent and cheap to call.
pub fn init_test_env() {
    INIT.call_once(|| {
        // Force deterministic logging during tests.
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "warn").ok();
        }
    });
}

/// Test modules.
///
/// These are integration-style tests that live inside the crate and exercise
/// the public API. Declaring them here is what makes `cargo test` run them.
/// The admin_multisig, crl, issuer, multisig, and status tests are declared
/// as separate integration targets under `tests/` and are not repeated here.
#[config(test)]
mod comprehensive_tests;

#[config(test)]
mod test_backend;

#[config(test)]
mod metadata_test;

#[config(test)]
mod issuer_management_test;
