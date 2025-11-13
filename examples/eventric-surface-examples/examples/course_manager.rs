#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use std::{
    any::Any,
    sync::OnceLock,
};

use derive_more::Debug;
use eventric_stream::{
    error::Error,
    event::{
        Data,
        Identifier,
        PersistentEvent,
        Tag,
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
use eventric_surface_examples::{
    Decision,
    DeserializedPersistentEvent,
    Event,
    GetIdentifier,
    GetQuery,
    GetSpecifier as _,
    GetTags,
    Update,
};
use fancy_constructor::new;

// NOTES

// At least initially, event versioning will be ignored entirely (all versions
// will be set to zero for now, until a meaningful model is in place).

// -------------------------------------------------------------------------------------------------

// Theoretically Generated...

static COURSE_REGISTERED_IDENTIFIER: OnceLock<Identifier> = OnceLock::new();

impl<'a> Event<'a> for CourseRegistered {
    fn deserialize(_data: &'a Data) -> Result<Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl GetIdentifier for CourseRegistered {
    fn identifier() -> Result<&'static Identifier, Error> {
        COURSE_REGISTERED_IDENTIFIER.get_or_try_init(|| Identifier::new("course_registered"))
    }
}

impl GetTags for CourseRegistered {
    fn tags(&self) -> Result<Vec<Tag>, Error> {
        [Tag::new(format!("course_id:{}", self.id))]
            .into_iter()
            .collect()
    }
}

#[derive(Debug)]
pub enum CourseExistsEvent<'a> {
    CourseRegistered(&'a CourseRegistered),
}

impl<'a> Decision<'a> for CourseExists {
    type Event = CourseExistsEvent<'a>;

    fn filter_deserialize(event: &'a PersistentEvent) -> Result<Option<Box<dyn Any>>, Error> {
        let event = match event.identifier() {
            identifier if identifier == CourseRegistered::identifier()? => {
                let event = CourseRegistered::deserialize(event.data())?;
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

#[derive(new, Debug)]
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
            [Tag::new(format!("course_id:{}", self.id))?],
        )?])
    }
}

impl Update<'_> for CourseExists {
    fn update(&mut self, event: Self::Event) {
        match event {
            Self::Event::CourseRegistered(_event) => self.exists = true,
        }
    }
}

// Example...

pub fn main() -> Result<(), Error> {
    let mut stream = Stream::builder(eventric_stream::temp_path())
        .temporary(true)
        .open()?;

    let mut decision = CourseExists::new("some_course");

    let query = decision.query()?;
    let condition = query::Condition::default().matches(&query);

    let mut position = None;

    for event in stream.query(&condition, None) {
        let event = event?;

        position = Some(*event.position());

        if let Some(deserialized) = CourseExists::filter_deserialize(&event)? {
            let event = DeserializedPersistentEvent::new(deserialized, event);

            if let Some(event) = CourseExists::filter_map(&event)? {
                decision.update(event);
            }
        }
    }

    if !decision.exists {
        let mut condition = append::Condition::new(&query);

        if let Some(position) = position {
            condition = condition.after(position);
        }

        stream.append([], Some(&condition))?;
    }

    Ok(())
}
