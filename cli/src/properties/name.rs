//! Names used within an Abscissa application

use ident_case::RenameRule;
use serde::{Deserialize, Serialize};
use std::{fmt, str};

/// Application name
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct App(String);

impl AsRef<str> for App {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for App {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(App(s.to_owned()))
    }
}

/// Author name
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Author(String);

impl AsRef<str> for Author {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for Author {
    fn from(s: String) -> Author {
        Author(s)
    }
}

/// Type names
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Type(String);

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Type {
    /// Create a new camel case name
    pub fn from_camel_case<S>(s: S) -> Type
    where
        S: ToString,
    {
        Type(s.to_string())
    }

    /// Inflect a snake case name into a type name
    pub fn from_snake_case<S>(s: S) -> Type
    where
        S: AsRef<str>,
    {
        Type(RenameRule::PascalCase.apply_to_field(s))
    }
}
