use eventric_stream::{
    error::Error,
    stream::select::Selections,
};

use crate::decision::projections::Projections;

// =================================================================================================
// Selection
// =================================================================================================

pub trait Select: Projections {
    fn select(&self, projections: &Self::Projections) -> Result<Selections, Error>;
}
