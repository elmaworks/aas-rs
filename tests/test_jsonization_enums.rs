//! Tests for JSON de/serialization of enumeration literals.
//! Mirrors aas-core3.0-typescript/test/jsonization.enums.spec.ts

use aas_rs::jsonization;
use aas_rs::*;
use serde_json::json;

macro_rules! test_enum_round_trip {
    ($from_fn:ident, $expected:expr, $text:expr) => {{
        let v = json!($text);
        let result = jsonization::$from_fn(&v);
        assert!(
            result.is_ok(),
            "Expected Ok for {:?} but got {:?}",
            $text,
            result
        );
        assert_eq!(result.unwrap(), $expected);
    }};
}

macro_rules! test_enum_fail {
    ($from_fn:ident, $text:expr) => {{
        let v = json!($text);
        let result = jsonization::$from_fn(&v);
        assert!(result.is_err(), "Expected Err for {:?} but got Ok", $text);
    }};
}

#[test]
fn test_modelling_kind_round_trip() {
    test_enum_round_trip!(
        modelling_kind_from_jsonable,
        ModellingKind::Template,
        "Template"
    );
}

#[test]
fn test_modelling_kind_fail() {
    test_enum_fail!(modelling_kind_from_jsonable, "invalid-literal");
}

#[test]
fn test_qualifier_kind_round_trip() {
    test_enum_round_trip!(
        qualifier_kind_from_jsonable,
        QualifierKind::ValueQualifier,
        "ValueQualifier"
    );
}

#[test]
fn test_qualifier_kind_fail() {
    test_enum_fail!(qualifier_kind_from_jsonable, "invalid-literal");
}

#[test]
fn test_asset_kind_round_trip() {
    test_enum_round_trip!(asset_kind_from_jsonable, AssetKind::Type, "Type");
}

#[test]
fn test_asset_kind_fail() {
    test_enum_fail!(asset_kind_from_jsonable, "invalid-literal");
}

#[test]
fn test_aas_submodel_elements_round_trip() {
    test_enum_round_trip!(
        aas_submodel_elements_from_jsonable,
        AasSubmodelElements::AnnotatedRelationshipElement,
        "AnnotatedRelationshipElement"
    );
}

#[test]
fn test_aas_submodel_elements_fail() {
    test_enum_fail!(aas_submodel_elements_from_jsonable, "invalid-literal");
}

#[test]
fn test_entity_type_round_trip() {
    test_enum_round_trip!(
        entity_type_from_jsonable,
        EntityType::CoManagedEntity,
        "CoManagedEntity"
    );
}

#[test]
fn test_entity_type_fail() {
    test_enum_fail!(entity_type_from_jsonable, "invalid-literal");
}

#[test]
fn test_direction_round_trip() {
    test_enum_round_trip!(direction_from_jsonable, Direction::Input, "input");
}

#[test]
fn test_direction_fail() {
    test_enum_fail!(direction_from_jsonable, "invalid-literal");
}

#[test]
fn test_state_of_event_round_trip() {
    test_enum_round_trip!(state_of_event_from_jsonable, StateOfEvent::On, "on");
}

#[test]
fn test_state_of_event_fail() {
    test_enum_fail!(state_of_event_from_jsonable, "invalid-literal");
}

#[test]
fn test_reference_types_round_trip() {
    test_enum_round_trip!(
        reference_types_from_jsonable,
        ReferenceTypes::ExternalReference,
        "ExternalReference"
    );
}

#[test]
fn test_reference_types_fail() {
    test_enum_fail!(reference_types_from_jsonable, "invalid-literal");
}

#[test]
fn test_key_types_round_trip() {
    test_enum_round_trip!(
        key_types_from_jsonable,
        KeyTypes::AnnotatedRelationshipElement,
        "AnnotatedRelationshipElement"
    );
}

#[test]
fn test_key_types_fail() {
    test_enum_fail!(key_types_from_jsonable, "invalid-literal");
}

#[test]
fn test_data_type_def_xsd_round_trip() {
    test_enum_round_trip!(
        data_type_def_xsd_from_jsonable,
        DataTypeDefXsd::AnyUri,
        "xs:anyURI"
    );
}

#[test]
fn test_data_type_def_xsd_fail() {
    test_enum_fail!(data_type_def_xsd_from_jsonable, "invalid-literal");
}

#[test]
fn test_data_type_iec61360_round_trip() {
    test_enum_round_trip!(
        data_type_iec61360_from_jsonable,
        DataTypeIec61360::Date,
        "DATE"
    );
}

#[test]
fn test_data_type_iec61360_fail() {
    test_enum_fail!(data_type_iec61360_from_jsonable, "invalid-literal");
}
