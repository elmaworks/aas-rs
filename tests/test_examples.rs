//! Tests for basic usage examples.
//! Mirrors aas-core3.0-typescript/test/examples.spec.ts

use aas_rs::types::class::Class;
use aas_rs::{Blob, DataTypeDefXsd, Property};

#[test]
fn test_types_match() {
    // Create a property
    let a_property = Class::Property(Property {
        value_type: DataTypeDefXsd::Int,
        semantic_id: None,
        supplemental_semantic_ids: None,
        extensions: None,
        category: None,
        id_short: None,
        display_name: None,
        description: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: None,
        value_id: None,
    });

    // Create a blob
    let a_blob = Class::Blob(Blob {
        content_type: "text/plain".to_string(),
        semantic_id: None,
        supplemental_semantic_ids: None,
        extensions: None,
        category: None,
        id_short: None,
        display_name: None,
        description: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: None,
    });

    // Create another property
    let another_property = Class::Property(Property {
        value_type: DataTypeDefXsd::Decimal,
        semantic_id: None,
        supplemental_semantic_ids: None,
        extensions: None,
        category: None,
        id_short: None,
        display_name: None,
        description: None,
        qualifiers: None,
        embedded_data_specifications: None,
        value: None,
        value_id: None,
    });

    // Check the type matches
    assert!(a_property.model_type() == a_property.model_type());
    assert!(a_property.model_type() != a_blob.model_type());
    assert!(a_property.model_type() == another_property.model_type());
}
