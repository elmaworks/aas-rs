//! Tests for `*OrDefault` property defaults.
//! Mirrors aas-core3.0-typescript/test/types.xOrDefault.spec.ts

mod common;

use aas_rs::stringification::{
    must_data_type_def_xsd_to_str, must_modelling_kind_to_str, must_qualifier_kind_to_str,
};

fn assert_golden(got: &str, type_name: &str, file_name: &str) {
    let golden_path = common::test_data_dir()
        .join("xOrDefault")
        .join(type_name)
        .join(file_name);
    let expected_raw = std::fs::read_to_string(&golden_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", golden_path.display()));
    // Golden files may be JSON-encoded strings (e.g., `"VARIABLE"`) or plain strings.
    // We normalize: if it starts/ends with `"`, parse as JSON string; otherwise use as-is.
    let expected = expected_raw.trim();
    let expected = if expected.starts_with('"') {
        serde_json::from_str::<String>(expected).unwrap_or_else(|_| expected.to_string())
    } else {
        expected.to_string()
    };
    assert_eq!(got, &expected, "Mismatch for {type_name}/{file_name}");
}

// ── Extension.valueTypeOrDefault ─────────────────────────────────────────────

#[test]
fn test_extension_value_type_or_default_non_default() {
    let instance = common::load_maximal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    let vt = ext.value_type.unwrap_or(aas_rs::DataTypeDefXsd::String);
    let got = format!("DataTypeDefXsd.{}", must_data_type_def_xsd_to_str(vt));
    assert_golden(&got, "Extension", "valueTypeOrDefault.non-default.json");
}

#[test]
fn test_extension_value_type_or_default_default() {
    let instance = common::load_minimal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    let vt = ext.value_type.unwrap_or(aas_rs::DataTypeDefXsd::String);
    let got = format!("DataTypeDefXsd.{}", must_data_type_def_xsd_to_str(vt));
    assert_golden(&got, "Extension", "valueTypeOrDefault.default.json");
}

// ── Submodel.kindOrDefault ────────────────────────────────────────────────────

#[test]
fn test_submodel_kind_or_default_non_default() {
    let instance = common::load_maximal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    let kind = sm.kind.unwrap_or(aas_rs::ModellingKind::Instance);
    let got = format!("ModellingKind.{}", must_modelling_kind_to_str(kind));
    assert_golden(&got, "Submodel", "kindOrDefault.non-default.json");
}

#[test]
fn test_submodel_kind_or_default_default() {
    let instance = common::load_minimal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    let kind = sm.kind.unwrap_or(aas_rs::ModellingKind::Instance);
    let got = format!("ModellingKind.{}", must_modelling_kind_to_str(kind));
    assert_golden(&got, "Submodel", "kindOrDefault.default.json");
}

// ── Qualifier.kindOrDefault ──────────────────────────────────────────────────

#[test]
fn test_qualifier_kind_or_default_non_default() {
    let instance = common::load_maximal_of("Qualifier", |c| c.is_qualifier());
    let q = instance.as_qualifier().unwrap();
    let kind = q.kind.unwrap_or(aas_rs::QualifierKind::ConceptQualifier);
    let got = format!("QualifierKind.{}", must_qualifier_kind_to_str(kind));
    assert_golden(&got, "Qualifier", "kindOrDefault.non-default.json");
}

#[test]
fn test_qualifier_kind_or_default_default() {
    let instance = common::load_minimal_of("Qualifier", |c| c.is_qualifier());
    let q = instance.as_qualifier().unwrap();
    let kind = q.kind.unwrap_or(aas_rs::QualifierKind::ConceptQualifier);
    let got = format!("QualifierKind.{}", must_qualifier_kind_to_str(kind));
    assert_golden(&got, "Qualifier", "kindOrDefault.default.json");
}

// ── SubmodelElement categoryOrDefault ────────────────────────────────────────
// TypeScript default for category is "VARIABLE"

fn category_or_default(category: Option<&str>) -> &str {
    category.unwrap_or("VARIABLE")
}

#[test]
fn test_blob_category_or_default_non_default() {
    let instance = common::load_maximal_of("Blob", |c| c.is_blob());
    let b = instance.as_blob().unwrap();
    let got = category_or_default(b.category.as_deref());
    assert_golden(got, "Blob", "categoryOrDefault.non-default.json");
}

#[test]
fn test_blob_category_or_default_default() {
    let instance = common::load_minimal_of("Blob", |c| c.is_blob());
    let b = instance.as_blob().unwrap();
    let got = category_or_default(b.category.as_deref());
    assert_golden(got, "Blob", "categoryOrDefault.default.json");
}

#[test]
fn test_file_category_or_default_non_default() {
    let instance = common::load_maximal_of("File", |c| c.is_file());
    let f = instance.as_file().unwrap();
    let got = category_or_default(f.category.as_deref());
    assert_golden(got, "File", "categoryOrDefault.non-default.json");
}

#[test]
fn test_file_category_or_default_default() {
    let instance = common::load_minimal_of("File", |c| c.is_file());
    let f = instance.as_file().unwrap();
    let got = category_or_default(f.category.as_deref());
    assert_golden(got, "File", "categoryOrDefault.default.json");
}

#[test]
fn test_multi_language_property_category_or_default_non_default() {
    let instance =
        common::load_maximal_of("MultiLanguageProperty", |c| c.is_multi_language_property());
    let mlp = instance.as_multi_language_property().unwrap();
    let got = category_or_default(mlp.category.as_deref());
    assert_golden(
        got,
        "MultiLanguageProperty",
        "categoryOrDefault.non-default.json",
    );
}

#[test]
fn test_multi_language_property_category_or_default_default() {
    let instance =
        common::load_minimal_of("MultiLanguageProperty", |c| c.is_multi_language_property());
    let mlp = instance.as_multi_language_property().unwrap();
    let got = category_or_default(mlp.category.as_deref());
    assert_golden(
        got,
        "MultiLanguageProperty",
        "categoryOrDefault.default.json",
    );
}

#[test]
fn test_property_category_or_default_non_default() {
    let instance = common::load_maximal_of("Property", |c| c.is_property());
    let p = instance.as_property().unwrap();
    let got = category_or_default(p.category.as_deref());
    assert_golden(got, "Property", "categoryOrDefault.non-default.json");
}

#[test]
fn test_property_category_or_default_default() {
    let instance = common::load_minimal_of("Property", |c| c.is_property());
    let p = instance.as_property().unwrap();
    let got = category_or_default(p.category.as_deref());
    assert_golden(got, "Property", "categoryOrDefault.default.json");
}

#[test]
fn test_range_category_or_default_non_default() {
    let instance = common::load_maximal_of("Range", |c| c.is_range());
    let r = instance.as_range().unwrap();
    let got = category_or_default(r.category.as_deref());
    assert_golden(got, "Range", "categoryOrDefault.non-default.json");
}

#[test]
fn test_range_category_or_default_default() {
    let instance = common::load_minimal_of("Range", |c| c.is_range());
    let r = instance.as_range().unwrap();
    let got = category_or_default(r.category.as_deref());
    assert_golden(got, "Range", "categoryOrDefault.default.json");
}

#[test]
fn test_reference_element_category_or_default_non_default() {
    let instance = common::load_maximal_of("ReferenceElement", |c| c.is_reference_element());
    let re = instance.as_reference_element().unwrap();
    let got = category_or_default(re.category.as_deref());
    assert_golden(
        got,
        "ReferenceElement",
        "categoryOrDefault.non-default.json",
    );
}

#[test]
fn test_reference_element_category_or_default_default() {
    let instance = common::load_minimal_of("ReferenceElement", |c| c.is_reference_element());
    let re = instance.as_reference_element().unwrap();
    let got = category_or_default(re.category.as_deref());
    assert_golden(got, "ReferenceElement", "categoryOrDefault.default.json");
}

// ── SubmodelElementList.orderRelevantOrDefault ───────────────────────────────

#[test]
fn test_submodel_element_list_order_relevant_or_default_non_default() {
    let instance = common::load_maximal_of("SubmodelElementList", |c| c.is_submodel_element_list());
    let sel = instance.as_submodel_element_list().unwrap();
    let got = sel.order_relevant.unwrap_or(true).to_string();
    assert_golden(
        &got,
        "SubmodelElementList",
        "orderRelevantOrDefault.non-default.json",
    );
}

#[test]
fn test_submodel_element_list_order_relevant_or_default_default() {
    let instance = common::load_minimal_of("SubmodelElementList", |c| c.is_submodel_element_list());
    let sel = instance.as_submodel_element_list().unwrap();
    let got = sel.order_relevant.unwrap_or(true).to_string();
    assert_golden(
        &got,
        "SubmodelElementList",
        "orderRelevantOrDefault.default.json",
    );
}
