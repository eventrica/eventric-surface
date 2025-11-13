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
        Data,
        Identifier,
        PersistentEvent,
        Specifier,
        Tag,
    },
    stream::query::Query,
};
use fancy_constructor::new;

#[derive(new, Debug)]
pub struct DeserializedPersistentEvent {
    pub deserialized: Box<dyn Any>,
    pub event: PersistentEvent,
}

impl DeserializedPersistentEvent {
    pub fn deserialize_as<T>(&self) -> Result<Option<&T>, Error>
    where
        T: GetIdentifier + 'static,
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

pub trait Event<'a> {
    fn deserialize(data: &'a Data) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait Update<'a>: Decision<'a> {
    fn update(&mut self, event: Self::Event);
}

pub trait GetIdentifier {
    fn identifier() -> Result<&'static Identifier, Error>;
}

pub trait GetQuery {
    fn query(&self) -> Result<Query, Error>;
}

pub trait GetTags {
    fn tags(&self) -> Result<Vec<Tag>, Error>;
}

// Temporary Convenience Traits

pub trait GetSpecifier {
    fn specifier() -> Result<Specifier, Error>;
}

impl<T> GetSpecifier for T
where
    T: GetIdentifier,
{
    fn specifier() -> Result<Specifier, Error> {
        T::identifier().cloned().map(Specifier::new)
    }
}
