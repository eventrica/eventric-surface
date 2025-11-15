#![allow(clippy::multiple_crate_versions)]

pub mod event {
    pub use eventric_surface_core::event::{
        Identified,
        Tagged,
    };
    pub use eventric_surface_macros::{
        Event,
        Identified,
        Tagged,
    };
}
