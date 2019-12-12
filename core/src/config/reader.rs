//! Mutex guard for immutably accessing global application configuration

use crate::application::{self, AppCell, Application};
use std::ops::Deref;

/// Convenience wrapper for `application::lock::Reader` for simplifying
/// access to global application configuration.
pub struct Reader<A>(application::lock::Reader<A>)
where
    A: 'static + Application;

impl<A> Reader<A>
where
    A: 'static + Application,
{
    /// Create wrapper around a read-only application mutex guard
    pub fn new(app_cell: &'static AppCell<A>) -> Self {
        Reader(app_cell.read())
    }
}

impl<A> Deref for Reader<A>
where
    A: 'static + Application,
{
    type Target = A::Cfg;

    fn deref(&self) -> &A::Cfg {
        self.0.config()
    }
}
