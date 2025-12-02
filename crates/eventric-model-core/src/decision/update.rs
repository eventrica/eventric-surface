use eventric_stream::{
    error::Error,
    stream::select::EventMasked,
};

use crate::decision::projections::Projections;

// =================================================================================================
// Update
// =================================================================================================

pub trait Update: Projections {
    fn update(&self, event: &EventMasked, projections: &mut Self::Projections)
    -> Result<(), Error>;
}
