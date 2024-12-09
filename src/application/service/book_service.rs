use std::sync::Arc;

use rocket::serde::json::Json;

use crate::{domain::{repositories::book_repository::BookRepository, value_object::book::{ReqBook, ResBook, ResBookList}}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}};

pub struct BookService<T>
where
    T: BookRepository + Send + Sync,
{
    book_service: Arc<T>
}

impl<T> BookService<T>
where 
    T : BookRepository + Send + Sync,
{
    
    pub fn new(book_service: Arc<T>) -> Self {
        Self {
            book_service
        }
    }

    pub async fn index(&self, user: AuthenticatedUser) -> Response<Json<ResBookList>>{
        self.book_service.index(user).await
    }

    pub async fn show(&self,user: AuthenticatedUser, id: i32) -> Response<Json<ResBook>>{
        self.book_service.show(user, id).await
    }

    pub async fn update(&self, user: AuthenticatedUser, id: i32, req_book: Json<ReqBook>) -> Response<Json<ResBook>>{
        self.book_service.update(user, id, req_book).await
    }

    pub async fn delete(&self, user: AuthenticatedUser, id: i32) -> Response<String>{
        self.book_service.delete(user, id).await
    }

    pub async fn create(&self, user: AuthenticatedUser, req_book: Json<ReqBook>) -> Response<Json<ResBook>> {
        self.book_service.create(user, req_book).await
    }
}