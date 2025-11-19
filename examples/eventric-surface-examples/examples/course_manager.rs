#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use derive_more::Debug;
use eventric_stream::{
    error::Error,
    stream::{
        Stream,
        append,
        query,
    },
};
use eventric_surface::{
    event::{
        Codec,
        Event,
        json,
    },
    projection::{
        Dispatch as _,
        Projection,
        Recognize,
        Update,
        UpdateEvent,
        query::Query as _,
    },
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

// #[derive(new, Debug, Deserialize, Event, Serialize)]
// #[event(identifier(course_removed), tags(course_id(id)))]
// pub struct CourseRemoved {
//     #[new(into)]
//     pub id: String,
// }

// Decisions

#[derive(new, Debug, Projection)]
#[projection(query(select(events(CourseRegistered), filter(course_id(id)))))]
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

// impl Update<CourseRemoved> for CourseExists {
//     fn update(&mut self, _: UpdateEvent<'_, CourseRemoved>) {
//         self.exists = false;
//     }
// }

// Example...

pub fn main() -> Result<(), Error> {
    let codec = json::Codec;

    let mut stream = Stream::builder(eventric_stream::temp_path())
        .temporary(true)
        .open()?;

    let course_id = "some_course";

    println!("creating new decision");

    let mut projection = CourseExists::new(course_id);

    println!("current decision state: {projection:#?}");

    let query = projection.query()?;
    let condition = query::Condition::default().matches(&query);

    let mut position = None;

    println!("running decision query: {query:#?}");

    for event in stream.query(&condition, None) {
        let event = event?;

        position = Some(*event.position());

        if let Some(event) = CourseExists::recognize(&codec, &event)? {
            projection.dispatch(&event);
        }
    }

    println!("making decision");
    println!("current decision state: {projection:#?}");

    if projection.exists {
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

        println!("appending event: {event:?}");

        stream.append([&event], Some(&condition))?;
    }

    Ok(())
}
