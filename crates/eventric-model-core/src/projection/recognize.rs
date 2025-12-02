use eventric_stream::{
    error::Error,
    stream::select::EventMasked,
};

use crate::projection::dispatch::DispatchEvent;

// =================================================================================================
// Recognise
// =================================================================================================

pub trait Recognize {
    fn recognize(&self, event: &EventMasked) -> Result<Option<DispatchEvent>, Error>;
}
