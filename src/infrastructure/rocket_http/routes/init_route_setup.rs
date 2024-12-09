use rocket::fairing::AdHoc;

use super::{auth::auth_routes, authors:: authors_routes, books::books_routes};


pub fn init_routes_setup() -> AdHoc {
    AdHoc::on_ignite("Initialize routes", |rocket| async {
        rocket
            .mount("/auth", auth_routes())
            .mount("/authors", authors_routes())
            .mount("/books", books_routes())
    })
}