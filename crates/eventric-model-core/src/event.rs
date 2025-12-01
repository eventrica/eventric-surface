//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod specifier;
pub(crate) mod tag;

use serde::{
    Serialize,
    de::DeserializeOwned,
};

// =================================================================================================
// Event
// =================================================================================================

pub trait Event: DeserializeOwned + Identifier + Tags + Serialize {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    codec::{
        Codec,
        JsonCodec,
    },
    identifier::Identifier,
    specifier::Specifier,
    tag::Tags,
};
