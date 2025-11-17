#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use std::any::Any;

use derive_more::Debug;
use eventric_stream::{
    error::Error,
    event::{
        PersistentEvent,
        tag,
    },
    stream::{
        Stream,
        append,
        query::{
            self,
            Query,
            Selector,
        },
    },
};
use eventric_surface::event::{
    Codec,
    Event,
    Identified as _,
    json,
};
use eventric_surface_examples::{
    Decision,
    DeserializedPersistentEvent,
    GetQuery,
    GetSpecifier as _,
    Update,
};
use fancy_constructor::new;
use serde::{
    Deserialize,
    Serialize,
};

// NOTES

// At least initially, event versioning will be ignored entirely (all versions
// will be set to zero for now, until a meaningful model is in place).

// -------------------------------------------------------------------------------------------------

// Theoretically Generated...

#[derive(Debug)]
pub enum CourseExistsEvent<'a> {
    CourseRegistered(&'a CourseRegistered),
}

impl<'a> Decision<'a> for CourseExists {
    type Event = CourseExistsEvent<'a>;

    fn filter_deserialize<C>(
        codec: &C,
        event: &'a PersistentEvent,
    ) -> Result<Option<Box<dyn Any>>, Error>
    where
        C: Codec,
    {
        let event = match event.identifier() {
            identifier if identifier == CourseRegistered::identifier()? => {
                let event = codec.decode::<CourseRegistered>(event)?;
                let event = Box::new(event) as Box<dyn Any>;

                Some(event)
            }
            _ => None,
        };

        Ok(event)
    }

    fn filter_map(event: &'a DeserializedPersistentEvent) -> Result<Option<Self::Event>, Error> {
        let event = match event {
            event if let Some(event) = event.deserialize_as::<CourseRegistered>()? => {
                let event = Self::Event::CourseRegistered(event);

                Some(event)
            }
            _ => None,
        };

        Ok(event)
    }
}

// -------------------------------------------------------------------------------------------------

// Events

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_registered), tags(course_id(id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

// Decisions

#[derive(new, Debug)]
pub struct CourseExists {
    #[new(default)]
    pub exists: bool,
    #[new(into)]
    pub id: String,
}

impl GetQuery for CourseExists {
    fn query(&self) -> Result<Query, Error> {
        Query::new([Selector::specifiers_and_tags(
            [CourseRegistered::specifier()?],
            [tag!(course_id, self.id)?],
        )?])
    }
}

impl Update<'_> for CourseExists {
    fn update(&mut self, event: Self::Event) {
        match event {
            Self::Event::CourseRegistered(_) => self.exists = true,
        }
    }
}

// Example...

pub fn main() -> Result<(), Error> {
    let codec = json::Codec;

    let mut stream = Stream::builder(eventric_stream::temp_path())
        .temporary(true)
        .open()?;

    let course_id = "some_course";

    println!("creating new decision");

    let mut decision = CourseExists::new(course_id);

    println!("current decision state: {decision:#?}");

    let query = decision.query()?;
    let condition = query::Condition::default().matches(&query);

    let mut position = None;

    println!("running decision query: {query:#?}");

    for event in stream.query(&condition, None) {
        let event = event?;

        position = Some(*event.position());

        if let Some(deserialized) = CourseExists::filter_deserialize(&codec, &event)? {
            let event = DeserializedPersistentEvent::new(deserialized, event);

            if let Some(event) = CourseExists::filter_map(&event)? {
                println!("applying update to decision: {event:#?}");

                decision.update(event);

                println!("current decision state: {decision:#?}");
            }
        }
    }

    println!("making decision");
    println!("current decision state: {decision:#?}");

    if decision.exists {
        println!("decision invalid, course already exists");
    } else {
        println!("decision valid, creating condition to append");

        let mut condition = append::Condition::new(&query);

        if let Some(position) = position {
            println!("extending append condition with after position");

            condition = condition.after(position);
        }

        println!("appending new events");

        let event = CourseRegistered::new(course_id, "My Course", 30);
        let event = codec.encode(event)?;

        println!("appending event: {event:#?}");

        stream.append([&event], Some(&condition))?;
    }

    Ok(())
}
