use std::{sync::Arc, time::SystemTime};

use rocket::{async_trait, http::Status, serde::json::Json};
use sea_orm::{prelude::DateTimeUtc, ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder, Set};

use crate::{domain::{entities::book, repositories::book_repository::BookRepository, value_object::book::{ReqBook, ResBook, ResBookList}}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::{ErrorResponse, Response, SuccessResponse}}};


use crate::domain::entities::book::Entity as Book;

pub struct BookRepositoryImplSql {
    pub db_pool: Arc<DatabaseConnection>
}


impl BookRepositoryImplSql {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            db_pool
        }
    }
}


#[async_trait]
impl BookRepository for BookRepositoryImplSql {
    // get all book
    async fn index(
        &self,
        _user: AuthenticatedUser
    )
    -> Response<Json<ResBookList>>
    {
        // get connection
        let conn = Arc::clone(&self.db_pool);

        
        // Fetch books from database
    match Book::find()
    .order_by_desc(book::Column::UpdatedAt)
    .all(&*conn)
    .await {
        Ok(result) => {
            // Transform the books into ResBook format
            let books: Vec<ResBook> = result
                .iter()
                .map(ResBook::from)
                .collect();

            // Create the response structure
            let result = ResBookList {
                total: books.len(),
                books,
            };

            // Return successful response
            Ok(SuccessResponse((
                Status::Ok,
                Json(result)
            )))
        },
        Err(_) => {
            // Handle database error
            
            Err(ErrorResponse((Status::InternalServerError,"Internal Error".to_string())))
        }
}

    }

    // find by id
    async fn show(
        &self,
        _user: AuthenticatedUser,
        id: i32
    ) -> Response<Json<ResBook>>
    {
        let conn = Arc::clone(&self.db_pool);

        let book = Book::find_by_id(id).one(&*conn).await;
        match book {
            Ok(Some(the_book)) => {
                Ok(SuccessResponse((
                    Status::Ok,
                    Json(ResBook::from(&the_book)),
                )))
            }
            Ok(None) => {
                Err(ErrorResponse((
                    Status::NotFound,
                    "no book found with the specified ID".to_string()
                )))
            }
            Err(_) => {
                Err(ErrorResponse((
                    Status::InternalServerError,
                    "Internal Error please try again".to_string()
                )))
            }
        }
    }

    async fn update(
        &self,
        _user: AuthenticatedUser, 
        id: i32, 
        req_book: Json<ReqBook>
    ) 
    -> Response<Json<ResBook>>
    {
        let conn = Arc::clone(&self.db_pool);

        // 1. Find the book and convert to ActiveModel
        // 1. Find the book
    let book_result = Book::find_by_id(id).one(&*conn).await;
    
    match book_result {
        Ok(Some(book)) => {
            // 2. Convert to ActiveModel
            let mut book_active: book::ActiveModel = book.into();
            
            // 3. Update fields
            book_active.author_id = Set(req_book.author_id as i32);
            book_active.title = Set(req_book.title.to_owned());
            book_active.year = Set(req_book.year.to_owned());
            book_active.cover = Set(req_book.cover.to_owned());
            book_active.updated_at = Set(Some(DateTimeUtc::from(SystemTime::now())));

            // 4. Save updates
            match book_active.update(&*conn).await  {
                Ok(updated_book) => {
                    Ok(SuccessResponse((
                        Status::Ok,
                        Json(ResBook::from(&updated_book)),
                    )))
                }
                Err(_) => {
                    Err(ErrorResponse((
                        Status::InternalServerError,
                        "Failed to update book".to_string()
                    )))
                }
            }
        }
        Ok(None) => {
            Err(ErrorResponse((
                Status::NotFound,
                "Book not found".to_string()
            )))
        }
        Err(_) => {
            Err(ErrorResponse((
                Status::InternalServerError,
                "Database error".to_string()
            )))
        }
    }
    }

    async fn delete(
        &self,
        _user: AuthenticatedUser, 
        id: i32
    ) 
    -> Response<String>{
        let conn = Arc::clone(&self.db_pool);

         // 1. Find the book
    match Book::find_by_id(id).one(&*conn).await {
        Ok(Some(book)) => {
            // 2. Delete the book
            match book.delete(&*conn).await {
                Ok(res) => {
                    if res.rows_affected == 1 {
                        Ok(SuccessResponse((
                            Status::Ok,
                            "Book successfully deleted".to_string()
                        )))
                    } else {
                        Err(ErrorResponse((
                            Status::InternalServerError,
                            "Failed to delete book".to_string()
                        )))
                    }
                }
                Err(_) => {
                    Err(ErrorResponse((
                        Status::InternalServerError,
                        "Error occurred while deleting book".to_string()
                    )))
                }
            }
        }
        Ok(None) => {
            Err(ErrorResponse((
                Status::NotFound,
                "Book not found".to_string()
            )))
        }
        Err(_) => {
            Err(ErrorResponse((
                Status::InternalServerError,
                "Database error".to_string()
            )))
        }
    }
    }
    async fn create(&self, user: AuthenticatedUser, req_book: Json<ReqBook>) -> Response<Json<ResBook>> {
        let conn = Arc::clone(&self.db_pool);

        let book = book::ActiveModel {
            user_id: Set(user.id as i32),
            author_id: Set(req_book.author_id.to_owned()),
            title: Set(req_book.title.to_owned()),
            year: Set(req_book.year.to_owned()),
            cover: Set(req_book.cover.to_owned()),
        ..Default::default()
        };

        // Insert with error handling
    match book.insert(&*conn).await {
        Ok(inserted_book) => {
            Ok(SuccessResponse((
                Status::Created,
                Json(ResBook::from(&inserted_book)),
            )))
        }
        Err(_) => {
            Err(ErrorResponse((
                Status::InternalServerError,
                "Failed to create book".to_string()
            )))
        }
    }
    }
}