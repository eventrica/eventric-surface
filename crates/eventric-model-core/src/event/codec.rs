use eventric_stream::{
    error::Error,
    event::{
        self,
        CandidateEvent,
        Data,
        Version,
    },
};

use crate::event::Event;

// =================================================================================================
// Codec
// =================================================================================================

pub trait Codec {
    // Encode

    fn encode<E>(&self, event: E) -> Result<CandidateEvent, Error>
    where
        E: Event,
    {
        let data = self.encode_data(&event)?;

        let identifier = E::identifier().cloned()?;
        let tags = event.tags()?;
        let version = Version::default();

        Ok(CandidateEvent::new(data, identifier, tags, version))
    }

    fn encode_data<E>(&self, event: &E) -> Result<Data, Error>
    where
        E: Event;

    // Decode

    fn decode<E>(&self, event: &event::Event) -> Result<E, Error>
    where
        E: Event,
    {
        if event.identifier() != E::identifier()? {
            return Err(Error::data("Event Identifier Mismatch"));
        }

        self.decode_data(event.data())
    }

    fn decode_data<E>(&self, data: &Data) -> Result<E, Error>
    where
        E: Event;
}

// -------------------------------------------------------------------------------------------------

// JSON Codec

#[derive(Debug)]
pub struct JsonCodec;

impl Codec for JsonCodec {
    fn encode_data<E>(&self, event: &E) -> Result<Data, Error>
    where
        E: Event,
    {
        serde_json::to_vec(&event)
            .map_err(|_| Error::data("Serialization Error"))
            .and_then(Data::new)
    }

    fn decode_data<E>(&self, data: &Data) -> Result<E, Error>
    where
        E: Event,
    {
        serde_json::from_slice(data.as_ref()).map_err(|_| Error::data("Deserialization Error"))
    }
}
