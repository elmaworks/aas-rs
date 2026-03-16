//! Tests for `over_*` iteration functions on enumerations.
//! Mirrors aas-core3.0-typescript/test/types.overEnum.spec.ts

use aas_rs::*;

#[test]
fn test_over_modelling_kind() {
    let expected = vec![ModellingKind::Template, ModellingKind::Instance];
    let got: Vec<_> = over_modelling_kind().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_qualifier_kind() {
    let expected = vec![
        QualifierKind::ValueQualifier,
        QualifierKind::ConceptQualifier,
        QualifierKind::TemplateQualifier,
    ];
    let got: Vec<_> = over_qualifier_kind().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_asset_kind() {
    let expected = vec![
        AssetKind::Type,
        AssetKind::Instance,
        AssetKind::NotApplicable,
    ];
    let got: Vec<_> = over_asset_kind().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_aas_submodel_elements() {
    let expected = vec![
        AasSubmodelElements::AnnotatedRelationshipElement,
        AasSubmodelElements::BasicEventElement,
        AasSubmodelElements::Blob,
        AasSubmodelElements::Capability,
        AasSubmodelElements::DataElement,
        AasSubmodelElements::Entity,
        AasSubmodelElements::EventElement,
        AasSubmodelElements::File,
        AasSubmodelElements::MultiLanguageProperty,
        AasSubmodelElements::Operation,
        AasSubmodelElements::Property,
        AasSubmodelElements::Range,
        AasSubmodelElements::ReferenceElement,
        AasSubmodelElements::RelationshipElement,
        AasSubmodelElements::SubmodelElement,
        AasSubmodelElements::SubmodelElementList,
        AasSubmodelElements::SubmodelElementCollection,
    ];
    let got: Vec<_> = over_aas_submodel_elements().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_entity_type() {
    let expected = vec![EntityType::CoManagedEntity, EntityType::SelfManagedEntity];
    let got: Vec<_> = over_entity_type().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_direction() {
    let expected = vec![Direction::Input, Direction::Output];
    let got: Vec<_> = over_direction().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_state_of_event() {
    let expected = vec![StateOfEvent::On, StateOfEvent::Off];
    let got: Vec<_> = over_state_of_event().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_reference_types() {
    let expected = vec![
        ReferenceTypes::ExternalReference,
        ReferenceTypes::ModelReference,
    ];
    let got: Vec<_> = over_reference_types().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_key_types() {
    let expected = vec![
        KeyTypes::AnnotatedRelationshipElement,
        KeyTypes::AssetAdministrationShell,
        KeyTypes::BasicEventElement,
        KeyTypes::Blob,
        KeyTypes::Capability,
        KeyTypes::ConceptDescription,
        KeyTypes::DataElement,
        KeyTypes::Entity,
        KeyTypes::EventElement,
        KeyTypes::File,
        KeyTypes::FragmentReference,
        KeyTypes::GlobalReference,
        KeyTypes::Identifiable,
        KeyTypes::MultiLanguageProperty,
        KeyTypes::Operation,
        KeyTypes::Property,
        KeyTypes::Range,
        KeyTypes::Referable,
        KeyTypes::ReferenceElement,
        KeyTypes::RelationshipElement,
        KeyTypes::Submodel,
        KeyTypes::SubmodelElement,
        KeyTypes::SubmodelElementCollection,
        KeyTypes::SubmodelElementList,
    ];
    let got: Vec<_> = over_key_types().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_data_type_def_xsd() {
    let expected = vec![
        DataTypeDefXsd::AnyUri,
        DataTypeDefXsd::Base64Binary,
        DataTypeDefXsd::Boolean,
        DataTypeDefXsd::Byte,
        DataTypeDefXsd::Date,
        DataTypeDefXsd::DateTime,
        DataTypeDefXsd::Decimal,
        DataTypeDefXsd::Double,
        DataTypeDefXsd::Duration,
        DataTypeDefXsd::Float,
        DataTypeDefXsd::GDay,
        DataTypeDefXsd::GMonth,
        DataTypeDefXsd::GMonthDay,
        DataTypeDefXsd::GYear,
        DataTypeDefXsd::GYearMonth,
        DataTypeDefXsd::HexBinary,
        DataTypeDefXsd::Int,
        DataTypeDefXsd::Integer,
        DataTypeDefXsd::Long,
        DataTypeDefXsd::NegativeInteger,
        DataTypeDefXsd::NonNegativeInteger,
        DataTypeDefXsd::NonPositiveInteger,
        DataTypeDefXsd::PositiveInteger,
        DataTypeDefXsd::Short,
        DataTypeDefXsd::String,
        DataTypeDefXsd::Time,
        DataTypeDefXsd::UnsignedByte,
        DataTypeDefXsd::UnsignedInt,
        DataTypeDefXsd::UnsignedLong,
        DataTypeDefXsd::UnsignedShort,
    ];
    let got: Vec<_> = over_data_type_def_xsd().collect();
    assert_eq!(got, expected);
}

#[test]
fn test_over_data_type_iec61360() {
    let expected = vec![
        DataTypeIec61360::Date,
        DataTypeIec61360::String,
        DataTypeIec61360::StringTranslatable,
        DataTypeIec61360::IntegerMeasure,
        DataTypeIec61360::IntegerCount,
        DataTypeIec61360::IntegerCurrency,
        DataTypeIec61360::RealMeasure,
        DataTypeIec61360::RealCount,
        DataTypeIec61360::RealCurrency,
        DataTypeIec61360::Boolean,
        DataTypeIec61360::Iri,
        DataTypeIec61360::Irdi,
        DataTypeIec61360::Rational,
        DataTypeIec61360::RationalMeasure,
        DataTypeIec61360::Time,
        DataTypeIec61360::Timestamp,
        DataTypeIec61360::File,
        DataTypeIec61360::Html,
        DataTypeIec61360::Blob,
    ];
    let got: Vec<_> = over_data_type_iec61360().collect();
    assert_eq!(got, expected);
}
