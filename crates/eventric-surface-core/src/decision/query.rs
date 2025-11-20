use eventric_stream::{
    error::Error,
    stream::query,
};

use crate::decision::projections::Projections;

// =================================================================================================
// Query
// =================================================================================================

pub trait Query: Projections {
    fn query(&self, projections: &Self::Projections) -> Result<query::Query, Error>;
}
