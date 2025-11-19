use std::ops::Deref;

use eventric_stream::event::{
    Position,
    Timestamp,
};
use fancy_constructor::new;

use crate::event::Event;

// =================================================================================================
// Update
// =================================================================================================

pub trait Update<E>
where
    E: Event,
{
    fn update(&mut self, event: UpdateEvent<'_, E>);
}

// -------------------------------------------------------------------------------------------------

// Event

#[derive(new, Debug)]
#[new(const_fn, vis(pub(crate)))]
pub struct UpdateEvent<'a, E>
where
    E: Event,
{
    event: &'a E,
    position: Position,
    timestamp: Timestamp,
}

impl<E> UpdateEvent<'_, E>
where
    E: Event,
{
    #[must_use]
    pub fn position(&self) -> &Position {
        &self.position
    }

    #[must_use]
    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}

impl<E> Deref for UpdateEvent<'_, E>
where
    E: Event,
{
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.event
    }
}
