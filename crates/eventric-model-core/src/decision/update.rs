use eventric_stream::{
    error::Error,
    stream::select::EventMasked,
};

use crate::{
    decision::projections::Projections,
    event::codec::Codec,
};

// =================================================================================================
// Update
// =================================================================================================

pub trait Update: Projections {
    fn update<C>(
        &self,
        codec: &C,
        event: &EventMasked,
        projections: &mut Self::Projections,
    ) -> Result<(), Error>
    where
        C: Codec;
}
