use derive_more::Debug;
use eventric_model::projection::{
    Project,
    Projection,
    ProjectionEvent,
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

impl Project<CourseDefined> for CourseExists {
    fn project(&mut self, _: ProjectionEvent<'_, CourseDefined>) {
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

impl Project<CourseDefined> for CourseCapacity {
    fn project(&mut self, event: ProjectionEvent<'_, CourseDefined>) {
        self.capacity = event.capacity;
    }
}

impl Project<CourseCapacityChanged> for CourseCapacity {
    fn project(&mut self, event: ProjectionEvent<'_, CourseCapacityChanged>) {
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

impl Project<StudentSubscribedToCourse> for StudentAlreadySubscribed {
    fn project(&mut self, _: ProjectionEvent<'_, StudentSubscribedToCourse>) {
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

impl Project<StudentSubscribedToCourse> for NumberOfCourseSubscriptions {
    fn project(&mut self, _: ProjectionEvent<'_, StudentSubscribedToCourse>) {
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

impl Project<StudentSubscribedToCourse> for NumberOfStudentSubscriptions {
    fn project(&mut self, _: ProjectionEvent<'_, StudentSubscribedToCourse>) {
        self.count += 1;
    }
}
