/// Identifies the concrete type of an AAS element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    Extension = 0,
    AdministrativeInformation = 1,
    Qualifier = 2,
    AssetAdministrationShell = 3,
    AssetInformation = 4,
    Resource = 5,
    SpecificAssetId = 6,
    Submodel = 7,
    RelationshipElement = 8,
    SubmodelElementList = 9,
    SubmodelElementCollection = 10,
    Property = 11,
    MultiLanguageProperty = 12,
    Range = 13,
    ReferenceElement = 14,
    Blob = 15,
    File = 16,
    AnnotatedRelationshipElement = 17,
    Entity = 18,
    EventPayload = 19,
    BasicEventElement = 20,
    Operation = 21,
    OperationVariable = 22,
    Capability = 23,
    ConceptDescription = 24,
    Reference = 25,
    Key = 26,
    LangStringNameType = 27,
    LangStringTextType = 28,
    Environment = 29,
    EmbeddedDataSpecification = 30,
    LevelType = 31,
    ValueReferencePair = 32,
    ValueList = 33,
    LangStringPreferredNameTypeIec61360 = 34,
    LangStringShortNameTypeIec61360 = 35,
    LangStringDefinitionTypeIec61360 = 36,
    DataSpecificationIec61360 = 37,
}

/// Defines whether a particular element is a template or an instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModellingKind {
    Template = 0,
    Instance = 1,
}

/// Defines the kind of a qualifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualifierKind {
    ValueQualifier = 0,
    ConceptQualifier = 1,
    TemplateQualifier = 2,
}

/// Enumeration for denoting whether an asset is a type or an instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
    Type = 0,
    Instance = 1,
    NotApplicable = 2,
}

/// Enumeration of all submodel element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AasSubmodelElements {
    AnnotatedRelationshipElement = 0,
    BasicEventElement = 1,
    Blob = 2,
    Capability = 3,
    DataElement = 4,
    Entity = 5,
    EventElement = 6,
    File = 7,
    MultiLanguageProperty = 8,
    Operation = 9,
    Property = 10,
    Range = 11,
    ReferenceElement = 12,
    RelationshipElement = 13,
    SubmodelElement = 14,
    SubmodelElementList = 15,
    SubmodelElementCollection = 16,
}

/// Enumeration for entity types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    CoManagedEntity = 0,
    SelfManagedEntity = 1,
}

/// Direction of an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Input = 0,
    Output = 1,
}

/// State of an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateOfEvent {
    On = 0,
    Off = 1,
}

/// Enumeration for reference types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferenceTypes {
    ExternalReference = 0,
    ModelReference = 1,
}

/// Enumeration of all key types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyTypes {
    AnnotatedRelationshipElement = 0,
    AssetAdministrationShell = 1,
    BasicEventElement = 2,
    Blob = 3,
    Capability = 4,
    ConceptDescription = 5,
    DataElement = 6,
    Entity = 7,
    EventElement = 8,
    File = 9,
    FragmentReference = 10,
    GlobalReference = 11,
    Identifiable = 12,
    MultiLanguageProperty = 13,
    Operation = 14,
    Property = 15,
    Range = 16,
    Referable = 17,
    ReferenceElement = 18,
    RelationshipElement = 19,
    Submodel = 20,
    SubmodelElement = 21,
    SubmodelElementCollection = 22,
    SubmodelElementList = 23,
}

/// Enumeration of XSD data type definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataTypeDefXsd {
    AnyUri = 0,
    Base64Binary = 1,
    Boolean = 2,
    Byte = 3,
    Date = 4,
    DateTime = 5,
    Decimal = 6,
    Double = 7,
    Duration = 8,
    Float = 9,
    GDay = 10,
    GMonth = 11,
    GMonthDay = 12,
    GYear = 13,
    GYearMonth = 14,
    HexBinary = 15,
    Int = 16,
    Integer = 17,
    Long = 18,
    NegativeInteger = 19,
    NonNegativeInteger = 20,
    NonPositiveInteger = 21,
    PositiveInteger = 22,
    Short = 23,
    String = 24,
    Time = 25,
    UnsignedByte = 26,
    UnsignedInt = 27,
    UnsignedLong = 28,
    UnsignedShort = 29,
}

/// Enumeration of IEC 61360 data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataTypeIec61360 {
    Date = 0,
    String = 1,
    StringTranslatable = 2,
    IntegerMeasure = 3,
    IntegerCount = 4,
    IntegerCurrency = 5,
    RealMeasure = 6,
    RealCount = 7,
    RealCurrency = 8,
    Boolean = 9,
    Iri = 10,
    Irdi = 11,
    Rational = 12,
    RationalMeasure = 13,
    Time = 14,
    Timestamp = 15,
    File = 16,
    Html = 17,
    Blob = 18,
}

/// Returns an iterator over all [`ModelType`] literals.
pub fn over_model_type() -> impl Iterator<Item = ModelType> {
    [
        ModelType::Extension,
        ModelType::AdministrativeInformation,
        ModelType::Qualifier,
        ModelType::AssetAdministrationShell,
        ModelType::AssetInformation,
        ModelType::Resource,
        ModelType::SpecificAssetId,
        ModelType::Submodel,
        ModelType::RelationshipElement,
        ModelType::SubmodelElementList,
        ModelType::SubmodelElementCollection,
        ModelType::Property,
        ModelType::MultiLanguageProperty,
        ModelType::Range,
        ModelType::ReferenceElement,
        ModelType::Blob,
        ModelType::File,
        ModelType::AnnotatedRelationshipElement,
        ModelType::Entity,
        ModelType::EventPayload,
        ModelType::BasicEventElement,
        ModelType::Operation,
        ModelType::OperationVariable,
        ModelType::Capability,
        ModelType::ConceptDescription,
        ModelType::Reference,
        ModelType::Key,
        ModelType::LangStringNameType,
        ModelType::LangStringTextType,
        ModelType::Environment,
        ModelType::EmbeddedDataSpecification,
        ModelType::LevelType,
        ModelType::ValueReferencePair,
        ModelType::ValueList,
        ModelType::LangStringPreferredNameTypeIec61360,
        ModelType::LangStringShortNameTypeIec61360,
        ModelType::LangStringDefinitionTypeIec61360,
        ModelType::DataSpecificationIec61360,
    ]
    .into_iter()
}

/// Returns an iterator over all [`ModellingKind`] literals.
pub fn over_modelling_kind() -> impl Iterator<Item = ModellingKind> {
    [ModellingKind::Template, ModellingKind::Instance].into_iter()
}

/// Returns an iterator over all [`QualifierKind`] literals.
pub fn over_qualifier_kind() -> impl Iterator<Item = QualifierKind> {
    [
        QualifierKind::ValueQualifier,
        QualifierKind::ConceptQualifier,
        QualifierKind::TemplateQualifier,
    ]
    .into_iter()
}

/// Returns an iterator over all [`AssetKind`] literals.
pub fn over_asset_kind() -> impl Iterator<Item = AssetKind> {
    [AssetKind::Type, AssetKind::Instance, AssetKind::NotApplicable].into_iter()
}

/// Returns an iterator over all [`AasSubmodelElements`] literals.
pub fn over_aas_submodel_elements() -> impl Iterator<Item = AasSubmodelElements> {
    [
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
    ]
    .into_iter()
}

/// Returns an iterator over all [`EntityType`] literals.
pub fn over_entity_type() -> impl Iterator<Item = EntityType> {
    [EntityType::CoManagedEntity, EntityType::SelfManagedEntity].into_iter()
}

/// Returns an iterator over all [`Direction`] literals.
pub fn over_direction() -> impl Iterator<Item = Direction> {
    [Direction::Input, Direction::Output].into_iter()
}

/// Returns an iterator over all [`StateOfEvent`] literals.
pub fn over_state_of_event() -> impl Iterator<Item = StateOfEvent> {
    [StateOfEvent::On, StateOfEvent::Off].into_iter()
}

/// Returns an iterator over all [`ReferenceTypes`] literals.
pub fn over_reference_types() -> impl Iterator<Item = ReferenceTypes> {
    [ReferenceTypes::ExternalReference, ReferenceTypes::ModelReference].into_iter()
}

/// Returns an iterator over all [`KeyTypes`] literals.
pub fn over_key_types() -> impl Iterator<Item = KeyTypes> {
    [
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
    ]
    .into_iter()
}

/// Returns an iterator over all [`DataTypeDefXsd`] literals.
pub fn over_data_type_def_xsd() -> impl Iterator<Item = DataTypeDefXsd> {
    [
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
    ]
    .into_iter()
}

/// Returns an iterator over all [`DataTypeIec61360`] literals.
pub fn over_data_type_iec61360() -> impl Iterator<Item = DataTypeIec61360> {
    [
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
    ]
    .into_iter()
}
