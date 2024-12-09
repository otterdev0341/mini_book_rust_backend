
use std::sync::Arc;

use rocket::serde::json::Json;

use crate::{domain::{repositories::author_repository::AuthorRepository, value_object::{author::{ReqAuthor, ResAuthor, ResAuthorList}, book::ResBookList}}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}};

pub struct AuthorService<T>
where 
    T: AuthorRepository + Send + Sync,
{
    author_service: Arc<T>
}

impl<T> AuthorService<T>
where
    T: AuthorRepository + Send + Sync 
{
    pub fn new(author_service: Arc<T>) -> Self {
        Self {
            author_service
        }
    }

     // get all author
     pub async fn index(&self,user: AuthenticatedUser) -> Response<Json<ResAuthorList>> {
        self.author_service.index(user).await
     }

     // et athor by id
     pub async fn show(&self,user: AuthenticatedUser, id: i32) -> Response<Json<ResAuthor>> {
        self.author_service.show(user, id).await
     }
 
     pub async fn update(&self,user: AuthenticatedUser, id: i32, req_author: Json<ReqAuthor>) -> Response<Json<ResAuthor>> {
        self.author_service.update(user, id, req_author).await
     }
 
     pub async fn delete(&self,user: AuthenticatedUser, id: i32) -> Response<String> {
        self.author_service.delete(user, id).await
     }
 
     pub async fn create(&self, user: AuthenticatedUser, req_author: Json<ReqAuthor>) -> Response<Json<ResAuthor>>{
        self.author_service.create(user, req_author).await
     }
     pub  async fn get_books(&self, user:AuthenticatedUser, author_id:i32) -> Response<Json<ResBookList>>{
        self.author_service.get_books(user, author_id).await
     }


}