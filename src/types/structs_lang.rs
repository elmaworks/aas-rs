use super::enums::DataTypeIec61360;
use super::structs_support::{LevelType, Reference, ValueList};

/// A language string for names.
#[derive(Debug, Clone)]
pub struct LangStringNameType {
    pub language: String,
    pub text: String,
}

impl LangStringNameType {
    /// Creates a new [`LangStringNameType`] with the given language and text.
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

/// A language string for text.
#[derive(Debug, Clone)]
pub struct LangStringTextType {
    pub language: String,
    pub text: String,
}

impl LangStringTextType {
    /// Creates a new [`LangStringTextType`] with the given language and text.
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

/// A language string for preferred names (IEC 61360).
#[derive(Debug, Clone)]
pub struct LangStringPreferredNameTypeIec61360 {
    pub language: String,
    pub text: String,
}

impl LangStringPreferredNameTypeIec61360 {
    /// Creates a new [`LangStringPreferredNameTypeIec61360`] with the given language and text.
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

/// A language string for short names (IEC 61360).
#[derive(Debug, Clone)]
pub struct LangStringShortNameTypeIec61360 {
    pub language: String,
    pub text: String,
}

impl LangStringShortNameTypeIec61360 {
    /// Creates a new [`LangStringShortNameTypeIec61360`] with the given language and text.
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

/// A language string for definitions (IEC 61360).
#[derive(Debug, Clone)]
pub struct LangStringDefinitionTypeIec61360 {
    pub language: String,
    pub text: String,
}

impl LangStringDefinitionTypeIec61360 {
    /// Creates a new [`LangStringDefinitionTypeIec61360`] with the given language and text.
    pub fn new(language: String, text: String) -> Self {
        Self { language, text }
    }
}

/// Data specification content following IEC 61360.
#[derive(Debug, Clone)]
pub struct DataSpecificationIec61360 {
    pub preferred_name: Vec<LangStringPreferredNameTypeIec61360>,
    pub short_name: Option<Vec<LangStringShortNameTypeIec61360>>,
    pub unit: Option<String>,
    pub unit_id: Option<Reference>,
    pub source_of_definition: Option<String>,
    pub symbol: Option<String>,
    pub data_type: Option<DataTypeIec61360>,
    pub definition: Option<Vec<LangStringDefinitionTypeIec61360>>,
    pub value_format: Option<String>,
    pub value_list: Option<ValueList>,
    pub value: Option<String>,
    pub level_type: Option<LevelType>,
}

impl DataSpecificationIec61360 {
    /// Creates a new [`DataSpecificationIec61360`] with the given preferred name.
    pub fn new(preferred_name: Vec<LangStringPreferredNameTypeIec61360>) -> Self {
        Self {
            preferred_name,
            short_name: None,
            unit: None,
            unit_id: None,
            source_of_definition: None,
            symbol: None,
            data_type: None,
            definition: None,
            value_format: None,
            value_list: None,
            value: None,
            level_type: None,
        }
    }
}
