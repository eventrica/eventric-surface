use eventric_stream::{
    error::Error,
    event,
};

use crate::event::identifier::Identifier;

// =================================================================================================
// Specifier
// =================================================================================================

pub trait Specifier {
    fn specifier() -> Result<event::Specifier, Error>;
}

impl<T> Specifier for T
where
    T: Identifier,
{
    fn specifier() -> Result<event::Specifier, Error> {
        T::identifier().cloned().map(event::Specifier::new)
    }
}
