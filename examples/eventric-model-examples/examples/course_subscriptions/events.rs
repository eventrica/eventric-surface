use eventric_model::event::Event;
use fancy_constructor::new;
use revision::revisioned;

// =================================================================================================
// Course Subscriptions: Events
// =================================================================================================

#[revisioned(revision = 1)]
#[derive(new, Event, Debug)]
#[event(
    identifier(course_defined),
    tags(course(&this.id))
)]
pub struct CourseDefined {
    #[new(into)]
    pub id: String,
    pub capacity: u8,
}

#[revisioned(revision = 1)]
#[derive(new, Event, Debug)]
#[event(
    identifier(course_capacity_changed),
    tags(course(&this.id))
)]
pub struct CourseCapacityChanged {
    #[new(into)]
    pub id: String,
    pub new_capacity: u8,
}

#[revisioned(revision = 1)]
#[derive(new, Event, Debug)]
#[event(
    identifier(student_subscribed_to_course),
    tags(course(&this.course_id), student(&this.student_id))
)]
pub struct StudentSubscribedToCourse {
    #[new(into)]
    pub course_id: String,
    #[new(into)]
    pub student_id: String,
}
