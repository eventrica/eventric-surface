//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod dispatch;
pub(crate) mod recognize;
pub(crate) mod select;
pub(crate) mod update;

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: Dispatch + Recognize + Select {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    dispatch::{
        Dispatch,
        DispatchEvent,
    },
    recognize::Recognize,
    select::Select,
    update::{
        Update,
        UpdateEvent,
    },
};
