//! Verification module for AAS model constraints.
//!
//! This module validates AAS model instances against the constraints defined in
//! the AAS specification (AASd-xxx and AASc-3a-xxx constraint codes).
//!
//! # Examples
//!
//! ```rust,ignore
//! use aas_rs::verification;
//! use aas_rs::types::class::Class;
//!
//! let errors = verification::verify(&my_class, true);
//! if errors.is_empty() {
//!     println!("Valid!");
//! } else {
//!     for err in &errors {
//!         println!("{}", err);
//!     }
//! }
//! ```

pub mod error;
mod pattern;
mod rules;
mod string_constraints;
mod value;
mod verifier;

#[allow(clippy::module_name_repetitions)]
pub use error::VerificationError;
pub use value::is_leap_year;
pub use verifier::verify_class as verify;
