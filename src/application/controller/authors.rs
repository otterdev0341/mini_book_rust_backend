use std::sync::Arc;

use rocket::{delete, get, post, put, serde::json::Json, State};

use crate::{application::service::author_service::AuthorService, domain::value_object::{author::{ReqAuthor, ResAuthor, ResAuthorList}, book::ResBookList}, infrastructure::{db::repositories::author_repository::AuthorRepositoryImplSql, rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}}};

#[get("/")]
pub async fn index(
    user: AuthenticatedUser,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>
)
-> Response<Json<ResAuthorList>>
{
    author_service.index(user).await
}

#[post("/", data = "<req_author>")]
pub async fn create(
    user: AuthenticatedUser,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>,
    req_author: Json<ReqAuthor>
) 
-> Response<Json<ResAuthor>> {
    author_service.create(user, req_author).await
}

#[get("/<id>")]
pub async fn show(
    user: AuthenticatedUser,
    id: u32,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>
) -> Response<Json<ResAuthor>> {
    author_service.show(user, id as i32).await
}

#[put("/<id>", data = "<req_author>")]
pub async fn update(
    id: u32,
    req_author : Json<ReqAuthor>,
    user: AuthenticatedUser,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>
) 
-> Response<Json<ResAuthor>> {
    author_service.update(user, id as i32, req_author).await
}

#[delete("/<id>")]
pub async fn delete(
    id: u32,
    user: AuthenticatedUser,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>
)
-> Response<String> 
{
        author_service.delete(user, id as i32).await
}

#[get("/<author_id>/books")]
pub async fn get_books(
    author_id: u32,
    user: AuthenticatedUser,
    author_service: &State<Arc<AuthorService<AuthorRepositoryImplSql>>>
) 
-> Response<Json<ResBookList>> 
{
    author_service.get_books(user, author_id as i32).await
}