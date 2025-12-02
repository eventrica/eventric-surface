#![allow(clippy::multiple_crate_versions)]

// =================================================================================================
// Eventric Surface
// =================================================================================================

pub mod decision {
    pub use eventric_model_core::decision::{
        Context,
        Decision,
        Events,
        Execute,
        Select,
        Update,
    };
    pub use eventric_model_macros::Decision;
}

pub mod event {
    pub use eventric_model_core::event::{
        Event,
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
        Projection,
        Recognize,
        Select,
        Update,
        UpdateEvent,
    };
    pub use eventric_model_macros::Projection;
}

pub mod stream {
    pub use eventric_model_core::stream::Executor;
}
