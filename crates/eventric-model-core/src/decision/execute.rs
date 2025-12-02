use eventric_stream::{
    error::Error,
    event::{
        CandidateEvent,
        Data,
        Version,
    },
};
use fancy_constructor::new;

use crate::{
    decision::Projections,
    event::Event,
};

// =================================================================================================
// Execute
// =================================================================================================

pub trait Execute: Projections {
    fn execute(
        &mut self,
        events: &mut Events,
        projections: &Self::Projections,
    ) -> Result<(), Error>;
}

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
