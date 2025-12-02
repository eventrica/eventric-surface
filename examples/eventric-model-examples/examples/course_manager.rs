#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use derive_more::Debug;
use eventric_model::{
    decision::{
        Decision,
        Events,
        Execute,
    },
    event::Event,
    projection::{
        Projection,
        Update,
        UpdateEvent,
    },
};
use eventric_stream::{
    error::Error,
    stream::{
        Stream,
        append::AppendSelect,
        iterate::IterateSelect,
    },
};
use fancy_constructor::new;
use revision::revisioned;

// =================================================================================================
// Course Manager
// =================================================================================================

// Events

#[revisioned(revision = 1)]
#[derive(new, Debug, Event)]
#[event(identifier(course_registered), tags(course(&this.id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

#[revisioned(revision = 1)]
#[derive(new, Debug, Event)]
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

impl Execute for RegisterCourse {
    fn execute(
        &mut self,
        events: &mut Events,
        projections: &Self::Projections,
    ) -> Result<(), Error> {
        if !projections.course_exists.exists {
            events.append(&CourseRegistered::new(&self.id, &self.title, self.capacity))?;
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Experimental...

#[derive(new, Debug)]
pub struct DecisionContext<'a> {
    stream: &'a mut Stream,
}

impl DecisionContext<'_> {
    pub fn execute<D>(&mut self, mut decision: D) -> Result<(), Error>
    where
        D: Decision,
    {
        let mut after = None;
        let mut projections = decision.projections();

        let selections = decision.select(&projections)?;

        let (events, select) = self.stream.iter_select(selections, None);

        for event in events {
            let event = event?;
            let position = *event.position();

            after = Some(position);

            decision.update(&event, &mut projections)?;
        }

        let mut events = Events::new();

        decision.execute(&mut events, &projections)?;

        let events = events.take();

        if !events.is_empty() {
            self.stream.append_select(events, select, after)?;
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Temporary Example Logic...

pub fn main() -> Result<(), Error> {
    let mut stream = Stream::builder("./temp").temporary(false).open()?;
    let mut context = DecisionContext::new(&mut stream);

    context.execute(RegisterCourse::new("my_course", "My Course", 30))
}
