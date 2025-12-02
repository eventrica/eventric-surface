//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

use eventric_stream::{
    error::Error,
    event::{
        self,
        CandidateEvent,
        Data,
        Version,
    },
};
use fancy_constructor::new;
use revision::{
    DeserializeRevisioned,
    SerializeRevisioned,
};

// =================================================================================================
// Event
// =================================================================================================

// Event

pub trait Event: DeserializeRevisioned + Identifier + Tags + SerializeRevisioned {}

// Identifier

pub trait Identifier {
    fn identifier() -> Result<&'static event::Identifier, Error>;
}

// Specifier

pub trait Specifier {
    fn specifier() -> Result<event::Specifier, Error>;
}

impl<T> Specifier for T
where
    T: Identifier,
{
    fn specifier() -> Result<event::Specifier, Error> {
        T::identifier().cloned().map(event::Specifier::new)
    }
}

// Tags

pub trait Tags {
    fn tags(&self) -> Result<Vec<event::Tag>, Error>;
}

// -------------------------------------------------------------------------------------------------

// Events

#[derive(new, Debug)]
pub struct Events {
    #[new(default)]
    events: Vec<CandidateEvent>,
}

impl Events {
    pub fn append<E>(&mut self, event: &E) -> Result<(), Error>
    where
        E: Event,
    {
        let data = revision::to_vec(event).map_err(|_| Error::data("serialization error"))?;
        let data = Data::new(data)?;

        let identifier = E::identifier().cloned()?;
        let tags = event.tags()?;
        let version = Version::default();

        let event = CandidateEvent::new(data, identifier, tags, version);

        self.events.push(event);

        Ok(())
    }

    #[must_use]
    pub fn take(self) -> Vec<CandidateEvent> {
        self.events
    }
}
