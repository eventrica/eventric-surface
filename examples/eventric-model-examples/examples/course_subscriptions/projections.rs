use derive_more::Debug;
use eventric_model::projection::{
    Projection,
    Update,
    UpdateEvent,
};
use fancy_constructor::new;

use crate::events::{
    CourseCapacityChanged,
    CourseDefined,
    StudentSubscribedToCourse,
};

// =================================================================================================
// Course Subscriptions: Projections
// =================================================================================================

// Projections

#[derive(new, Projection, Debug)]
#[projection(
    select(
        events(CourseDefined),
        filter(course(&this.id))
    )
)]
pub struct CourseExists {
    #[new(default)]
    pub exists: bool,
    #[new(into)]
    pub id: String,
}

impl Update<CourseDefined> for CourseExists {
    fn update(&mut self, _: UpdateEvent<'_, CourseDefined>) {
        self.exists = true;
    }
}

#[derive(new, Projection, Debug)]
#[projection(
    select(
        events(CourseDefined, CourseCapacityChanged),
        filter(course(&this.id))
    )
)]
pub struct CourseCapacity {
    #[new(default)]
    pub capacity: u8,
    #[new(into)]
    pub id: String,
}

impl Update<CourseDefined> for CourseCapacity {
    fn update(&mut self, event: UpdateEvent<'_, CourseDefined>) {
        self.capacity = event.capacity;
    }
}

impl Update<CourseCapacityChanged> for CourseCapacity {
    fn update(&mut self, event: UpdateEvent<'_, CourseCapacityChanged>) {
        self.capacity = event.new_capacity;
    }
}

#[derive(new, Projection, Debug)]
#[projection(
    select(
        events(StudentSubscribedToCourse),
        filter(course(&this.course_id), student(&this.student_id))
    )
)]
pub struct StudentAlreadySubscribed {
    #[new(default)]
    pub subscribed: bool,
    #[new(into)]
    pub course_id: String,
    #[new(into)]
    pub student_id: String,
}

impl Update<StudentSubscribedToCourse> for StudentAlreadySubscribed {
    fn update(&mut self, _: UpdateEvent<'_, StudentSubscribedToCourse>) {
        self.subscribed = true;
    }
}

#[derive(new, Projection, Debug)]
#[projection(
    select(
        events(StudentSubscribedToCourse),
        filter(course(&this.course_id))
    )
)]
pub struct NumberOfCourseSubscriptions {
    #[new(into)]
    pub course_id: String,
    #[new(default)]
    pub count: u8,
}

impl Update<StudentSubscribedToCourse> for NumberOfCourseSubscriptions {
    fn update(&mut self, _: UpdateEvent<'_, StudentSubscribedToCourse>) {
        self.count += 1;
    }
}

#[derive(new, Projection, Debug)]
#[projection(
    select(
        events(StudentSubscribedToCourse),
        filter(student(&this.student_id))
    )
)]
pub struct NumberOfStudentSubscriptions {
    #[new(into)]
    pub student_id: String,
    #[new(default)]
    pub count: u8,
}

impl Update<StudentSubscribedToCourse> for NumberOfStudentSubscriptions {
    fn update(&mut self, _: UpdateEvent<'_, StudentSubscribedToCourse>) {
        self.count += 1;
    }
}
