use rocket::{routes, Route};

use crate::application::controller::auth;

pub fn auth_routes() -> Vec<Route> {
    routes![
        auth::sign_in,
        auth::sign_up,
        auth::me
    ]
}