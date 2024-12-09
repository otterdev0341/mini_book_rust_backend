use rocket::{routes, Route};

use crate::application::controller::authors;

pub fn authors_routes() -> Vec<Route> {
    routes![
        authors::index,
        authors::create,
        authors::show,
        authors::update,
        authors::delete,
        authors::get_books
    ]
}