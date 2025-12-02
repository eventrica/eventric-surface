//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

use std::any::Any;

use derive_more::Deref;
use eventric_stream::{
    error::Error,
    event,
    stream::select::{
        EventMasked,
        Selection,
    },
};
use fancy_constructor::new;

use crate::event::Event;

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: Dispatch + Recognize + Select {}

// Dispatch

pub trait Dispatch {
    fn dispatch(&mut self, event: &DispatchEvent);
}

// Project

pub trait Project<E>
where
    E: Event,
{
    fn project(&mut self, event: ProjectionEvent<'_, E>);
}

// Recognize

pub trait Recognize {
    fn recognize(&self, event: &EventMasked) -> Result<Option<DispatchEvent>, Error>;
}

// Select

pub trait Select {
    fn select(&self) -> Result<Selection, Error>;
}

// -------------------------------------------------------------------------------------------------

// Dispatch Event

#[derive(new, Debug)]
#[new(const_fn, vis(pub(crate)))]
pub struct DispatchEvent {
    pub event: Box<dyn Any>,
    pub identifier: event::Identifier,
    pub position: event::Position,
    pub timestamp: event::Timestamp,
}

impl DispatchEvent {
    #[must_use]
    pub fn as_projection_event<E>(&self) -> Option<ProjectionEvent<'_, E>>
    where
        E: Event + 'static,
    {
        self.event
            .downcast_ref()
            .map(|inner_event| ProjectionEvent::new(inner_event, self.position, self.timestamp))
    }

    pub fn from_event<E>(event: &event::Event) -> Result<Self, Error>
    where
        E: Event + 'static,
    {
        revision::from_slice::<E>(event.data().as_ref())
            .map_err(|_| Error::data("deserialization error"))
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

// -------------------------------------------------------------------------------------------------

// Projection Event

#[derive(new, Debug, Deref)]
#[new(const_fn, vis(pub(crate)))]
pub struct ProjectionEvent<'a, E>
where
    E: Event,
{
    #[deref]
    event: &'a E,
    position: event::Position,
    timestamp: event::Timestamp,
}

impl<E> ProjectionEvent<'_, E>
where
    E: Event,
{
    #[must_use]
    pub fn position(&self) -> &event::Position {
        &self.position
    }

    #[must_use]
    pub fn timestamp(&self) -> &event::Timestamp {
        &self.timestamp
    }
}
