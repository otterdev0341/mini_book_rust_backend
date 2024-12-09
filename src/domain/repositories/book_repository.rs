

use rocket::{async_trait, serde::json::Json};

use crate::{domain::value_object::book::{ReqBook, ResBook, ResBookList}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}};

#[async_trait]
pub trait BookRepository {
    // get all book
    async fn index(&self,_user: AuthenticatedUser) -> Response<Json<ResBookList>>;

    async fn show(&self,_user: AuthenticatedUser, id: i32) -> Response<Json<ResBook>>;

    async fn update(&self,_user: AuthenticatedUser, _id: i32, _req_book: Json<ReqBook>) -> Response<Json<ResBook>>;

    async fn delete(&self,_user: AuthenticatedUser, _id: i32) -> Response<String>;

    async fn create(&self, _user: AuthenticatedUser, _req_book: Json<ReqBook>) -> Response<Json<ResBook>>;
}