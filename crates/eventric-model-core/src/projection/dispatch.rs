use std::any::Any;

use eventric_stream::{
    error::Error,
    event::{
        self,
        Identifier,
        Position,
        Timestamp,
    },
};
use fancy_constructor::new;

use crate::{
    event::{
        Codec,
        Event,
    },
    projection::update::UpdateEvent,
};

// =================================================================================================
// Dispatch
// =================================================================================================

pub trait Dispatch {
    fn dispatch(&mut self, event: &DispatchEvent);
}

// -------------------------------------------------------------------------------------------------

// Event

#[derive(new, Debug)]
#[new(const_fn, vis(pub(crate)))]
pub struct DispatchEvent {
    pub event: Box<dyn Any>,
    pub identifier: Identifier,
    pub position: Position,
    pub timestamp: Timestamp,
}

impl DispatchEvent {
    #[must_use]
    pub fn as_update_event<E>(&self) -> Option<UpdateEvent<'_, E>>
    where
        E: Event + 'static,
    {
        self.event
            .downcast_ref()
            .map(|inner_event| UpdateEvent::new(inner_event, self.position, self.timestamp))
    }

    pub fn from_event<C, E>(codec: &C, event: &event::Event) -> Result<Self, Error>
    where
        C: Codec,
        E: Event + 'static,
    {
        codec
            .decode::<E>(event)
            .map(|inner_event| Box::new(inner_event) as Box<dyn Any>)
            .map(|inner_event| {
                Self::new(
                    inner_event,
                    event.identifier().clone(),
                    *event.position(),
                    *event.timestamp(),
                )
            })
    }
}
