#![allow(clippy::multiple_crate_versions)]

pub mod event {
    pub use eventric_surface_core::event::{
        Codec,
        Event,
        Identified,
        Tagged,
    };
    pub use eventric_surface_macros::{
        Event,
        Identified,
        Tagged,
    };

    pub mod json {
        pub use eventric_surface_core::event::JsonCodec as Codec;
    }
}
