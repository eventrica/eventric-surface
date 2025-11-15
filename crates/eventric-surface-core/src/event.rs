#![allow(clippy::needless_continue)]

use eventric_stream::{
    error::Error,
    event::{
        Identifier,
        Tag,
    },
};

pub(crate) mod macros;

// =================================================================================================
// Event
// =================================================================================================

// Identified

pub trait Identified {
    fn identifier() -> Result<&'static Identifier, Error>;
}

// Tagged

pub trait Tagged {
    fn tags(&self) -> Result<Vec<Tag>, Error>;
}
