use std::sync::Arc;

use rocket::http::Status;
use sea_orm_migration::MigratorTrait;
use udemy_mini_book::{ application::service::init_service_setup::init_service_setup, config::db_config::DatabaseConfig, domain::migrator::Migrator, infrastructure::{db::mysql_connection, rocket_http::{middleware::{cors::CORS, jwt_auth::AuthenticatedUser}, response_type::custom_response::{Response, SuccessResponse}, routes::init_route_setup::init_routes_setup}}};

#[macro_use] extern crate rocket;



#[get("/")]
fn index() -> Response<String>{
    Ok(SuccessResponse((Status::Ok, "Hi".to_string())))
}

#[get("/me")]
pub async fn me(user: AuthenticatedUser) -> Response<String> {
    Ok(SuccessResponse((
        Status::Ok,
        "My user ID is: ".to_string() + user.id.to_string().as_str(),
    )))
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // load database config
    let config = DatabaseConfig::default();
    // create database connection
    let db = mysql_connection::connect(&config).await.unwrap();
    // running migrator
    Migrator::up(&db, None).await.unwrap();
    let db_arc = Arc::new(db);
    rocket::build()
        .attach(CORS)
        .attach(init_service_setup(Arc::clone(&db_arc)))
        .mount("/", routes![index,me])
        .attach(init_routes_setup())
        .ignite().await?
        .launch().await?;

    Ok(())
}
