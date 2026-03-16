//! Common test helpers shared across all integration tests.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use aas_rs::jsonization;
use aas_rs::types::class::Class;
use aas_rs::{stringification, Environment};

/// Returns the path to the test data directory inside the TypeScript submodule.
pub fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("aas-core3.0-typescript")
        .join("test_data")
}

/// Parses JSON from a file, panicking on failure.
pub fn load_json(path: &Path) -> serde_json::Value {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("Failed to parse JSON from {}: {e}", path.display()))
}

/// Loads an `Environment` by deserializing the JSON at `path`.
pub fn load_environment(path: &Path) -> Environment {
    let v = load_json(path);
    jsonization::environment_from_jsonable_value(&v).unwrap_or_else(|e| {
        panic!(
            "Failed to deserialize environment from {}: {e}",
            path.display()
        )
    })
}

/// Finds the first `Class` in `start` (itself or descendants) satisfying `condition`.
///
/// Panics if none is found.
pub fn must_find<F: Fn(&Class) -> bool>(start: &Class, condition: F) -> Class {
    if condition(start) {
        return start.clone();
    }
    for instance in start.descend() {
        if condition(&instance) {
            return instance;
        }
    }
    panic!("No instance found satisfying the condition")
}

/// Collects all `.json` files recursively under `dir`, sorted.
pub fn find_json_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if !dir.exists() {
        return result;
    }
    collect_json_files(dir, &mut result);
    result.sort();
    result
}

fn collect_json_files(dir: &Path, result: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, result);
        } else if path.extension().map(|e| e == "json").unwrap_or(false) {
            result.push(path);
        }
    }
}

/// Returns a human-readable mark for a `Class` instance — the same format
/// as the TypeScript `traceMark()` helper.
pub fn trace_mark(instance: &Class) -> String {
    let type_name = stringification::must_model_type_to_str(instance.model_type());

    // Identifiables: include id
    let id: Option<&str> = match instance {
        Class::AssetAdministrationShell(x) => Some(&x.id),
        Class::Submodel(x) => Some(&x.id),
        Class::ConceptDescription(x) => Some(&x.id),
        _ => None,
    };
    if let Some(id) = id {
        return format!("{type_name} with ID {id:?}");
    }

    // Referables (IReferable in TypeScript = SubmodelElements + Identifiables).
    // Extension, AdministrativeInformation, AssetInformation, etc. are NOT IReferable.
    // id_short is always included for Referables even when None (shown as "null").
    let referable_id_short: Option<Option<&str>> = match instance {
        Class::RelationshipElement(x) => Some(x.id_short.as_deref()),
        Class::SubmodelElementList(x) => Some(x.id_short.as_deref()),
        Class::SubmodelElementCollection(x) => Some(x.id_short.as_deref()),
        Class::Property(x) => Some(x.id_short.as_deref()),
        Class::MultiLanguageProperty(x) => Some(x.id_short.as_deref()),
        Class::Range(x) => Some(x.id_short.as_deref()),
        Class::ReferenceElement(x) => Some(x.id_short.as_deref()),
        Class::Blob(x) => Some(x.id_short.as_deref()),
        Class::File(x) => Some(x.id_short.as_deref()),
        Class::AnnotatedRelationshipElement(x) => Some(x.id_short.as_deref()),
        Class::Entity(x) => Some(x.id_short.as_deref()),
        Class::BasicEventElement(x) => Some(x.id_short.as_deref()),
        Class::Operation(x) => Some(x.id_short.as_deref()),
        Class::Capability(x) => Some(x.id_short.as_deref()),
        _ => None,
    };
    if let Some(id_short) = referable_id_short {
        let id_short_repr = match id_short {
            Some(s) => format!("{s:?}"),
            None => "null".to_string(),
        };
        return format!("{type_name} with ID-short {id_short_repr}");
    }

    type_name.to_string()
}

/// Builds the descend-once trace for `instance` (newline-terminated).
pub fn descend_once_trace(instance: &Class) -> String {
    let mut lines: Vec<String> = instance.descend_once().map(|d| trace_mark(&d)).collect();
    lines.push(String::new()); // trailing newline
    lines.join("\n")
}

/// Builds the full descend trace for `instance` (newline-terminated).
pub fn descend_trace(instance: &Class) -> String {
    let mut lines: Vec<String> = instance.descend().map(|d| trace_mark(&d)).collect();
    lines.push(String::new()); // trailing newline
    lines.join("\n")
}

/// Reads expected trace from `golden_path` and asserts it equals `got`.
pub fn assert_trace_eq(got: &str, golden_path: &Path) {
    let expected = std::fs::read_to_string(golden_path)
        .unwrap_or_else(|e| panic!("Failed to read golden file {}: {e}", golden_path.display()))
        .replace("\r\n", "\n");
    assert_eq!(
        got,
        expected,
        "Trace mismatch for {}",
        golden_path.display()
    );
}

/// Loads an `Environment` from the ContainedInEnvironment/Expected/{type_name}/{example}.json
/// and finds the first instance satisfying `condition`.
pub fn load_minimal_of<F: Fn(&Class) -> bool>(type_name: &str, condition: F) -> Class {
    let path = test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Expected")
        .join(type_name)
        .join("minimal.json");
    let env = Class::Environment(load_environment(&path));
    must_find(&env, condition)
}

/// Loads an `Environment` from the ContainedInEnvironment/Expected/{type_name}/{example}.json
/// and finds the first instance satisfying `condition`.
pub fn load_maximal_of<F: Fn(&Class) -> bool>(type_name: &str, condition: F) -> Class {
    let path = test_data_dir()
        .join("Json")
        .join("ContainedInEnvironment")
        .join("Expected")
        .join(type_name)
        .join("maximal.json");
    let env = Class::Environment(load_environment(&path));
    must_find(&env, condition)
}

/// Verifies JSON round-trip: `original_json` → deserialize → serialize → compare.
///
/// Returns `None` on success or a descriptive error string.
pub fn check_json_equal(expected: &serde_json::Value, got: &serde_json::Value) -> Option<String> {
    if expected == got {
        None
    } else {
        Some(format!(
            "JSON mismatch:\nExpected: {}\nGot:      {}",
            serde_json::to_string_pretty(expected).unwrap_or_default(),
            serde_json::to_string_pretty(got).unwrap_or_default(),
        ))
    }
}
