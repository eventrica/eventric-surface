use eventric_stream::{
    error::Error,
    event,
};

// =================================================================================================
// Identifier
// =================================================================================================

pub trait Identifier {
    fn identifier() -> Result<&'static event::Identifier, Error>;
}
