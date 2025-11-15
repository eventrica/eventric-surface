#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]

// Temporary Event Wrapper

use std::any::Any;

use eventric_stream::{
    error::Error,
    event::{
        PersistentEvent,
        Specifier,
    },
    stream::query::Query,
};
use eventric_surface::event::Identified;
use fancy_constructor::new;

#[derive(new, Debug)]
pub struct DeserializedPersistentEvent {
    pub deserialized: Box<dyn Any>,
    pub event: PersistentEvent,
}

impl DeserializedPersistentEvent {
    pub fn deserialize_as<T>(&self) -> Result<Option<&T>, Error>
    where
        T: Identified + 'static,
    {
        if self.event.identifier() != T::identifier()? {
            return Ok(None);
        }

        Ok(self.deserialized.downcast_ref::<T>())
    }
}

// Temporary Test Traits

pub trait Decision<'a> {
    type Event;

    fn filter_deserialize(event: &'a PersistentEvent) -> Result<Option<Box<dyn Any>>, Error>;
    fn filter_map(event: &'a DeserializedPersistentEvent) -> Result<Option<Self::Event>, Error>;
}

pub trait Update<'a>: Decision<'a> {
    fn update(&mut self, event: Self::Event);
}

pub trait GetQuery {
    fn query(&self) -> Result<Query, Error>;
}

// Temporary Convenience Traits

pub trait GetSpecifier {
    fn specifier() -> Result<Specifier, Error>;
}

impl<T> GetSpecifier for T
where
    T: Identified,
{
    fn specifier() -> Result<Specifier, Error> {
        T::identifier().cloned().map(Specifier::new)
    }
}
