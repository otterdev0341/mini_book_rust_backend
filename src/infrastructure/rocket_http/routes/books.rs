use rocket::{routes, Route};

use crate::application::controller::books;

pub fn books_routes() -> Vec<Route> {
    routes![
        books::index,
        books::create,
        books::show,
        books::update,
        books::delete
    ]
}