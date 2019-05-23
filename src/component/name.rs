//! Component names

use std::fmt;

/// Name of an individual component
///
/// This should ideally match the Rust path name to the corresponding type.
// TODO(tarcieri): obtain this automatically via `std::module_path`?
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Name(pub &'static str);

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
