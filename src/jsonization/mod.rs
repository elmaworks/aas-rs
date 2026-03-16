//! JSON serialization and deserialization for AAS classes.

pub mod error;
mod deserialize;
mod serialize;

pub use deserialize::*;
pub use error::DeserializationError;
pub use serialize::to_jsonable;
