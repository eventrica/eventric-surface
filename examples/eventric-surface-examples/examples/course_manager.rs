#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use derive_more::Debug;
use eventric_surface::{
    decision::Decision,
    event::{
        Event,
        json,
    },
    projection::{
        Projection,
        Update,
        UpdateEvent,
    },
};
use fancy_constructor::new;
use serde::{
    Deserialize,
    Serialize,
};

// =================================================================================================
// Course Manager
// =================================================================================================

// Events

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_registered), tags(course_id(&this.id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_withdrawn), tags(course_id(&this.id)))]
pub struct CourseWithdrawn {
    #[new(into)]
    pub id: String,
}

// Projections

#[derive(new, Debug, Projection)]
#[projection(select(events(CourseRegistered, CourseWithdrawn), filter(course_id(&this.id))))]
pub struct CourseExists {
    #[new(default)]
    pub exists: bool,
    #[new(into)]
    pub id: String,
}

impl Update<CourseRegistered> for CourseExists {
    fn update(&mut self, _: UpdateEvent<'_, CourseRegistered>) {
        self.exists = true;
    }
}

impl Update<CourseWithdrawn> for CourseExists {
    fn update(&mut self, _: UpdateEvent<'_, CourseWithdrawn>) {
        self.exists = false;
    }
}

// Decisions

#[derive(new, Debug, Decision)]
#[decision(projection(CourseExists: CourseExists::new(&this.id)))]
pub struct RegisterCourse {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

// -------------------------------------------------------------------------------------------------

// Temporary Manual Implementations

// NOTES

// At least initially, event versioning will be ignored entirely (all versions
// will be set to zero for now, until a meaningful model is in place).

impl eventric_surface::decision::Projections for RegisterCourse {
    type Projections = RegisterCourseProjections;

    fn projections(&self) -> Self::Projections {
        Self::Projections::new(self)
    }
}

impl eventric_surface::decision::Query for RegisterCourse {
    fn query(
        &self,
        projections: &Self::Projections,
    ) -> Result<eventric_stream::stream::query::Query, eventric_stream::error::Error> {
        eventric_surface::projection::Query::query(&projections.course_exists)
    }
}

impl eventric_surface::decision::Update for RegisterCourse {
    fn update<C>(
        &self,
        codec: &C,
        event: &eventric_stream::event::PersistentEvent,
        projections: &mut Self::Projections,
    ) -> Result<(), eventric_stream::error::Error>
    where
        C: eventric_surface::event::Codec,
    {
        let mut dispatch_event = None;

        // Repeat per projection

        {
            if dispatch_event.is_none() {
                dispatch_event = eventric_surface::projection::Recognize::recognize(
                    &projections.course_exists,
                    codec,
                    event,
                )?;
            }

            if let Some(dispatch_event) = dispatch_event {
                eventric_surface::projection::Dispatch::dispatch(
                    &mut projections.course_exists,
                    &dispatch_event,
                );
            }
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Temporary Example Logic...

pub fn main() -> Result<(), eventric_stream::error::Error> {
    let codec = json::Codec;

    let mut stream = eventric_stream::stream::Stream::builder(eventric_stream::temp_path())
        .temporary(true)
        .open()?;

    let course_id = "some_course";

    println!("creating decision");

    let decision = RegisterCourse::new(course_id, "My Course", 30);

    println!("creating projections");

    let mut projections = eventric_surface::decision::Projections::projections(&decision);

    println!("current projections state: {projections:#?}");

    let query = eventric_surface::decision::Query::query(&decision, &projections)?;
    let condition = eventric_stream::stream::query::Condition::default().matches(&query);

    let mut position = None;

    println!("running decision query: {query:#?}");

    for event in stream.query(&condition, None) {
        let event = event?;

        eventric_surface::decision::Update::update(&decision, &codec, &event, &mut projections)?;

        position = Some(*event.position());
    }

    println!("making decision");
    println!("current projections state: {projections:#?}");

    if projections.course_exists.exists {
        println!("decision invalid, course already exists");
    } else {
        println!("decision valid, creating condition to append");

        let mut condition = eventric_stream::stream::append::Condition::new(&query);

        if let Some(position) = position {
            println!("extending append condition with after position");

            condition = condition.after(position);
        }

        println!("appending new events");

        let event = CourseRegistered::new(course_id, "My Course", 30);
        let event = eventric_surface::event::Codec::encode(&codec, event)?;

        println!("appending event: {event:?}");

        stream.append([&event], Some(&condition))?;
    }

    Ok(())
}
