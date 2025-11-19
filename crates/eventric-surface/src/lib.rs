#![allow(clippy::multiple_crate_versions)]

// =================================================================================================
// Eventric Surface
// =================================================================================================

pub mod event {
    pub use eventric_surface_core::event::{
        Codec,
        Event,
        Identifier,
        Specifier,
        Tags,
    };
    pub use eventric_surface_macros::{
        Event,
        Identifier,
        Tags,
    };

    pub mod json {
        pub use eventric_surface_core::event::JsonCodec as Codec;
    }
}

pub mod projection {
    pub use eventric_surface_core::projection::{
        Dispatch,
        DispatchEvent,
        Projection,
        Recognize,
        Update,
        UpdateEvent,
    };
    pub use eventric_surface_macros::{
        Dispatch,
        Projection,
        Recognize,
    };

    pub mod query {
        pub use eventric_surface_core::projection::query::Query;
        pub use eventric_surface_macros::Query;
    }
}
