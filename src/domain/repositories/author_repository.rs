use rocket::{async_trait, serde::json::Json};

use crate::{domain::value_object::{author::{ReqAuthor, ResAuthor, ResAuthorList}, book::ResBookList}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}};

#[async_trait]
pub trait AuthorRepository {
    // get all author
    async fn index(&self,_user: AuthenticatedUser) -> Response<Json<ResAuthorList>>;

    // et athor by id
    async fn show(&self,_user: AuthenticatedUser, id: i32) -> Response<Json<ResAuthor>>;

    async fn update(&self,_user: AuthenticatedUser, _id: i32, _req_author: Json<ReqAuthor>) -> Response<Json<ResAuthor>>;

    async fn delete(&self,_user: AuthenticatedUser, _id: i32) -> Response<String>;

    async fn create(&self, _user: AuthenticatedUser, _req_author: Json<ReqAuthor>) -> Response<Json<ResAuthor>>;

    // get book by author_id
    async fn get_books(&self, _user:AuthenticatedUser, author_id:i32) -> Response<Json<ResBookList>>;
}