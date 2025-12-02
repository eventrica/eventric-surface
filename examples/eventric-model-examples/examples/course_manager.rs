#![feature(associated_type_defaults)]
#![feature(if_let_guard)]
#![feature(once_cell_try)]

use derive_more::Debug;
use eventric_model::{
    action::{
        Act,
        Action,
    },
    event::Event,
    projection::{
        Projection,
        Update,
        UpdateEvent,
    },
    stream::Enactor as _,
};
use eventric_stream::{
    error::Error,
    stream::Stream,
};
use fancy_constructor::new;
use revision::revisioned;

// =================================================================================================
// Course Manager
// =================================================================================================

// Events

#[revisioned(revision = 1)]
#[derive(new, Event, Debug)]
#[event(identifier(course_registered), tags(course(&this.id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

#[revisioned(revision = 1)]
#[derive(new, Event, Debug)]
#[event(identifier(course_withdrawn), tags(course(&this.id)))]
pub struct CourseWithdrawn {
    #[new(into)]
    pub id: String,
}

// Projections

#[derive(new, Projection, Debug)]
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

// Actions

#[derive(new, Action, Debug)]
#[action(projection(CourseExists: CourseExists::new(&this.id)))]
pub struct RegisterCourse {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

impl Act for RegisterCourse {
    type Err = Error;

    fn action(&mut self, context: &mut Self::Context) -> Result<Self::Ok, Self::Err> {
        if context.course_exists.exists {
            return Err(Error::data("Course Already Exists!"));
        }

        let course_registered = CourseRegistered::new(&self.id, &self.title, self.capacity);

        context.append(&course_registered)?;

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Example

pub fn main() -> Result<(), Error> {
    let mut stream = Stream::builder("./temp").open()?;

    let register_course = RegisterCourse::new("my_course", "My Course Title", 30);
    let register_course_result = stream.enact(register_course);

    println!("Register Course enacted with result: {register_course_result:?}");

    Ok(())
}
