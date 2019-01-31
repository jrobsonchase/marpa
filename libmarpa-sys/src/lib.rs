//! Generated bindings to [libmarpa]
//!
//! [libmarpa]: https://jeffreykegler.github.io/Marpa-web-site/libmarpa.html

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

mod raw {
    include!(concat!(env!("OUT_DIR"), "/raw.rs"));
}
pub use crate::raw::*;

#[cfg(test)]
mod test;
