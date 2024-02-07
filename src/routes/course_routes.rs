use axum::{routing::get, Router};

use crate::state::course_state::CourseState;

pub fn routes() -> Router<CourseState> {
    use crate::handler::course_handler::*;

    Router::new()
        .route("/my-course", get(get_my_courses))
        .route("/my-course/detail", get(get_my_courses_detail))
        .route("/my-course-code", get(get_my_course_codes))
        .route("/tree", get(get_course_tree))
}
