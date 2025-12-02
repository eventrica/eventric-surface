#![allow(clippy::multiple_crate_versions)]

// =================================================================================================
// Eventric Surface
// =================================================================================================

pub mod action {
    pub use eventric_model_core::action::{
        Act,
        Action,
        Context,
        Select,
        Update,
    };
    pub use eventric_model_macros::Action;
}

pub mod event {
    pub use eventric_model_core::event::{
        Event,
        Events,
        Identifier,
        Specifier,
        Tags,
    };
    pub use eventric_model_macros::Event;
}

pub mod projection {
    pub use eventric_model_core::projection::{
        Dispatch,
        DispatchEvent,
        Project,
        Projection,
        ProjectionEvent,
        Recognize,
        Select,
    };
    pub use eventric_model_macros::Projection;
}

pub mod stream {
    pub use eventric_model_core::stream::Enactor;
}
