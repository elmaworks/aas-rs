//! Verification error types.

use std::fmt;

/// Represents a segment in the path to an erroneous value.
#[derive(Debug, Clone)]
pub enum Segment {
    /// A property access segment.
    Property(String),
    /// An index access segment.
    Index(usize),
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Segment::Property(name) => write!(f, ".{name}"),
            Segment::Index(idx) => write!(f, "[{idx}]"),
        }
    }
}

/// Represents the relative path to the erroneous value.
#[derive(Debug, Clone, Default)]
pub struct Path {
    /// Segments of the path.
    pub segments: Vec<Segment>,
}

impl Path {
    /// Prepends a segment to the beginning of the path.
    pub fn prepend(&mut self, segment: Segment) {
        self.segments.insert(0, segment);
    }

    /// Prepends a property segment.
    pub fn prepend_property(&mut self, name: &str) {
        self.prepend(Segment::Property(name.to_string()));
    }

    /// Prepends an index segment.
    pub fn prepend_index(&mut self, index: usize) {
        self.prepend(Segment::Index(index));
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            write!(f, "{segment}")?;
        }
        Ok(())
    }
}

/// Represents a verification error in the data.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct VerificationError {
    /// Human-readable description of the error.
    pub message: String,
    /// Path to the erroneous value.
    pub path: Path,
}

impl VerificationError {
    /// Creates a new verification error with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        VerificationError {
            message: message.into(),
            path: Path::default(),
        }
    }

    /// Prepends a property segment to the error path.
    pub fn prepend_property(&mut self, name: &str) {
        self.path.prepend_property(name);
    }

    /// Prepends an index segment to the error path.
    pub fn prepend_index(&mut self, index: usize) {
        self.path.prepend_index(index);
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.segments.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "{}: {}", self.path, self.message)
        }
    }
}
