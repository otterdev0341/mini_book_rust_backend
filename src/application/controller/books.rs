use std::sync::Arc;

use rocket::{delete, get, post, put, serde::json::Json, State};

use crate::{application::service::book_service::BookService, domain::value_object::book::{ReqBook, ResBook, ResBookList}, infrastructure::{db::repositories::book_repository::BookRepositoryImplSql, rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}}};

#[get("/")]
pub async fn index(
    user: AuthenticatedUser,
    book_service: &State<Arc<BookService<BookRepositoryImplSql>>>
) 
-> Response<Json<ResBookList>> {
    book_service.index(user).await
}

#[post("/", data = "<req_book>")]
pub async fn create(
    user: AuthenticatedUser,
    book_service: &State<Arc<BookService<BookRepositoryImplSql>>>,
    req_book: Json<ReqBook>
) -> Response<Json<ResBook>> {
    book_service.create(user, req_book).await
}

#[get("/<id>")]
pub async fn show(
    id: u32,
    user: AuthenticatedUser,
    book_service: &State<Arc<BookService<BookRepositoryImplSql>>>
) 
-> Response<Json<ResBook>> 
{
    book_service.show(user, id as i32).await
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(
    id: u32,
    req_book: Json<ReqBook>,
    user: AuthenticatedUser,
    book_service: &State<Arc<BookService<BookRepositoryImplSql>>>
) 
-> Response<Json<ResBook>> 
{
    book_service.update(user, id as i32, req_book).await
}

#[delete("/<id>")]
pub async fn delete(
    id: u32,
    user: AuthenticatedUser,
    book_service: &State<Arc<BookService<BookRepositoryImplSql>>>
) 
-> Response<String> 
{
    book_service.delete(user, id as i32).await
}