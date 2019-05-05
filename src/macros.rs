//! Macros defined by this crate

#[cfg(feature = "errors")]
#[macro_use]
pub mod error;

#[cfg(feature = "status")]
#[macro_use]
pub mod status;
