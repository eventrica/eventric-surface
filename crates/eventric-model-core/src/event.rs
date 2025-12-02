//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

// pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod specifier;
pub(crate) mod tag;

use revision::{
    DeserializeRevisioned,
    SerializeRevisioned,
};

// =================================================================================================
// Event
// =================================================================================================

pub trait Event: DeserializeRevisioned + Identifier + Tags + SerializeRevisioned {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use revision::revisioned;

pub use self::{
    identifier::Identifier,
    specifier::Specifier,
    tag::Tags,
};
