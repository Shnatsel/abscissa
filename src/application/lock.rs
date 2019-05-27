//! Wrapper for thread-safe Abscissa `Application` state access.

mod reader;
mod writer;

pub use self::{reader::Reader, writer::Writer};
use super::Application;
use std::sync::{PoisonError, RwLock};

/// `RwLock` for accessing global `Application` state.
///
/// Supports multiple concurrent readers (immutable-only) with an exclusive
/// mutable writer.
pub struct Lock<A: Application>(RwLock<Option<A>>);

impl<A> Default for Lock<A>
where
    A: Application,
{
    /// Initialize application state to a default unloaded state
    fn default() -> Self {
        Lock(RwLock::new(None))
    }
}

impl<A: Application> Lock<A> {
    /// Get the global application state, acquiring a shared, read-only lock
    /// around it which permits concurrent access by multiple readers.
    ///
    /// If the application has not yet been initialized, calls `not_loaded()`.
    pub fn read(&'static self) -> Reader<A> {
        Reader::new(self.0.read().unwrap_or_else(|e| poisoned(e)))
    }

    /// Obtain an exclusive lock on the application state, allowing it to be
    /// accessed mutably.
    pub fn write(&'static self) -> Writer<A> {
        Writer::new(self.0.write().unwrap_or_else(|e| poisoned(e)))
    }

    /// Set the global application state to the given value
    pub fn set(&self, new_app: A) {
        let mut state = self.0.write().unwrap_or_else(|e| poisoned(e));
        *state = Some(new_app);
    }
}

/// Error handler called if the `RwLock` protecting the application state
/// has been poisoned.
///
/// This indicates a bug in the program accessing this type.
fn poisoned<Guard>(e: PoisonError<Guard>) -> ! {
    panic!(
        "Abscissa application state corrupted by unhandled crash: {}",
        e
    )
}
