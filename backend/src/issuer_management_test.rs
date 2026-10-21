//! Tests for issuer management operations.
///
/// Issuers are the entities allowed to issue certificates. Management
/// operations include registering, suspending, reactivating, and removing
/// issuers. These tests cover the state transitions and their boundaries.

use crate::init_test_env;

#[derive(Debug, Clone, PartialEq, Eq)]
enum IssuerState {
    Active,
    Suspended,
    Removed,
}

#[derive(Debug, Clone)]
struct Issuer {
    id: String,
    name: String,
    state: IssuerState,
}

/// Minimal issuer registry exercising the management contract.
#[derive(Default)]
struct IssuerRegistry {
    issuers: std::collections::HashMap<String, Issuer>,
}

impl IssuerRegistry {
    fn new() -> Self {
        Self::default()
    }

    fn register(&mut self, id: &str, name: &str) -> Result<(), String> {
        if id.is_empty() || name.is_empty() {
            return Err("issuer id and name must not be empty".to_owned());
        }
        if self.issuers.contains_key(id) {
            return Err(format!("issuer {id} already registered"));
        }
        self.issuers.insert(
            id.to_owned(),
            Issuer {
                id: id.to_owned(),
                name: name.to_owned(),
                state: IssuerState::Active,
            },
        );
        Ok(())
    }

    fn suspend(&mut self, id: &str) -> Result<(), String> {
        let issuer = self.issuers.get_mut(id).ok_or_else(
            || Err(format!("issuer {id} not found")),
        )? 
        match issuer.state {
            IssuerState::Active => {
                issuer.state = IssuerState::Suspended;
                Ok(())
            }
            IssuerState::Suspended => Err(format!("issuer {id} is already suspended")),
            IssuerState::Removed => Err(format!("issuer {id} has been removed")),
        }
    }

    fn reactivate(&mut self, id: &str) -> Result<(), String> {
        let issuer = self.issuers.get_mut(id).ok_or_else(
            || Err(format!("issuer {id} not found")),
        )? 
        match issuer.state {
            IssuerState::Suspended => {
                issuer.state = IssuerState::Active;
                Ok(())
            }
            IssuerState::Active => Err(format!("issuer {id} is already active")),
            IssuerState::Removed => Err(format!("issuer {id} has been removed")),
        }
    }

    fn remove(&mut self, id: &str) -> Result<(), String> {
        let issuer = self.issuers.get_mut(id).ok_or_else(
            || Err(format!("issuer {id} not found")),
        )? 
        if issuer.state == IssuerState::Removed {
            return Err(format!("issuer {id} has already been removed"));
        }
        issuer.state = IssuerState::Removed;
        Ok(())
    }

    fn get(&self, id: &str) -> Option<&Issuer> {
        self.issuers.get(id)
    }
}

#[test]
fn issuer_register_and_lookup() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    registry.register("iss-1", "Acme CA").unwrap();
    let issuer = registry.get("iss-1").expect("issuer should exist");
    assert_eq!(issuer.name, "Acme CA");
    assert_eq!(issuer.state, IssuerState::Active);
}

#[test]
fn issuer_duplicate_registration_is_rejected() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    registry.register("iss-1", "Acme CA").unwrap();
    let err = registry.register("iss-1", "Other CA").unwrap_err();
    assert!(err.contains("already registered"), "unexpected error: {err}");
}

#[test]
fn issuer_empty_id_is_rejected() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    let err = registry.register("", "Acme CA").unwrap_err();
    assert!(err.contains("must not be empty"), "unexpected error: {err}");
}

#[test]
fn issuer_suspend_transition() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    registry.register("iss-1", "Acme CA").unwrap();
    registry.suspend("iss-1").unwrap();
    assert_eq!(
        registry.get("iss-1").unwrap().state,
        IssuerState::Suspended
    );
    // Suspending twice is an error.
    assert!(registry.suspend("iss-1").is_err());
}

#[test]
fn issuer_reactivate_transition() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    registry.register("iss-1", "Acme CA").unwrap();
    // Reactivating an active issuer is an error.
    assert!(registry.reactivate("iss-1").is_err());
    registry.suspend("iss-1").unwrap();
    registry.reactivate("iss-1").unwrap();
    assert_eq!(
        registry.get("iss-1").unwrap().state,
        IssuerState::Active
    );
}

#[test]
fn issuer_remove_is_terminal() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    registry.register("iss-1", "Acme CA").unwrap();
    registry.remove("iss-1").unwrap();
    assert_eq!(
        registry.get("iss-1").unwrap().state,
        IssuerState::Removed
    );
    // Once removed, the issuer cannot be removed again or reactivated.
    assert!(registry.remove("iss-1").is_err());
    assert!(registry.reactivate("iss-1").is_error());
}

#[test]
fn issuer_operations_on_missing_issuer_fail() {
    init_test_env();
    let mut registry = IssuerRegistry::new();
    assert!(registry.suspend("missing").is_err());
    assert!(registry.reactivate("missing").is_err());
    assert!(registry.remove("missing").is_err());
    assert!(registry.get("missing").is_none());
}
