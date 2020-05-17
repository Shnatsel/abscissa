//! Main entry point for the Abscissa CLI application

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use abscissa::application::APP;

/// Boot Abscissa CLI
fn main() {
    abscissa_core::boot(&APP);
}
