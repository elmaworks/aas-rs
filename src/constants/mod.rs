//! Constant values of the AAS meta-model.

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::{DataTypeIec61360, KeyTypes};

/// Categories for [`crate::types::structs_elements::Property`] as defined in Constraint AASd-090.
pub static VALID_CATEGORIES_FOR_DATA_ELEMENT: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert("CONSTANT");
    s.insert("PARAMETER");
    s.insert("VARIABLE");
    s
});

/// Keys that are only valid as fragment references.
pub static GENERIC_FRAGMENT_KEYS: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::FragmentReference);
    s
});

/// Keys that are globally identifiable outside of an AAS context.
pub static GENERIC_GLOBALLY_IDENTIFIABLES: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::GlobalReference);
    s
});

/// Keys for identifiable elements within an AAS.
pub static AAS_IDENTIFIABLES: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::AssetAdministrationShell);
    s.insert(KeyTypes::ConceptDescription);
    s.insert(KeyTypes::Identifiable);
    s.insert(KeyTypes::Submodel);
    s
});

/// Keys for all submodel element types.
pub static AAS_SUBMODEL_ELEMENTS_AS_KEYS: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::AnnotatedRelationshipElement);
    s.insert(KeyTypes::BasicEventElement);
    s.insert(KeyTypes::Blob);
    s.insert(KeyTypes::Capability);
    s.insert(KeyTypes::DataElement);
    s.insert(KeyTypes::Entity);
    s.insert(KeyTypes::EventElement);
    s.insert(KeyTypes::File);
    s.insert(KeyTypes::MultiLanguageProperty);
    s.insert(KeyTypes::Operation);
    s.insert(KeyTypes::Property);
    s.insert(KeyTypes::Range);
    s.insert(KeyTypes::ReferenceElement);
    s.insert(KeyTypes::RelationshipElement);
    s.insert(KeyTypes::SubmodelElement);
    s.insert(KeyTypes::SubmodelElementCollection);
    s.insert(KeyTypes::SubmodelElementList);
    s
});

/// Keys for referable but non-identifiable elements.
pub static AAS_REFERABLE_NON_IDENTIFIABLES: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::AnnotatedRelationshipElement);
    s.insert(KeyTypes::BasicEventElement);
    s.insert(KeyTypes::Blob);
    s.insert(KeyTypes::Capability);
    s.insert(KeyTypes::DataElement);
    s.insert(KeyTypes::Entity);
    s.insert(KeyTypes::EventElement);
    s.insert(KeyTypes::File);
    s.insert(KeyTypes::MultiLanguageProperty);
    s.insert(KeyTypes::Operation);
    s.insert(KeyTypes::Property);
    s.insert(KeyTypes::Range);
    s.insert(KeyTypes::ReferenceElement);
    s.insert(KeyTypes::RelationshipElement);
    s.insert(KeyTypes::SubmodelElement);
    s.insert(KeyTypes::SubmodelElementCollection);
    s.insert(KeyTypes::SubmodelElementList);
    s
});

/// All referable key types (identifiables + non-identifiables).
pub static AAS_REFERABLES: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::AssetAdministrationShell);
    s.insert(KeyTypes::ConceptDescription);
    s.insert(KeyTypes::Identifiable);
    s.insert(KeyTypes::Submodel);
    s.insert(KeyTypes::AnnotatedRelationshipElement);
    s.insert(KeyTypes::BasicEventElement);
    s.insert(KeyTypes::Blob);
    s.insert(KeyTypes::Capability);
    s.insert(KeyTypes::DataElement);
    s.insert(KeyTypes::Entity);
    s.insert(KeyTypes::EventElement);
    s.insert(KeyTypes::File);
    s.insert(KeyTypes::MultiLanguageProperty);
    s.insert(KeyTypes::Operation);
    s.insert(KeyTypes::Property);
    s.insert(KeyTypes::Range);
    s.insert(KeyTypes::ReferenceElement);
    s.insert(KeyTypes::Referable);
    s.insert(KeyTypes::RelationshipElement);
    s.insert(KeyTypes::SubmodelElement);
    s.insert(KeyTypes::SubmodelElementCollection);
    s.insert(KeyTypes::SubmodelElementList);
    s
});

/// All globally identifiable key types (AAS + generic).
pub static GLOBALLY_IDENTIFIABLES: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::GlobalReference);
    s.insert(KeyTypes::AssetAdministrationShell);
    s.insert(KeyTypes::ConceptDescription);
    s.insert(KeyTypes::Identifiable);
    s.insert(KeyTypes::Submodel);
    s
});

/// Keys valid as fragment keys (submodel elements + `FragmentReference`).
pub static FRAGMENT_KEYS: Lazy<HashSet<KeyTypes>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(KeyTypes::AnnotatedRelationshipElement);
    s.insert(KeyTypes::BasicEventElement);
    s.insert(KeyTypes::Blob);
    s.insert(KeyTypes::Capability);
    s.insert(KeyTypes::DataElement);
    s.insert(KeyTypes::Entity);
    s.insert(KeyTypes::EventElement);
    s.insert(KeyTypes::File);
    s.insert(KeyTypes::FragmentReference);
    s.insert(KeyTypes::MultiLanguageProperty);
    s.insert(KeyTypes::Operation);
    s.insert(KeyTypes::Property);
    s.insert(KeyTypes::Range);
    s.insert(KeyTypes::ReferenceElement);
    s.insert(KeyTypes::RelationshipElement);
    s.insert(KeyTypes::SubmodelElement);
    s.insert(KeyTypes::SubmodelElementCollection);
    s.insert(KeyTypes::SubmodelElementList);
    s
});

/// IEC 61360 data types for concept descriptions categorized with PROPERTY or VALUE.
pub static DATA_TYPE_IEC_61360_FOR_PROPERTY_OR_VALUE: Lazy<HashSet<DataTypeIec61360>> =
    Lazy::new(|| {
        let mut s = HashSet::new();
        s.insert(DataTypeIec61360::Date);
        s.insert(DataTypeIec61360::String);
        s.insert(DataTypeIec61360::StringTranslatable);
        s.insert(DataTypeIec61360::IntegerMeasure);
        s.insert(DataTypeIec61360::IntegerCount);
        s.insert(DataTypeIec61360::IntegerCurrency);
        s.insert(DataTypeIec61360::RealMeasure);
        s.insert(DataTypeIec61360::RealCount);
        s.insert(DataTypeIec61360::RealCurrency);
        s.insert(DataTypeIec61360::Boolean);
        s.insert(DataTypeIec61360::Rational);
        s.insert(DataTypeIec61360::RationalMeasure);
        s.insert(DataTypeIec61360::Time);
        s.insert(DataTypeIec61360::Timestamp);
        s
    });

/// IEC 61360 data types for concept descriptions categorized with REFERENCE.
pub static DATA_TYPE_IEC_61360_FOR_REFERENCE: Lazy<HashSet<DataTypeIec61360>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(DataTypeIec61360::String);
    s.insert(DataTypeIec61360::Iri);
    s.insert(DataTypeIec61360::Irdi);
    s
});

/// IEC 61360 data types for concept descriptions categorized with DOCUMENT.
pub static DATA_TYPE_IEC_61360_FOR_DOCUMENT: Lazy<HashSet<DataTypeIec61360>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(DataTypeIec61360::File);
    s.insert(DataTypeIec61360::Blob);
    s.insert(DataTypeIec61360::Html);
    s
});

/// IEC 61360 data types that imply a unit defined in the data specification.
pub static IEC_61360_DATA_TYPES_WITH_UNIT: Lazy<HashSet<DataTypeIec61360>> = Lazy::new(|| {
    let mut s = HashSet::new();
    s.insert(DataTypeIec61360::IntegerMeasure);
    s.insert(DataTypeIec61360::RealMeasure);
    s.insert(DataTypeIec61360::RationalMeasure);
    s.insert(DataTypeIec61360::IntegerCurrency);
    s.insert(DataTypeIec61360::RealCurrency);
    s
});
