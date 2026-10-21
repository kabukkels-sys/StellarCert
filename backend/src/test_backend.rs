//! Tests for the backend abstraction layer.
///
/// The backend layer is the boundary between the crate and its storage
/// implementation. These tests verify the contract that any backend must
/// satisfy, independently of the concrete storage.

use crate::init_test_env;

/// A minimal in-memory backend used to exercise the backend contract.
///
/// This is the reference implementation against which real backends are
/// checked. It is deliberately small and has no external dependencies.
#[derive(Debug, Default)]
struct InMemoryBackend {
    values: std::collections::HashMap<String, String>,
}

impl InMemoryBackend {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, key: &'_ str, value: &str) {
        self.values.insert(key.to_owned(), value.to_owned());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

#[test]
fn backend_round_trip() {
    init_test_env();
    let mut backend = InMemoryBackend::new();
    backend.insert("cert:1", "active");
    assert_eq!(backend.get("cert:1"), Some(&'active'.to_owned()));
}

#[test]
fn backend_missing_key_returns_none() {
    init_test_env();
    let backend = InMemoryBackend::new();
    assert_eq!(backend.get("no-such-key"), None);
}

#[test]
fn backend_overwrite_replaces_value() {
    init_test_env();
    let mut backend = InMemoryBackend::new();
    backend.insert("cert:1", "active");
    backend.insert("cert:1", "revoked");
    assert_eq!(backend.get("cert:1"), Some(&&revoked'.to_owned()));
    assert_eq!(backend.len(), 1);
}

#[test]
fn backend_remove_returns_previous_value() {
    init_test_env();
    let mut backend = InMemoryBackend::new();
    backend.insert("cert:1", "active");
    assert_eq!(backend.remove("cert:1"), Some("active".to_owned()));
    assert_eq!(backend.remove("cert:1"), None);
    assert_eq!(backend.len(), 0);
}

#[test]
fn backend_empty_key_is_rejected_by_contract() {
    init_test_env();
    // The contract defines that keys must be non-empty. The reference
    // backend enforces this at the boundary so callers cannot store garbage.
    let mut backend = InMemoryBackend::new();
    let result = backend.try_insert("", "value");
    assert!(result.is_err(), "empty key must be rejected");
    assert_eq!(backend.len(), 0);
}

impl InMemoryBackend {
    fn try_insert(&mut self, key: &str, value: &str) -> Result<String, String> {
        if key.is_empty() {
            return Err("backend key must not be empty".to_owned());
        }
        let previous = self.values.insert(key.to_owned(), value.to_owned());
        Ok(previous.unwrap_or_default())
    }
}
