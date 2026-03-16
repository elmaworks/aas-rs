//! Error types for JSON deserialization.

use std::fmt;

/// A segment in a path pointing to where deserialization failed.
#[derive(Debug, Clone)]
pub enum Segment {
    /// A JSON object property by its field name.
    Property(String),
    /// An array element by its index.
    Index(usize),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Property(name) => write!(f, ".{name}"),
            Segment::Index(i) => write!(f, "[{i}]"),
        }
    }
}

/// Error returned when deserializing an AAS value from JSON fails.
#[derive(Debug, thiserror::Error)]
pub struct DeserializationError {
    /// Human-readable description of what went wrong.
    pub message: String,
    /// Path from the root of the JSON value to the erroneous location,
    /// stored outermost-first (prepend new segments as you unwind the stack).
    pub path: Vec<Segment>,
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}", self.message)
        } else {
            let path_str: String = self
                .path
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            write!(f, "{}: {}", path_str, self.message)
        }
    }
}

impl DeserializationError {
    /// Create a new error with the given message and an empty path.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            path: Vec::new(),
        }
    }

    /// Prepend a [`Segment::Property`] to the error path.
    pub fn prepend_property(mut self, name: impl Into<String>) -> Self {
        self.path.insert(0, Segment::Property(name.into()));
        self
    }

    /// Prepend a [`Segment::Index`] to the error path.
    pub fn prepend_index(mut self, index: usize) -> Self {
        self.path.insert(0, Segment::Index(index));
        self
    }
}
