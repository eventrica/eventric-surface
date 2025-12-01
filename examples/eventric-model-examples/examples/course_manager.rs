#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use derive_more::Debug;
use eventric_model::{
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
#[event(identifier(course_registered), tags(course(&this.id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_withdrawn), tags(course(&this.id)))]
pub struct CourseWithdrawn {
    #[new(into)]
    pub id: String,
}

// Projections

#[derive(new, Debug, Projection)]
#[projection(select(events(CourseRegistered, CourseWithdrawn), filter(course(&this.id))))]
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

    let mut projections = eventric_model::decision::Projections::projections(&decision);

    println!("current projections state: {projections:#?}");

    let query = eventric_model::decision::Select::select(&decision, &projections)?;

    let mut position = None;

    println!("running decision query: {query:#?}");

    let (events, select) =
        eventric_stream::stream::iterate::IterateSelect::iter_select(&stream, query, None);

    for event in events {
        let event = event?;

        eventric_model::decision::Update::update(&decision, &codec, &event, &mut projections)?;

        position = Some(*event.position());
    }

    println!("making decision");
    println!("current projections state: {projections:#?}");

    if projections.course_exists.exists {
        println!("decision invalid, course already exists");
    } else {
        println!("decision valid, creating condition to append");

        println!("appending new events");

        let course_registered = CourseRegistered::new(course_id, "My Course", 30);
        let course_registered = eventric_model::event::Codec::encode(&codec, course_registered)?;

        println!("appending event: {course_registered:?}");

        eventric_stream::stream::append::AppendSelect::append_select(
            &mut stream,
            [course_registered],
            select,
            position,
        )?;
    }

    Ok(())
}
