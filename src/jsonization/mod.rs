//! JSON serialization and deserialization for AAS classes.

mod deserialize;
pub mod error;
mod serialize;

pub use deserialize::*;
pub use error::DeserializationError;
pub use serialize::to_jsonable;
