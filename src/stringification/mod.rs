//! De/serialize enumerations from and to string representations.

use crate::{
    AasSubmodelElements, AssetKind, DataTypeDefXsd, DataTypeIec61360, Direction, EntityType,
    KeyTypes, ModelType, ModellingKind, QualifierKind, ReferenceTypes, StateOfEvent,
};

// ── ModelType ────────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`ModelType`].
///
/// Returns `None` if `text` is not a valid literal.
pub fn model_type_from_str(text: &str) -> Option<ModelType> {
    match text {
        "Extension" => Some(ModelType::Extension),
        "AdministrativeInformation" => Some(ModelType::AdministrativeInformation),
        "Qualifier" => Some(ModelType::Qualifier),
        "AssetAdministrationShell" => Some(ModelType::AssetAdministrationShell),
        "AssetInformation" => Some(ModelType::AssetInformation),
        "Resource" => Some(ModelType::Resource),
        "SpecificAssetId" => Some(ModelType::SpecificAssetId),
        "Submodel" => Some(ModelType::Submodel),
        "RelationshipElement" => Some(ModelType::RelationshipElement),
        "SubmodelElementList" => Some(ModelType::SubmodelElementList),
        "SubmodelElementCollection" => Some(ModelType::SubmodelElementCollection),
        "Property" => Some(ModelType::Property),
        "MultiLanguageProperty" => Some(ModelType::MultiLanguageProperty),
        "Range" => Some(ModelType::Range),
        "ReferenceElement" => Some(ModelType::ReferenceElement),
        "Blob" => Some(ModelType::Blob),
        "File" => Some(ModelType::File),
        "AnnotatedRelationshipElement" => Some(ModelType::AnnotatedRelationshipElement),
        "Entity" => Some(ModelType::Entity),
        "EventPayload" => Some(ModelType::EventPayload),
        "BasicEventElement" => Some(ModelType::BasicEventElement),
        "Operation" => Some(ModelType::Operation),
        "OperationVariable" => Some(ModelType::OperationVariable),
        "Capability" => Some(ModelType::Capability),
        "ConceptDescription" => Some(ModelType::ConceptDescription),
        "Reference" => Some(ModelType::Reference),
        "Key" => Some(ModelType::Key),
        "LangStringNameType" => Some(ModelType::LangStringNameType),
        "LangStringTextType" => Some(ModelType::LangStringTextType),
        "Environment" => Some(ModelType::Environment),
        "EmbeddedDataSpecification" => Some(ModelType::EmbeddedDataSpecification),
        "LevelType" => Some(ModelType::LevelType),
        "ValueReferencePair" => Some(ModelType::ValueReferencePair),
        "ValueList" => Some(ModelType::ValueList),
        "LangStringPreferredNameTypeIec61360" => {
            Some(ModelType::LangStringPreferredNameTypeIec61360)
        }
        "LangStringShortNameTypeIec61360" => Some(ModelType::LangStringShortNameTypeIec61360),
        "LangStringDefinitionTypeIec61360" => Some(ModelType::LangStringDefinitionTypeIec61360),
        "DataSpecificationIec61360" => Some(ModelType::DataSpecificationIec61360),
        _ => None,
    }
}

/// Translate [`ModelType`] to its string representation.
///
/// Returns `None` only when an unrecognised discriminant is encountered (should never happen
/// with valid enum values).
pub fn model_type_to_str(value: ModelType) -> Option<&'static str> {
    Some(must_model_type_to_str(value))
}

/// Translate [`ModelType`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`ModelType`] discriminant.
pub fn must_model_type_to_str(value: ModelType) -> &'static str {
    match value {
        ModelType::Extension => "Extension",
        ModelType::AdministrativeInformation => "AdministrativeInformation",
        ModelType::Qualifier => "Qualifier",
        ModelType::AssetAdministrationShell => "AssetAdministrationShell",
        ModelType::AssetInformation => "AssetInformation",
        ModelType::Resource => "Resource",
        ModelType::SpecificAssetId => "SpecificAssetId",
        ModelType::Submodel => "Submodel",
        ModelType::RelationshipElement => "RelationshipElement",
        ModelType::SubmodelElementList => "SubmodelElementList",
        ModelType::SubmodelElementCollection => "SubmodelElementCollection",
        ModelType::Property => "Property",
        ModelType::MultiLanguageProperty => "MultiLanguageProperty",
        ModelType::Range => "Range",
        ModelType::ReferenceElement => "ReferenceElement",
        ModelType::Blob => "Blob",
        ModelType::File => "File",
        ModelType::AnnotatedRelationshipElement => "AnnotatedRelationshipElement",
        ModelType::Entity => "Entity",
        ModelType::EventPayload => "EventPayload",
        ModelType::BasicEventElement => "BasicEventElement",
        ModelType::Operation => "Operation",
        ModelType::OperationVariable => "OperationVariable",
        ModelType::Capability => "Capability",
        ModelType::ConceptDescription => "ConceptDescription",
        ModelType::Reference => "Reference",
        ModelType::Key => "Key",
        ModelType::LangStringNameType => "LangStringNameType",
        ModelType::LangStringTextType => "LangStringTextType",
        ModelType::Environment => "Environment",
        ModelType::EmbeddedDataSpecification => "EmbeddedDataSpecification",
        ModelType::LevelType => "LevelType",
        ModelType::ValueReferencePair => "ValueReferencePair",
        ModelType::ValueList => "ValueList",
        ModelType::LangStringPreferredNameTypeIec61360 => "LangStringPreferredNameTypeIec61360",
        ModelType::LangStringShortNameTypeIec61360 => "LangStringShortNameTypeIec61360",
        ModelType::LangStringDefinitionTypeIec61360 => "LangStringDefinitionTypeIec61360",
        ModelType::DataSpecificationIec61360 => "DataSpecificationIec61360",
    }
}

// ── ModellingKind ─────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`ModellingKind`].
pub fn modelling_kind_from_str(text: &str) -> Option<ModellingKind> {
    match text {
        "Template" => Some(ModellingKind::Template),
        "Instance" => Some(ModellingKind::Instance),
        _ => None,
    }
}

/// Translate [`ModellingKind`] to its string representation.
pub fn modelling_kind_to_str(value: ModellingKind) -> Option<&'static str> {
    Some(must_modelling_kind_to_str(value))
}

/// Translate [`ModellingKind`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`ModellingKind`] discriminant.
pub fn must_modelling_kind_to_str(value: ModellingKind) -> &'static str {
    match value {
        ModellingKind::Template => "Template",
        ModellingKind::Instance => "Instance",
    }
}

// ── QualifierKind ─────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`QualifierKind`].
pub fn qualifier_kind_from_str(text: &str) -> Option<QualifierKind> {
    match text {
        "ValueQualifier" => Some(QualifierKind::ValueQualifier),
        "ConceptQualifier" => Some(QualifierKind::ConceptQualifier),
        "TemplateQualifier" => Some(QualifierKind::TemplateQualifier),
        _ => None,
    }
}

/// Translate [`QualifierKind`] to its string representation.
pub fn qualifier_kind_to_str(value: QualifierKind) -> Option<&'static str> {
    Some(must_qualifier_kind_to_str(value))
}

/// Translate [`QualifierKind`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`QualifierKind`] discriminant.
pub fn must_qualifier_kind_to_str(value: QualifierKind) -> &'static str {
    match value {
        QualifierKind::ValueQualifier => "ValueQualifier",
        QualifierKind::ConceptQualifier => "ConceptQualifier",
        QualifierKind::TemplateQualifier => "TemplateQualifier",
    }
}

// ── AssetKind ─────────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`AssetKind`].
pub fn asset_kind_from_str(text: &str) -> Option<AssetKind> {
    match text {
        "Type" => Some(AssetKind::Type),
        "Instance" => Some(AssetKind::Instance),
        "NotApplicable" => Some(AssetKind::NotApplicable),
        _ => None,
    }
}

/// Translate [`AssetKind`] to its string representation.
pub fn asset_kind_to_str(value: AssetKind) -> Option<&'static str> {
    Some(must_asset_kind_to_str(value))
}

/// Translate [`AssetKind`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`AssetKind`] discriminant.
pub fn must_asset_kind_to_str(value: AssetKind) -> &'static str {
    match value {
        AssetKind::Type => "Type",
        AssetKind::Instance => "Instance",
        AssetKind::NotApplicable => "NotApplicable",
    }
}

// ── AasSubmodelElements ───────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`AasSubmodelElements`].
pub fn aas_submodel_elements_from_str(text: &str) -> Option<AasSubmodelElements> {
    match text {
        "AnnotatedRelationshipElement" => Some(AasSubmodelElements::AnnotatedRelationshipElement),
        "BasicEventElement" => Some(AasSubmodelElements::BasicEventElement),
        "Blob" => Some(AasSubmodelElements::Blob),
        "Capability" => Some(AasSubmodelElements::Capability),
        "DataElement" => Some(AasSubmodelElements::DataElement),
        "Entity" => Some(AasSubmodelElements::Entity),
        "EventElement" => Some(AasSubmodelElements::EventElement),
        "File" => Some(AasSubmodelElements::File),
        "MultiLanguageProperty" => Some(AasSubmodelElements::MultiLanguageProperty),
        "Operation" => Some(AasSubmodelElements::Operation),
        "Property" => Some(AasSubmodelElements::Property),
        "Range" => Some(AasSubmodelElements::Range),
        "ReferenceElement" => Some(AasSubmodelElements::ReferenceElement),
        "RelationshipElement" => Some(AasSubmodelElements::RelationshipElement),
        "SubmodelElement" => Some(AasSubmodelElements::SubmodelElement),
        "SubmodelElementList" => Some(AasSubmodelElements::SubmodelElementList),
        "SubmodelElementCollection" => Some(AasSubmodelElements::SubmodelElementCollection),
        _ => None,
    }
}

/// Translate [`AasSubmodelElements`] to its string representation.
pub fn aas_submodel_elements_to_str(value: AasSubmodelElements) -> Option<&'static str> {
    Some(must_aas_submodel_elements_to_str(value))
}

/// Translate [`AasSubmodelElements`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`AasSubmodelElements`] discriminant.
pub fn must_aas_submodel_elements_to_str(value: AasSubmodelElements) -> &'static str {
    match value {
        AasSubmodelElements::AnnotatedRelationshipElement => "AnnotatedRelationshipElement",
        AasSubmodelElements::BasicEventElement => "BasicEventElement",
        AasSubmodelElements::Blob => "Blob",
        AasSubmodelElements::Capability => "Capability",
        AasSubmodelElements::DataElement => "DataElement",
        AasSubmodelElements::Entity => "Entity",
        AasSubmodelElements::EventElement => "EventElement",
        AasSubmodelElements::File => "File",
        AasSubmodelElements::MultiLanguageProperty => "MultiLanguageProperty",
        AasSubmodelElements::Operation => "Operation",
        AasSubmodelElements::Property => "Property",
        AasSubmodelElements::Range => "Range",
        AasSubmodelElements::ReferenceElement => "ReferenceElement",
        AasSubmodelElements::RelationshipElement => "RelationshipElement",
        AasSubmodelElements::SubmodelElement => "SubmodelElement",
        AasSubmodelElements::SubmodelElementList => "SubmodelElementList",
        AasSubmodelElements::SubmodelElementCollection => "SubmodelElementCollection",
    }
}

// ── EntityType ────────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`EntityType`].
pub fn entity_type_from_str(text: &str) -> Option<EntityType> {
    match text {
        "CoManagedEntity" => Some(EntityType::CoManagedEntity),
        "SelfManagedEntity" => Some(EntityType::SelfManagedEntity),
        _ => None,
    }
}

/// Translate [`EntityType`] to its string representation.
pub fn entity_type_to_str(value: EntityType) -> Option<&'static str> {
    Some(must_entity_type_to_str(value))
}

/// Translate [`EntityType`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`EntityType`] discriminant.
pub fn must_entity_type_to_str(value: EntityType) -> &'static str {
    match value {
        EntityType::CoManagedEntity => "CoManagedEntity",
        EntityType::SelfManagedEntity => "SelfManagedEntity",
    }
}

// ── Direction ─────────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`Direction`].
pub fn direction_from_str(text: &str) -> Option<Direction> {
    match text {
        "input" => Some(Direction::Input),
        "output" => Some(Direction::Output),
        _ => None,
    }
}

/// Translate [`Direction`] to its string representation.
pub fn direction_to_str(value: Direction) -> Option<&'static str> {
    Some(must_direction_to_str(value))
}

/// Translate [`Direction`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`Direction`] discriminant.
pub fn must_direction_to_str(value: Direction) -> &'static str {
    match value {
        Direction::Input => "input",
        Direction::Output => "output",
    }
}

// ── StateOfEvent ──────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`StateOfEvent`].
pub fn state_of_event_from_str(text: &str) -> Option<StateOfEvent> {
    match text {
        "on" => Some(StateOfEvent::On),
        "off" => Some(StateOfEvent::Off),
        _ => None,
    }
}

/// Translate [`StateOfEvent`] to its string representation.
pub fn state_of_event_to_str(value: StateOfEvent) -> Option<&'static str> {
    Some(must_state_of_event_to_str(value))
}

/// Translate [`StateOfEvent`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`StateOfEvent`] discriminant.
pub fn must_state_of_event_to_str(value: StateOfEvent) -> &'static str {
    match value {
        StateOfEvent::On => "on",
        StateOfEvent::Off => "off",
    }
}

// ── ReferenceTypes ────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`ReferenceTypes`].
pub fn reference_types_from_str(text: &str) -> Option<ReferenceTypes> {
    match text {
        "ExternalReference" => Some(ReferenceTypes::ExternalReference),
        "ModelReference" => Some(ReferenceTypes::ModelReference),
        _ => None,
    }
}

/// Translate [`ReferenceTypes`] to its string representation.
pub fn reference_types_to_str(value: ReferenceTypes) -> Option<&'static str> {
    Some(must_reference_types_to_str(value))
}

/// Translate [`ReferenceTypes`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`ReferenceTypes`] discriminant.
pub fn must_reference_types_to_str(value: ReferenceTypes) -> &'static str {
    match value {
        ReferenceTypes::ExternalReference => "ExternalReference",
        ReferenceTypes::ModelReference => "ModelReference",
    }
}

// ── KeyTypes ──────────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`KeyTypes`].
pub fn key_types_from_str(text: &str) -> Option<KeyTypes> {
    match text {
        "AnnotatedRelationshipElement" => Some(KeyTypes::AnnotatedRelationshipElement),
        "AssetAdministrationShell" => Some(KeyTypes::AssetAdministrationShell),
        "BasicEventElement" => Some(KeyTypes::BasicEventElement),
        "Blob" => Some(KeyTypes::Blob),
        "Capability" => Some(KeyTypes::Capability),
        "ConceptDescription" => Some(KeyTypes::ConceptDescription),
        "DataElement" => Some(KeyTypes::DataElement),
        "Entity" => Some(KeyTypes::Entity),
        "EventElement" => Some(KeyTypes::EventElement),
        "File" => Some(KeyTypes::File),
        "FragmentReference" => Some(KeyTypes::FragmentReference),
        "GlobalReference" => Some(KeyTypes::GlobalReference),
        "Identifiable" => Some(KeyTypes::Identifiable),
        "MultiLanguageProperty" => Some(KeyTypes::MultiLanguageProperty),
        "Operation" => Some(KeyTypes::Operation),
        "Property" => Some(KeyTypes::Property),
        "Range" => Some(KeyTypes::Range),
        "Referable" => Some(KeyTypes::Referable),
        "ReferenceElement" => Some(KeyTypes::ReferenceElement),
        "RelationshipElement" => Some(KeyTypes::RelationshipElement),
        "Submodel" => Some(KeyTypes::Submodel),
        "SubmodelElement" => Some(KeyTypes::SubmodelElement),
        "SubmodelElementCollection" => Some(KeyTypes::SubmodelElementCollection),
        "SubmodelElementList" => Some(KeyTypes::SubmodelElementList),
        _ => None,
    }
}

/// Translate [`KeyTypes`] to its string representation.
pub fn key_types_to_str(value: KeyTypes) -> Option<&'static str> {
    Some(must_key_types_to_str(value))
}

/// Translate [`KeyTypes`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`KeyTypes`] discriminant.
pub fn must_key_types_to_str(value: KeyTypes) -> &'static str {
    match value {
        KeyTypes::AnnotatedRelationshipElement => "AnnotatedRelationshipElement",
        KeyTypes::AssetAdministrationShell => "AssetAdministrationShell",
        KeyTypes::BasicEventElement => "BasicEventElement",
        KeyTypes::Blob => "Blob",
        KeyTypes::Capability => "Capability",
        KeyTypes::ConceptDescription => "ConceptDescription",
        KeyTypes::DataElement => "DataElement",
        KeyTypes::Entity => "Entity",
        KeyTypes::EventElement => "EventElement",
        KeyTypes::File => "File",
        KeyTypes::FragmentReference => "FragmentReference",
        KeyTypes::GlobalReference => "GlobalReference",
        KeyTypes::Identifiable => "Identifiable",
        KeyTypes::MultiLanguageProperty => "MultiLanguageProperty",
        KeyTypes::Operation => "Operation",
        KeyTypes::Property => "Property",
        KeyTypes::Range => "Range",
        KeyTypes::Referable => "Referable",
        KeyTypes::ReferenceElement => "ReferenceElement",
        KeyTypes::RelationshipElement => "RelationshipElement",
        KeyTypes::Submodel => "Submodel",
        KeyTypes::SubmodelElement => "SubmodelElement",
        KeyTypes::SubmodelElementCollection => "SubmodelElementCollection",
        KeyTypes::SubmodelElementList => "SubmodelElementList",
    }
}

// ── DataTypeDefXsd ────────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`DataTypeDefXsd`].
///
/// Strings use the `xs:` prefix (e.g. `"xs:string"`).
pub fn data_type_def_xsd_from_str(text: &str) -> Option<DataTypeDefXsd> {
    match text {
        "xs:anyURI" => Some(DataTypeDefXsd::AnyUri),
        "xs:base64Binary" => Some(DataTypeDefXsd::Base64Binary),
        "xs:boolean" => Some(DataTypeDefXsd::Boolean),
        "xs:byte" => Some(DataTypeDefXsd::Byte),
        "xs:date" => Some(DataTypeDefXsd::Date),
        "xs:dateTime" => Some(DataTypeDefXsd::DateTime),
        "xs:decimal" => Some(DataTypeDefXsd::Decimal),
        "xs:double" => Some(DataTypeDefXsd::Double),
        "xs:duration" => Some(DataTypeDefXsd::Duration),
        "xs:float" => Some(DataTypeDefXsd::Float),
        "xs:gDay" => Some(DataTypeDefXsd::GDay),
        "xs:gMonth" => Some(DataTypeDefXsd::GMonth),
        "xs:gMonthDay" => Some(DataTypeDefXsd::GMonthDay),
        "xs:gYear" => Some(DataTypeDefXsd::GYear),
        "xs:gYearMonth" => Some(DataTypeDefXsd::GYearMonth),
        "xs:hexBinary" => Some(DataTypeDefXsd::HexBinary),
        "xs:int" => Some(DataTypeDefXsd::Int),
        "xs:integer" => Some(DataTypeDefXsd::Integer),
        "xs:long" => Some(DataTypeDefXsd::Long),
        "xs:negativeInteger" => Some(DataTypeDefXsd::NegativeInteger),
        "xs:nonNegativeInteger" => Some(DataTypeDefXsd::NonNegativeInteger),
        "xs:nonPositiveInteger" => Some(DataTypeDefXsd::NonPositiveInteger),
        "xs:positiveInteger" => Some(DataTypeDefXsd::PositiveInteger),
        "xs:short" => Some(DataTypeDefXsd::Short),
        "xs:string" => Some(DataTypeDefXsd::String),
        "xs:time" => Some(DataTypeDefXsd::Time),
        "xs:unsignedByte" => Some(DataTypeDefXsd::UnsignedByte),
        "xs:unsignedInt" => Some(DataTypeDefXsd::UnsignedInt),
        "xs:unsignedLong" => Some(DataTypeDefXsd::UnsignedLong),
        "xs:unsignedShort" => Some(DataTypeDefXsd::UnsignedShort),
        _ => None,
    }
}

/// Translate [`DataTypeDefXsd`] to its string representation.
pub fn data_type_def_xsd_to_str(value: DataTypeDefXsd) -> Option<&'static str> {
    Some(must_data_type_def_xsd_to_str(value))
}

/// Translate [`DataTypeDefXsd`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`DataTypeDefXsd`] discriminant.
pub fn must_data_type_def_xsd_to_str(value: DataTypeDefXsd) -> &'static str {
    match value {
        DataTypeDefXsd::AnyUri => "xs:anyURI",
        DataTypeDefXsd::Base64Binary => "xs:base64Binary",
        DataTypeDefXsd::Boolean => "xs:boolean",
        DataTypeDefXsd::Byte => "xs:byte",
        DataTypeDefXsd::Date => "xs:date",
        DataTypeDefXsd::DateTime => "xs:dateTime",
        DataTypeDefXsd::Decimal => "xs:decimal",
        DataTypeDefXsd::Double => "xs:double",
        DataTypeDefXsd::Duration => "xs:duration",
        DataTypeDefXsd::Float => "xs:float",
        DataTypeDefXsd::GDay => "xs:gDay",
        DataTypeDefXsd::GMonth => "xs:gMonth",
        DataTypeDefXsd::GMonthDay => "xs:gMonthDay",
        DataTypeDefXsd::GYear => "xs:gYear",
        DataTypeDefXsd::GYearMonth => "xs:gYearMonth",
        DataTypeDefXsd::HexBinary => "xs:hexBinary",
        DataTypeDefXsd::Int => "xs:int",
        DataTypeDefXsd::Integer => "xs:integer",
        DataTypeDefXsd::Long => "xs:long",
        DataTypeDefXsd::NegativeInteger => "xs:negativeInteger",
        DataTypeDefXsd::NonNegativeInteger => "xs:nonNegativeInteger",
        DataTypeDefXsd::NonPositiveInteger => "xs:nonPositiveInteger",
        DataTypeDefXsd::PositiveInteger => "xs:positiveInteger",
        DataTypeDefXsd::Short => "xs:short",
        DataTypeDefXsd::String => "xs:string",
        DataTypeDefXsd::Time => "xs:time",
        DataTypeDefXsd::UnsignedByte => "xs:unsignedByte",
        DataTypeDefXsd::UnsignedInt => "xs:unsignedInt",
        DataTypeDefXsd::UnsignedLong => "xs:unsignedLong",
        DataTypeDefXsd::UnsignedShort => "xs:unsignedShort",
    }
}

// ── DataTypeIec61360 ──────────────────────────────────────────────────────────

/// Parse `text` as a string representation of [`DataTypeIec61360`].
pub fn data_type_iec_61360_from_str(text: &str) -> Option<DataTypeIec61360> {
    match text {
        "DATE" => Some(DataTypeIec61360::Date),
        "STRING" => Some(DataTypeIec61360::String),
        "STRING_TRANSLATABLE" => Some(DataTypeIec61360::StringTranslatable),
        "INTEGER_MEASURE" => Some(DataTypeIec61360::IntegerMeasure),
        "INTEGER_COUNT" => Some(DataTypeIec61360::IntegerCount),
        "INTEGER_CURRENCY" => Some(DataTypeIec61360::IntegerCurrency),
        "REAL_MEASURE" => Some(DataTypeIec61360::RealMeasure),
        "REAL_COUNT" => Some(DataTypeIec61360::RealCount),
        "REAL_CURRENCY" => Some(DataTypeIec61360::RealCurrency),
        "BOOLEAN" => Some(DataTypeIec61360::Boolean),
        "IRI" => Some(DataTypeIec61360::Iri),
        "IRDI" => Some(DataTypeIec61360::Irdi),
        "RATIONAL" => Some(DataTypeIec61360::Rational),
        "RATIONAL_MEASURE" => Some(DataTypeIec61360::RationalMeasure),
        "TIME" => Some(DataTypeIec61360::Time),
        "TIMESTAMP" => Some(DataTypeIec61360::Timestamp),
        "FILE" => Some(DataTypeIec61360::File),
        "HTML" => Some(DataTypeIec61360::Html),
        "BLOB" => Some(DataTypeIec61360::Blob),
        _ => None,
    }
}

/// Translate [`DataTypeIec61360`] to its string representation.
pub fn data_type_iec_61360_to_str(value: DataTypeIec61360) -> Option<&'static str> {
    Some(must_data_type_iec_61360_to_str(value))
}

/// Translate [`DataTypeIec61360`] to its string representation.
///
/// # Panics
///
/// Panics if `value` is not a valid [`DataTypeIec61360`] discriminant.
pub fn must_data_type_iec_61360_to_str(value: DataTypeIec61360) -> &'static str {
    match value {
        DataTypeIec61360::Date => "DATE",
        DataTypeIec61360::String => "STRING",
        DataTypeIec61360::StringTranslatable => "STRING_TRANSLATABLE",
        DataTypeIec61360::IntegerMeasure => "INTEGER_MEASURE",
        DataTypeIec61360::IntegerCount => "INTEGER_COUNT",
        DataTypeIec61360::IntegerCurrency => "INTEGER_CURRENCY",
        DataTypeIec61360::RealMeasure => "REAL_MEASURE",
        DataTypeIec61360::RealCount => "REAL_COUNT",
        DataTypeIec61360::RealCurrency => "REAL_CURRENCY",
        DataTypeIec61360::Boolean => "BOOLEAN",
        DataTypeIec61360::Iri => "IRI",
        DataTypeIec61360::Irdi => "IRDI",
        DataTypeIec61360::Rational => "RATIONAL",
        DataTypeIec61360::RationalMeasure => "RATIONAL_MEASURE",
        DataTypeIec61360::Time => "TIME",
        DataTypeIec61360::Timestamp => "TIMESTAMP",
        DataTypeIec61360::File => "FILE",
        DataTypeIec61360::Html => "HTML",
        DataTypeIec61360::Blob => "BLOB",
    }
}
