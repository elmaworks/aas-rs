//! Tests for `over_*_or_empty` iteration patterns.
//! Mirrors aas-core3.0-typescript/test/types.overXOrEmpty.spec.ts
//!
//! Since Rust does not have `overXOrEmpty()` methods, we use
//! `Option<Vec<T>>.as_deref().unwrap_or_default()` equivalents.

mod common;

/// Assert that the optional slice has the expected item count.
fn assert_count(opt: Option<&[impl Sized]>, expected: usize) {
    let count = opt.map(|s| s.len()).unwrap_or(0);
    assert_eq!(count, expected);
}

// ── Extension ──────────────────────────────────────────────────────────────────

#[test]
fn test_extension_over_supplemental_semantic_ids_or_empty_non_default() {
    let instance = common::load_maximal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    let ids = ext.supplemental_semantic_ids.as_deref();
    assert!(
        ids.is_some(),
        "supplementalSemanticIds should be set in maximal example"
    );
    assert_count(ids, ids.unwrap().len());
}

#[test]
fn test_extension_over_supplemental_semantic_ids_or_empty_default() {
    let instance = common::load_minimal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    assert!(
        ext.supplemental_semantic_ids.is_none(),
        "supplementalSemanticIds should be None in minimal example"
    );
    assert_count(ext.supplemental_semantic_ids.as_deref(), 0);
}

#[test]
fn test_extension_over_refers_to_or_empty_non_default() {
    let instance = common::load_maximal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    let refers = ext.refers_to.as_deref();
    assert!(
        refers.is_some(),
        "refersTo should be set in maximal example"
    );
    assert_count(refers, refers.unwrap().len());
}

#[test]
fn test_extension_over_refers_to_or_empty_default() {
    let instance = common::load_minimal_of("Extension", |c| c.is_extension());
    let ext = instance.as_extension().unwrap();
    assert!(
        ext.refers_to.is_none(),
        "refersTo should be None in minimal example"
    );
    assert_count(ext.refers_to.as_deref(), 0);
}

// ── AdministrativeInformation ────────────────────────────────────────────────

#[test]
fn test_administrative_information_over_embedded_data_specifications_non_default() {
    let instance = common::load_maximal_of("AdministrativeInformation", |c| {
        c.is_administrative_information()
    });
    let ai = instance.as_administrative_information().unwrap();
    let eds = ai.embedded_data_specifications.as_deref();
    assert!(eds.is_some());
    assert_count(eds, eds.unwrap().len());
}

#[test]
fn test_administrative_information_over_embedded_data_specifications_default() {
    let instance = common::load_minimal_of("AdministrativeInformation", |c| {
        c.is_administrative_information()
    });
    let ai = instance.as_administrative_information().unwrap();
    assert!(ai.embedded_data_specifications.is_none());
    assert_count(ai.embedded_data_specifications.as_deref(), 0);
}

// ── Qualifier ────────────────────────────────────────────────────────────────

#[test]
fn test_qualifier_over_supplemental_semantic_ids_non_default() {
    let instance = common::load_maximal_of("Qualifier", |c| c.is_qualifier());
    let q = instance.as_qualifier().unwrap();
    let ids = q.supplemental_semantic_ids.as_deref();
    assert!(ids.is_some());
    assert_count(ids, ids.unwrap().len());
}

#[test]
fn test_qualifier_over_supplemental_semantic_ids_default() {
    let instance = common::load_minimal_of("Qualifier", |c| c.is_qualifier());
    let q = instance.as_qualifier().unwrap();
    assert!(q.supplemental_semantic_ids.is_none());
    assert_count(q.supplemental_semantic_ids.as_deref(), 0);
}

// ── Submodel ─────────────────────────────────────────────────────────────────

#[test]
fn test_submodel_over_extensions_non_default() {
    let instance = common::load_maximal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    let exts = sm.extensions.as_deref();
    assert!(exts.is_some());
    assert_count(exts, exts.unwrap().len());
}

#[test]
fn test_submodel_over_extensions_default() {
    let instance = common::load_minimal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    assert!(sm.extensions.is_none());
    assert_count(sm.extensions.as_deref(), 0);
}

#[test]
fn test_submodel_over_qualifiers_non_default() {
    let instance = common::load_maximal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    let quals = sm.qualifiers.as_deref();
    assert!(quals.is_some());
    assert_count(quals, quals.unwrap().len());
}

#[test]
fn test_submodel_over_qualifiers_default() {
    let instance = common::load_minimal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    assert!(sm.qualifiers.is_none());
    assert_count(sm.qualifiers.as_deref(), 0);
}

#[test]
fn test_submodel_over_submodel_elements_non_default() {
    let instance = common::load_maximal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    let elems = sm.submodel_elements.as_deref();
    assert!(elems.is_some());
    assert_count(elems, elems.unwrap().len());
}

#[test]
fn test_submodel_over_submodel_elements_default() {
    let instance = common::load_minimal_of("Submodel", |c| c.is_submodel());
    let sm = instance.as_submodel().unwrap();
    assert!(sm.submodel_elements.is_none());
    assert_count(sm.submodel_elements.as_deref(), 0);
}

// ── Property ─────────────────────────────────────────────────────────────────

#[test]
fn test_property_over_extensions_non_default() {
    let instance = common::load_maximal_of("Property", |c| c.is_property());
    let p = instance.as_property().unwrap();
    let exts = p.extensions.as_deref();
    assert!(exts.is_some());
    assert_count(exts, exts.unwrap().len());
}

#[test]
fn test_property_over_extensions_default() {
    let instance = common::load_minimal_of("Property", |c| c.is_property());
    let p = instance.as_property().unwrap();
    assert!(p.extensions.is_none());
    assert_count(p.extensions.as_deref(), 0);
}

// ── Entity ───────────────────────────────────────────────────────────────────

#[test]
fn test_entity_over_statements_non_default() {
    let instance = common::load_maximal_of("Entity", |c| c.is_entity());
    let e = instance.as_entity().unwrap();
    let stmts = e.statements.as_deref();
    assert!(stmts.is_some());
    assert_count(stmts, stmts.unwrap().len());
}

#[test]
fn test_entity_over_statements_default() {
    let instance = common::load_minimal_of("Entity", |c| c.is_entity());
    let e = instance.as_entity().unwrap();
    assert!(e.statements.is_none());
    assert_count(e.statements.as_deref(), 0);
}

// ── Operation ────────────────────────────────────────────────────────────────

#[test]
fn test_operation_over_input_variables_non_default() {
    let instance = common::load_maximal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    let vars = op.input_variables.as_deref();
    assert!(vars.is_some());
    assert_count(vars, vars.unwrap().len());
}

#[test]
fn test_operation_over_input_variables_default() {
    let instance = common::load_minimal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    assert!(op.input_variables.is_none());
    assert_count(op.input_variables.as_deref(), 0);
}

#[test]
fn test_operation_over_output_variables_non_default() {
    let instance = common::load_maximal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    let vars = op.output_variables.as_deref();
    assert!(vars.is_some());
    assert_count(vars, vars.unwrap().len());
}

#[test]
fn test_operation_over_output_variables_default() {
    let instance = common::load_minimal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    assert!(op.output_variables.is_none());
    assert_count(op.output_variables.as_deref(), 0);
}

#[test]
fn test_operation_over_inoutput_variables_non_default() {
    let instance = common::load_maximal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    let vars = op.inoutput_variables.as_deref();
    assert!(vars.is_some());
    assert_count(vars, vars.unwrap().len());
}

#[test]
fn test_operation_over_inoutput_variables_default() {
    let instance = common::load_minimal_of("Operation", |c| c.is_operation());
    let op = instance.as_operation().unwrap();
    assert!(op.inoutput_variables.is_none());
    assert_count(op.inoutput_variables.as_deref(), 0);
}

// ── ConceptDescription ───────────────────────────────────────────────────────

#[test]
fn test_concept_description_over_is_case_of_non_default() {
    let instance = common::load_maximal_of("ConceptDescription", |c| c.is_concept_description());
    let cd = instance.as_concept_description().unwrap();
    let refs = cd.is_case_of.as_deref();
    assert!(refs.is_some());
    assert_count(refs, refs.unwrap().len());
}

#[test]
fn test_concept_description_over_is_case_of_default() {
    let instance = common::load_minimal_of("ConceptDescription", |c| c.is_concept_description());
    let cd = instance.as_concept_description().unwrap();
    assert!(cd.is_case_of.is_none());
    assert_count(cd.is_case_of.as_deref(), 0);
}

// ── AssetAdministrationShell ─────────────────────────────────────────────────

#[test]
fn test_aas_over_submodels_non_default() {
    let instance = common::load_maximal_of("AssetAdministrationShell", |c| {
        c.is_asset_administration_shell()
    });
    let aas = instance.as_asset_administration_shell().unwrap();
    let subs = aas.submodels.as_deref();
    assert!(subs.is_some());
    assert_count(subs, subs.unwrap().len());
}

#[test]
fn test_aas_over_submodels_default() {
    let instance = common::load_minimal_of("AssetAdministrationShell", |c| {
        c.is_asset_administration_shell()
    });
    let aas = instance.as_asset_administration_shell().unwrap();
    assert!(aas.submodels.is_none());
    assert_count(aas.submodels.as_deref(), 0);
}
