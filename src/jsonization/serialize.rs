//! Serialize AAS classes to `serde_json::Value`.

#![allow(clippy::too_many_lines)]

use serde_json::{Map, Value};

use crate::stringification;
use crate::types::class::Class;
use crate::{
    AdministrativeInformation, AnnotatedRelationshipElement, AssetAdministrationShell,
    AssetInformation, BasicEventElement, Blob, Capability, ConceptDescription,
    DataSpecificationIec61360, EmbeddedDataSpecification, Entity, Environment, EventPayload,
    Extension, File, Key, LangStringDefinitionTypeIec61360, LangStringNameType,
    LangStringPreferredNameTypeIec61360, LangStringShortNameTypeIec61360, LangStringTextType,
    LevelType, MultiLanguageProperty, Operation, OperationVariable, Property, Qualifier, Range,
    Reference, ReferenceElement, RelationshipElement, Resource, SpecificAssetId,
    Submodel, SubmodelElementCollection, SubmodelElementList, ValueList, ValueReferencePair,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn serialize_reference(r: &Reference) -> Value {
    let mut m = Map::new();
    m.insert(
        "type".to_owned(),
        Value::String(
            stringification::must_reference_types_to_str(r.type_).to_owned(),
        ),
    );
    if let Some(rsi) = &r.referred_semantic_id {
        m.insert("referredSemanticId".to_owned(), serialize_reference(rsi));
    }
    let keys: Vec<Value> = r.keys.iter().map(serialize_key).collect();
    m.insert("keys".to_owned(), Value::Array(keys));
    Value::Object(m)
}

fn serialize_key(k: &Key) -> Value {
    let mut m = Map::new();
    m.insert(
        "type".to_owned(),
        Value::String(stringification::must_key_types_to_str(k.type_).to_owned()),
    );
    m.insert("value".to_owned(), Value::String(k.value.clone()));
    Value::Object(m)
}

fn serialize_lang_string_name(l: &LangStringNameType) -> Value {
    let mut m = Map::new();
    m.insert("language".to_owned(), Value::String(l.language.clone()));
    m.insert("text".to_owned(), Value::String(l.text.clone()));
    Value::Object(m)
}

fn serialize_lang_string_text(l: &LangStringTextType) -> Value {
    let mut m = Map::new();
    m.insert("language".to_owned(), Value::String(l.language.clone()));
    m.insert("text".to_owned(), Value::String(l.text.clone()));
    Value::Object(m)
}

fn serialize_lang_string_preferred_name_iec61360(
    l: &LangStringPreferredNameTypeIec61360,
) -> Value {
    let mut m = Map::new();
    m.insert("language".to_owned(), Value::String(l.language.clone()));
    m.insert("text".to_owned(), Value::String(l.text.clone()));
    Value::Object(m)
}

fn serialize_lang_string_short_name_iec61360(l: &LangStringShortNameTypeIec61360) -> Value {
    let mut m = Map::new();
    m.insert("language".to_owned(), Value::String(l.language.clone()));
    m.insert("text".to_owned(), Value::String(l.text.clone()));
    Value::Object(m)
}

fn serialize_lang_string_definition_iec61360(l: &LangStringDefinitionTypeIec61360) -> Value {
    let mut m = Map::new();
    m.insert("language".to_owned(), Value::String(l.language.clone()));
    m.insert("text".to_owned(), Value::String(l.text.clone()));
    Value::Object(m)
}

fn serialize_resource(r: &Resource) -> Value {
    let mut m = Map::new();
    m.insert("path".to_owned(), Value::String(r.path.clone()));
    if let Some(ct) = &r.content_type {
        m.insert("contentType".to_owned(), Value::String(ct.clone()));
    }
    Value::Object(m)
}

fn serialize_level_type(lt: &LevelType) -> Value {
    let mut m = Map::new();
    m.insert("min".to_owned(), Value::Bool(lt.min));
    m.insert("nom".to_owned(), Value::Bool(lt.nom));
    m.insert("typ".to_owned(), Value::Bool(lt.typ));
    m.insert("max".to_owned(), Value::Bool(lt.max));
    Value::Object(m)
}

fn serialize_value_reference_pair(vrp: &ValueReferencePair) -> Value {
    let mut m = Map::new();
    m.insert("value".to_owned(), Value::String(vrp.value.clone()));
    m.insert("valueId".to_owned(), serialize_reference(&vrp.value_id));
    Value::Object(m)
}

fn serialize_value_list(vl: &ValueList) -> Value {
    let mut m = Map::new();
    let pairs: Vec<Value> = vl
        .value_reference_pairs
        .iter()
        .map(serialize_value_reference_pair)
        .collect();
    m.insert("valueReferencePairs".to_owned(), Value::Array(pairs));
    Value::Object(m)
}

fn serialize_extension(ext: &Extension) -> Value {
    let mut m = Map::new();
    if let Some(sid) = &ext.semantic_id {
        m.insert("semanticId".to_owned(), serialize_reference(sid));
    }
    if let Some(sids) = &ext.supplemental_semantic_ids {
        m.insert(
            "supplementalSemanticIds".to_owned(),
            Value::Array(sids.iter().map(serialize_reference).collect()),
        );
    }
    m.insert("name".to_owned(), Value::String(ext.name.clone()));
    if let Some(vt) = ext.value_type {
        m.insert(
            "valueType".to_owned(),
            Value::String(
                stringification::must_data_type_def_xsd_to_str(vt).to_owned(),
            ),
        );
    }
    if let Some(v) = &ext.value {
        m.insert("value".to_owned(), Value::String(v.clone()));
    }
    if let Some(rt) = &ext.refers_to {
        m.insert(
            "refersTo".to_owned(),
            Value::Array(rt.iter().map(serialize_reference).collect()),
        );
    }
    Value::Object(m)
}

fn serialize_qualifier(q: &Qualifier) -> Value {
    let mut m = Map::new();
    if let Some(sid) = &q.semantic_id {
        m.insert("semanticId".to_owned(), serialize_reference(sid));
    }
    if let Some(sids) = &q.supplemental_semantic_ids {
        m.insert(
            "supplementalSemanticIds".to_owned(),
            Value::Array(sids.iter().map(serialize_reference).collect()),
        );
    }
    if let Some(k) = q.kind {
        m.insert(
            "kind".to_owned(),
            Value::String(stringification::must_qualifier_kind_to_str(k).to_owned()),
        );
    }
    m.insert("type".to_owned(), Value::String(q.type_.clone()));
    m.insert(
        "valueType".to_owned(),
        Value::String(
            stringification::must_data_type_def_xsd_to_str(q.value_type).to_owned(),
        ),
    );
    if let Some(v) = &q.value {
        m.insert("value".to_owned(), Value::String(v.clone()));
    }
    if let Some(vi) = &q.value_id {
        m.insert("valueId".to_owned(), serialize_reference(vi));
    }
    Value::Object(m)
}

fn serialize_specific_asset_id(s: &SpecificAssetId) -> Value {
    let mut m = Map::new();
    if let Some(sid) = &s.semantic_id {
        m.insert("semanticId".to_owned(), serialize_reference(sid));
    }
    if let Some(sids) = &s.supplemental_semantic_ids {
        m.insert(
            "supplementalSemanticIds".to_owned(),
            Value::Array(sids.iter().map(serialize_reference).collect()),
        );
    }
    m.insert("name".to_owned(), Value::String(s.name.clone()));
    m.insert("value".to_owned(), Value::String(s.value.clone()));
    if let Some(esi) = &s.external_subject_id {
        m.insert("externalSubjectId".to_owned(), serialize_reference(esi));
    }
    Value::Object(m)
}

fn serialize_administrative_information(a: &AdministrativeInformation) -> Value {
    let mut m = Map::new();
    if let Some(eds) = &a.embedded_data_specifications {
        m.insert(
            "embeddedDataSpecifications".to_owned(),
            Value::Array(eds.iter().map(serialize_embedded_data_specification).collect()),
        );
    }
    if let Some(v) = &a.version {
        m.insert("version".to_owned(), Value::String(v.clone()));
    }
    if let Some(r) = &a.revision {
        m.insert("revision".to_owned(), Value::String(r.clone()));
    }
    if let Some(c) = &a.creator {
        m.insert("creator".to_owned(), serialize_reference(c));
    }
    if let Some(ti) = &a.template_id {
        m.insert("templateId".to_owned(), Value::String(ti.clone()));
    }
    Value::Object(m)
}

fn serialize_embedded_data_specification(e: &EmbeddedDataSpecification) -> Value {
    let mut m = Map::new();
    m.insert(
        "dataSpecification".to_owned(),
        serialize_reference(&e.data_specification),
    );
    m.insert(
        "dataSpecificationContent".to_owned(),
        to_jsonable(&e.data_specification_content),
    );
    Value::Object(m)
}

fn serialize_data_specification_iec61360(d: &DataSpecificationIec61360) -> Value {
    let mut m = Map::new();
    m.insert(
        "preferredName".to_owned(),
        Value::Array(
            d.preferred_name
                .iter()
                .map(serialize_lang_string_preferred_name_iec61360)
                .collect(),
        ),
    );
    if let Some(sn) = &d.short_name {
        m.insert(
            "shortName".to_owned(),
            Value::Array(sn.iter().map(serialize_lang_string_short_name_iec61360).collect()),
        );
    }
    if let Some(u) = &d.unit {
        m.insert("unit".to_owned(), Value::String(u.clone()));
    }
    if let Some(uid) = &d.unit_id {
        m.insert("unitId".to_owned(), serialize_reference(uid));
    }
    if let Some(sod) = &d.source_of_definition {
        m.insert("sourceOfDefinition".to_owned(), Value::String(sod.clone()));
    }
    if let Some(sym) = &d.symbol {
        m.insert("symbol".to_owned(), Value::String(sym.clone()));
    }
    if let Some(dt) = d.data_type {
        m.insert(
            "dataType".to_owned(),
            Value::String(
                stringification::must_data_type_iec_61360_to_str(dt).to_owned(),
            ),
        );
    }
    if let Some(def) = &d.definition {
        m.insert(
            "definition".to_owned(),
            Value::Array(
                def.iter()
                    .map(serialize_lang_string_definition_iec61360)
                    .collect(),
            ),
        );
    }
    if let Some(vf) = &d.value_format {
        m.insert("valueFormat".to_owned(), Value::String(vf.clone()));
    }
    if let Some(vl) = &d.value_list {
        m.insert("valueList".to_owned(), serialize_value_list(vl));
    }
    if let Some(v) = &d.value {
        m.insert("value".to_owned(), Value::String(v.clone()));
    }
    if let Some(lt) = &d.level_type {
        m.insert("levelType".to_owned(), serialize_level_type(lt));
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("DataSpecificationIec61360".to_owned()),
    );
    Value::Object(m)
}

// ── SME common fields helper ──────────────────────────────────────────────────

fn insert_sme_common_fields(
    m: &mut Map<String, Value>,
    extensions: &Option<Vec<Extension>>,
    category: &Option<String>,
    id_short: &Option<String>,
    display_name: &Option<Vec<LangStringNameType>>,
    description: &Option<Vec<LangStringTextType>>,
    semantic_id: &Option<Reference>,
    supplemental_semantic_ids: &Option<Vec<Reference>>,
    qualifiers: &Option<Vec<Qualifier>>,
    embedded_data_specifications: &Option<Vec<EmbeddedDataSpecification>>,
) {
    if let Some(exts) = extensions {
        m.insert(
            "extensions".to_owned(),
            Value::Array(exts.iter().map(serialize_extension).collect()),
        );
    }
    if let Some(cat) = category {
        m.insert("category".to_owned(), Value::String(cat.clone()));
    }
    if let Some(ids) = id_short {
        m.insert("idShort".to_owned(), Value::String(ids.clone()));
    }
    if let Some(dn) = display_name {
        m.insert(
            "displayName".to_owned(),
            Value::Array(dn.iter().map(serialize_lang_string_name).collect()),
        );
    }
    if let Some(desc) = description {
        m.insert(
            "description".to_owned(),
            Value::Array(desc.iter().map(serialize_lang_string_text).collect()),
        );
    }
    if let Some(sid) = semantic_id {
        m.insert("semanticId".to_owned(), serialize_reference(sid));
    }
    if let Some(sids) = supplemental_semantic_ids {
        m.insert(
            "supplementalSemanticIds".to_owned(),
            Value::Array(sids.iter().map(serialize_reference).collect()),
        );
    }
    if let Some(qs) = qualifiers {
        m.insert(
            "qualifiers".to_owned(),
            Value::Array(qs.iter().map(serialize_qualifier).collect()),
        );
    }
    if let Some(eds) = embedded_data_specifications {
        m.insert(
            "embeddedDataSpecifications".to_owned(),
            Value::Array(eds.iter().map(serialize_embedded_data_specification).collect()),
        );
    }
}

// ── Core struct serializers ───────────────────────────────────────────────────

fn serialize_asset_information(a: &AssetInformation) -> Value {
    let mut m = Map::new();
    m.insert(
        "assetKind".to_owned(),
        Value::String(stringification::must_asset_kind_to_str(a.asset_kind).to_owned()),
    );
    if let Some(gai) = &a.global_asset_id {
        m.insert("globalAssetId".to_owned(), Value::String(gai.clone()));
    }
    if let Some(ids) = &a.specific_asset_ids {
        m.insert(
            "specificAssetIds".to_owned(),
            Value::Array(ids.iter().map(serialize_specific_asset_id).collect()),
        );
    }
    if let Some(at) = &a.asset_type {
        m.insert("assetType".to_owned(), Value::String(at.clone()));
    }
    if let Some(dt) = &a.default_thumbnail {
        m.insert("defaultThumbnail".to_owned(), serialize_resource(dt));
    }
    Value::Object(m)
}

fn serialize_asset_administration_shell(a: &AssetAdministrationShell) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &a.extensions,
        &a.category,
        &a.id_short,
        &a.display_name,
        &a.description,
        &None,
        &None,
        &None,
        &a.embedded_data_specifications,
    );
    if let Some(adm) = &a.administration {
        m.insert("administration".to_owned(), serialize_administrative_information(adm));
    }
    m.insert("id".to_owned(), Value::String(a.id.clone()));
    if let Some(df) = &a.derived_from {
        m.insert("derivedFrom".to_owned(), serialize_reference(df));
    }
    m.insert(
        "assetInformation".to_owned(),
        serialize_asset_information(&a.asset_information),
    );
    if let Some(sms) = &a.submodels {
        m.insert(
            "submodels".to_owned(),
            Value::Array(sms.iter().map(serialize_reference).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("AssetAdministrationShell".to_owned()),
    );
    Value::Object(m)
}

fn serialize_submodel(s: &Submodel) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &s.extensions,
        &s.category,
        &s.id_short,
        &s.display_name,
        &s.description,
        &s.semantic_id,
        &s.supplemental_semantic_ids,
        &s.qualifiers,
        &s.embedded_data_specifications,
    );
    if let Some(adm) = &s.administration {
        m.insert("administration".to_owned(), serialize_administrative_information(adm));
    }
    m.insert("id".to_owned(), Value::String(s.id.clone()));
    if let Some(k) = s.kind {
        m.insert(
            "kind".to_owned(),
            Value::String(stringification::must_modelling_kind_to_str(k).to_owned()),
        );
    }
    if let Some(sme) = &s.submodel_elements {
        m.insert(
            "submodelElements".to_owned(),
            Value::Array(sme.iter().map(to_jsonable).collect()),
        );
    }
    m.insert("modelType".to_owned(), Value::String("Submodel".to_owned()));
    Value::Object(m)
}

fn serialize_concept_description(c: &ConceptDescription) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &c.extensions,
        &c.category,
        &c.id_short,
        &c.display_name,
        &c.description,
        &None,
        &None,
        &None,
        &c.embedded_data_specifications,
    );
    if let Some(adm) = &c.administration {
        m.insert("administration".to_owned(), serialize_administrative_information(adm));
    }
    m.insert("id".to_owned(), Value::String(c.id.clone()));
    if let Some(ico) = &c.is_case_of {
        m.insert(
            "isCaseOf".to_owned(),
            Value::Array(ico.iter().map(serialize_reference).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("ConceptDescription".to_owned()),
    );
    Value::Object(m)
}

fn serialize_environment(e: &Environment) -> Value {
    let mut m = Map::new();
    if let Some(aas) = &e.asset_administration_shells {
        m.insert(
            "assetAdministrationShells".to_owned(),
            Value::Array(aas.iter().map(serialize_asset_administration_shell).collect()),
        );
    }
    if let Some(sms) = &e.submodels {
        m.insert(
            "submodels".to_owned(),
            Value::Array(sms.iter().map(serialize_submodel).collect()),
        );
    }
    if let Some(cds) = &e.concept_descriptions {
        m.insert(
            "conceptDescriptions".to_owned(),
            Value::Array(cds.iter().map(serialize_concept_description).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("Environment".to_owned()),
    );
    Value::Object(m)
}

fn serialize_operation_variable(ov: &OperationVariable) -> Value {
    let mut m = Map::new();
    m.insert("value".to_owned(), to_jsonable(&ov.value));
    Value::Object(m)
}

fn serialize_event_payload(ep: &EventPayload) -> Value {
    let mut m = Map::new();
    m.insert("source".to_owned(), serialize_reference(&ep.source));
    if let Some(ssi) = &ep.source_semantic_id {
        m.insert("sourceSemanticId".to_owned(), serialize_reference(ssi));
    }
    m.insert(
        "observableReference".to_owned(),
        serialize_reference(&ep.observable_reference),
    );
    if let Some(osi) = &ep.observable_semantic_id {
        m.insert("observableSemanticId".to_owned(), serialize_reference(osi));
    }
    if let Some(t) = &ep.topic {
        m.insert("topic".to_owned(), Value::String(t.clone()));
    }
    if let Some(sid) = &ep.subject_id {
        m.insert("subjectId".to_owned(), serialize_reference(sid));
    }
    m.insert("timeStamp".to_owned(), Value::String(ep.time_stamp.clone()));
    if let Some(p) = &ep.payload {
        m.insert(
            "payload".to_owned(),
            Value::String(crate::common::base64_encode(p)),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("EventPayload".to_owned()),
    );
    Value::Object(m)
}

// ── SME serializers ───────────────────────────────────────────────────────────

fn serialize_property(p: &Property) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &p.extensions,
        &p.category,
        &p.id_short,
        &p.display_name,
        &p.description,
        &p.semantic_id,
        &p.supplemental_semantic_ids,
        &p.qualifiers,
        &p.embedded_data_specifications,
    );
    m.insert(
        "valueType".to_owned(),
        Value::String(
            stringification::must_data_type_def_xsd_to_str(p.value_type).to_owned(),
        ),
    );
    if let Some(v) = &p.value {
        m.insert("value".to_owned(), Value::String(v.clone()));
    }
    if let Some(vi) = &p.value_id {
        m.insert("valueId".to_owned(), serialize_reference(vi));
    }
    m.insert("modelType".to_owned(), Value::String("Property".to_owned()));
    Value::Object(m)
}

fn serialize_multi_language_property(mlp: &MultiLanguageProperty) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &mlp.extensions,
        &mlp.category,
        &mlp.id_short,
        &mlp.display_name,
        &mlp.description,
        &mlp.semantic_id,
        &mlp.supplemental_semantic_ids,
        &mlp.qualifiers,
        &mlp.embedded_data_specifications,
    );
    if let Some(v) = &mlp.value {
        m.insert(
            "value".to_owned(),
            Value::Array(v.iter().map(serialize_lang_string_text).collect()),
        );
    }
    if let Some(vi) = &mlp.value_id {
        m.insert("valueId".to_owned(), serialize_reference(vi));
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("MultiLanguageProperty".to_owned()),
    );
    Value::Object(m)
}

fn serialize_range(r: &Range) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &r.extensions,
        &r.category,
        &r.id_short,
        &r.display_name,
        &r.description,
        &r.semantic_id,
        &r.supplemental_semantic_ids,
        &r.qualifiers,
        &r.embedded_data_specifications,
    );
    m.insert(
        "valueType".to_owned(),
        Value::String(
            stringification::must_data_type_def_xsd_to_str(r.value_type).to_owned(),
        ),
    );
    if let Some(min) = &r.min {
        m.insert("min".to_owned(), Value::String(min.clone()));
    }
    if let Some(max) = &r.max {
        m.insert("max".to_owned(), Value::String(max.clone()));
    }
    m.insert("modelType".to_owned(), Value::String("Range".to_owned()));
    Value::Object(m)
}

fn serialize_reference_element(re: &ReferenceElement) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &re.extensions,
        &re.category,
        &re.id_short,
        &re.display_name,
        &re.description,
        &re.semantic_id,
        &re.supplemental_semantic_ids,
        &re.qualifiers,
        &re.embedded_data_specifications,
    );
    if let Some(v) = &re.value {
        m.insert("value".to_owned(), serialize_reference(v));
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("ReferenceElement".to_owned()),
    );
    Value::Object(m)
}

fn serialize_blob(b: &Blob) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &b.extensions,
        &b.category,
        &b.id_short,
        &b.display_name,
        &b.description,
        &b.semantic_id,
        &b.supplemental_semantic_ids,
        &b.qualifiers,
        &b.embedded_data_specifications,
    );
    if let Some(v) = &b.value {
        m.insert(
            "value".to_owned(),
            Value::String(crate::common::base64_encode(v)),
        );
    }
    m.insert("contentType".to_owned(), Value::String(b.content_type.clone()));
    m.insert("modelType".to_owned(), Value::String("Blob".to_owned()));
    Value::Object(m)
}

fn serialize_file(f: &File) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &f.extensions,
        &f.category,
        &f.id_short,
        &f.display_name,
        &f.description,
        &f.semantic_id,
        &f.supplemental_semantic_ids,
        &f.qualifiers,
        &f.embedded_data_specifications,
    );
    if let Some(v) = &f.value {
        m.insert("value".to_owned(), Value::String(v.clone()));
    }
    m.insert("contentType".to_owned(), Value::String(f.content_type.clone()));
    m.insert("modelType".to_owned(), Value::String("File".to_owned()));
    Value::Object(m)
}

fn serialize_relationship_element(re: &RelationshipElement) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &re.extensions,
        &re.category,
        &re.id_short,
        &re.display_name,
        &re.description,
        &re.semantic_id,
        &re.supplemental_semantic_ids,
        &re.qualifiers,
        &re.embedded_data_specifications,
    );
    m.insert("first".to_owned(), serialize_reference(&re.first));
    m.insert("second".to_owned(), serialize_reference(&re.second));
    m.insert(
        "modelType".to_owned(),
        Value::String("RelationshipElement".to_owned()),
    );
    Value::Object(m)
}

fn serialize_annotated_relationship_element(are: &AnnotatedRelationshipElement) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &are.extensions,
        &are.category,
        &are.id_short,
        &are.display_name,
        &are.description,
        &are.semantic_id,
        &are.supplemental_semantic_ids,
        &are.qualifiers,
        &are.embedded_data_specifications,
    );
    m.insert("first".to_owned(), serialize_reference(&are.first));
    m.insert("second".to_owned(), serialize_reference(&are.second));
    if let Some(anns) = &are.annotations {
        m.insert(
            "annotations".to_owned(),
            Value::Array(anns.iter().map(to_jsonable).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("AnnotatedRelationshipElement".to_owned()),
    );
    Value::Object(m)
}

fn serialize_submodel_element_list(sel: &SubmodelElementList) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &sel.extensions,
        &sel.category,
        &sel.id_short,
        &sel.display_name,
        &sel.description,
        &sel.semantic_id,
        &sel.supplemental_semantic_ids,
        &sel.qualifiers,
        &sel.embedded_data_specifications,
    );
    if let Some(or) = sel.order_relevant {
        m.insert("orderRelevant".to_owned(), Value::Bool(or));
    }
    if let Some(sile) = &sel.semantic_id_list_element {
        m.insert("semanticIdListElement".to_owned(), serialize_reference(sile));
    }
    m.insert(
        "typeValueListElement".to_owned(),
        Value::String(
            stringification::must_aas_submodel_elements_to_str(sel.type_value_list_element)
                .to_owned(),
        ),
    );
    if let Some(vtl) = sel.value_type_list_element {
        m.insert(
            "valueTypeListElement".to_owned(),
            Value::String(
                stringification::must_data_type_def_xsd_to_str(vtl).to_owned(),
            ),
        );
    }
    if let Some(vals) = &sel.value {
        m.insert(
            "value".to_owned(),
            Value::Array(vals.iter().map(to_jsonable).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("SubmodelElementList".to_owned()),
    );
    Value::Object(m)
}

fn serialize_submodel_element_collection(sec: &SubmodelElementCollection) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &sec.extensions,
        &sec.category,
        &sec.id_short,
        &sec.display_name,
        &sec.description,
        &sec.semantic_id,
        &sec.supplemental_semantic_ids,
        &sec.qualifiers,
        &sec.embedded_data_specifications,
    );
    if let Some(vals) = &sec.value {
        m.insert(
            "value".to_owned(),
            Value::Array(vals.iter().map(to_jsonable).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("SubmodelElementCollection".to_owned()),
    );
    Value::Object(m)
}

fn serialize_entity(e: &Entity) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &e.extensions,
        &e.category,
        &e.id_short,
        &e.display_name,
        &e.description,
        &e.semantic_id,
        &e.supplemental_semantic_ids,
        &e.qualifiers,
        &e.embedded_data_specifications,
    );
    if let Some(stmts) = &e.statements {
        m.insert(
            "statements".to_owned(),
            Value::Array(stmts.iter().map(to_jsonable).collect()),
        );
    }
    m.insert(
        "entityType".to_owned(),
        Value::String(stringification::must_entity_type_to_str(e.entity_type).to_owned()),
    );
    if let Some(gai) = &e.global_asset_id {
        m.insert("globalAssetId".to_owned(), Value::String(gai.clone()));
    }
    if let Some(ids) = &e.specific_asset_ids {
        m.insert(
            "specificAssetIds".to_owned(),
            Value::Array(ids.iter().map(serialize_specific_asset_id).collect()),
        );
    }
    m.insert("modelType".to_owned(), Value::String("Entity".to_owned()));
    Value::Object(m)
}

fn serialize_basic_event_element(be: &BasicEventElement) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &be.extensions,
        &be.category,
        &be.id_short,
        &be.display_name,
        &be.description,
        &be.semantic_id,
        &be.supplemental_semantic_ids,
        &be.qualifiers,
        &be.embedded_data_specifications,
    );
    m.insert("observed".to_owned(), serialize_reference(&be.observed));
    m.insert(
        "direction".to_owned(),
        Value::String(stringification::must_direction_to_str(be.direction).to_owned()),
    );
    m.insert(
        "state".to_owned(),
        Value::String(stringification::must_state_of_event_to_str(be.state).to_owned()),
    );
    if let Some(mt) = &be.message_topic {
        m.insert("messageTopic".to_owned(), Value::String(mt.clone()));
    }
    if let Some(mb) = &be.message_broker {
        m.insert("messageBroker".to_owned(), serialize_reference(mb));
    }
    if let Some(lu) = &be.last_update {
        m.insert("lastUpdate".to_owned(), Value::String(lu.clone()));
    }
    if let Some(mi) = &be.min_interval {
        m.insert("minInterval".to_owned(), Value::String(mi.clone()));
    }
    if let Some(ma) = &be.max_interval {
        m.insert("maxInterval".to_owned(), Value::String(ma.clone()));
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("BasicEventElement".to_owned()),
    );
    Value::Object(m)
}

fn serialize_operation(op: &Operation) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &op.extensions,
        &op.category,
        &op.id_short,
        &op.display_name,
        &op.description,
        &op.semantic_id,
        &op.supplemental_semantic_ids,
        &op.qualifiers,
        &op.embedded_data_specifications,
    );
    if let Some(ivs) = &op.input_variables {
        m.insert(
            "inputVariables".to_owned(),
            Value::Array(ivs.iter().map(serialize_operation_variable).collect()),
        );
    }
    if let Some(ovs) = &op.output_variables {
        m.insert(
            "outputVariables".to_owned(),
            Value::Array(ovs.iter().map(serialize_operation_variable).collect()),
        );
    }
    if let Some(iovs) = &op.inoutput_variables {
        m.insert(
            "inoutputVariables".to_owned(),
            Value::Array(iovs.iter().map(serialize_operation_variable).collect()),
        );
    }
    m.insert(
        "modelType".to_owned(),
        Value::String("Operation".to_owned()),
    );
    Value::Object(m)
}

fn serialize_capability(c: &Capability) -> Value {
    let mut m = Map::new();
    insert_sme_common_fields(
        &mut m,
        &c.extensions,
        &c.category,
        &c.id_short,
        &c.display_name,
        &c.description,
        &c.semantic_id,
        &c.supplemental_semantic_ids,
        &c.qualifiers,
        &c.embedded_data_specifications,
    );
    m.insert(
        "modelType".to_owned(),
        Value::String("Capability".to_owned()),
    );
    Value::Object(m)
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Serialize any [`Class`] variant into a [`serde_json::Value`].
pub fn to_jsonable(value: &Class) -> Value {
    match value {
        Class::Extension(x) => {
            let mut v = serialize_extension(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("Extension".to_owned()),
                );
            }
            v
        }
        Class::AdministrativeInformation(x) => {
            let mut v = serialize_administrative_information(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("AdministrativeInformation".to_owned()),
                );
            }
            v
        }
        Class::Qualifier(x) => {
            let mut v = serialize_qualifier(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("Qualifier".to_owned()),
                );
            }
            v
        }
        Class::AssetAdministrationShell(x) => serialize_asset_administration_shell(x),
        Class::AssetInformation(x) => {
            let mut v = serialize_asset_information(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("AssetInformation".to_owned()),
                );
            }
            v
        }
        Class::Resource(x) => {
            let mut v = serialize_resource(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("Resource".to_owned()),
                );
            }
            v
        }
        Class::SpecificAssetId(x) => {
            let mut v = serialize_specific_asset_id(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("SpecificAssetId".to_owned()),
                );
            }
            v
        }
        Class::Submodel(x) => serialize_submodel(x),
        Class::RelationshipElement(x) => serialize_relationship_element(x),
        Class::SubmodelElementList(x) => serialize_submodel_element_list(x),
        Class::SubmodelElementCollection(x) => serialize_submodel_element_collection(x),
        Class::Property(x) => serialize_property(x),
        Class::MultiLanguageProperty(x) => serialize_multi_language_property(x),
        Class::Range(x) => serialize_range(x),
        Class::ReferenceElement(x) => serialize_reference_element(x),
        Class::Blob(x) => serialize_blob(x),
        Class::File(x) => serialize_file(x),
        Class::AnnotatedRelationshipElement(x) => serialize_annotated_relationship_element(x),
        Class::Entity(x) => serialize_entity(x),
        Class::EventPayload(x) => serialize_event_payload(x),
        Class::BasicEventElement(x) => serialize_basic_event_element(x),
        Class::Operation(x) => serialize_operation(x),
        Class::OperationVariable(x) => {
            let mut v = serialize_operation_variable(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("OperationVariable".to_owned()),
                );
            }
            v
        }
        Class::Capability(x) => serialize_capability(x),
        Class::ConceptDescription(x) => serialize_concept_description(x),
        Class::Reference(x) => {
            let mut v = serialize_reference(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("Reference".to_owned()),
                );
            }
            v
        }
        Class::Key(x) => {
            let mut v = serialize_key(x);
            if let Value::Object(m) = &mut v {
                m.insert("modelType".to_owned(), Value::String("Key".to_owned()));
            }
            v
        }
        Class::LangStringNameType(x) => {
            let mut v = serialize_lang_string_name(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LangStringNameType".to_owned()),
                );
            }
            v
        }
        Class::LangStringTextType(x) => {
            let mut v = serialize_lang_string_text(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LangStringTextType".to_owned()),
                );
            }
            v
        }
        Class::Environment(x) => serialize_environment(x),
        Class::EmbeddedDataSpecification(x) => {
            let mut v = serialize_embedded_data_specification(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("EmbeddedDataSpecification".to_owned()),
                );
            }
            v
        }
        Class::LevelType(x) => {
            let mut v = serialize_level_type(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LevelType".to_owned()),
                );
            }
            v
        }
        Class::ValueReferencePair(x) => {
            let mut v = serialize_value_reference_pair(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("ValueReferencePair".to_owned()),
                );
            }
            v
        }
        Class::ValueList(x) => {
            let mut v = serialize_value_list(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("ValueList".to_owned()),
                );
            }
            v
        }
        Class::LangStringPreferredNameTypeIec61360(x) => {
            let mut v = serialize_lang_string_preferred_name_iec61360(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LangStringPreferredNameTypeIec61360".to_owned()),
                );
            }
            v
        }
        Class::LangStringShortNameTypeIec61360(x) => {
            let mut v = serialize_lang_string_short_name_iec61360(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LangStringShortNameTypeIec61360".to_owned()),
                );
            }
            v
        }
        Class::LangStringDefinitionTypeIec61360(x) => {
            let mut v = serialize_lang_string_definition_iec61360(x);
            if let Value::Object(m) = &mut v {
                m.insert(
                    "modelType".to_owned(),
                    Value::String("LangStringDefinitionTypeIec61360".to_owned()),
                );
            }
            v
        }
        Class::DataSpecificationIec61360(x) => serialize_data_specification_iec61360(x),
    }
}
