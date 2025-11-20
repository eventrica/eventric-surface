use eventric_stream::{
    error::Error,
    event::PersistentEvent,
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
        event: &PersistentEvent,
        projections: &mut Self::Projections,
    ) -> Result<(), Error>
    where
        C: Codec;
}
