use std::sync::Arc;

use rocket::{async_trait, http::Status, serde::json::Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder, Set};

use crate::{domain::{entities::author, entities::book, repositories::author_repository::AuthorRepository, value_object::{author::{ReqAuthor, ResAuthor, ResAuthorList}, book::{ResBook, ResBookList}}}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::{ErrorResponse, Response, SuccessResponse}}};
use crate::domain::entities::author::Entity as Author;
pub struct AuthorRepositoryImplSql {
    pub db_pool: Arc<DatabaseConnection>
}

impl AuthorRepositoryImplSql {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            db_pool
        }
    }
}

#[async_trait]
impl AuthorRepository for AuthorRepositoryImplSql {
    // get all author
    async fn index(
        &self,
        _user: AuthenticatedUser
    )
    -> Response<Json<ResAuthorList>>
    {
        let conn = Arc::clone(&self.db_pool);
        

        // fetc all author from the database
        match Author::find().order_by_desc(author::Column::UpdatedAt)
                          .all(&*conn)
                          .await {
                            Ok(result) => {
                                let authors: Vec<ResAuthor> = result.iter()
                                                                    .map(ResAuthor::from)
                                                                    .collect();
                                // prepare restonse
                                let result = ResAuthorList {
                                    total: authors.len(),
                                    authors
                                };

                                // return response
                                Ok(SuccessResponse((
                                    Status::Ok,
                                    Json(result)
                                )))
                            },
                            Err(_) => {
                                // handle database error
                                Err(ErrorResponse((Status::InternalServerError, "Internal Error".to_string())))
                            }
    }
    }

    // show author by id
    async fn show(
        &self,_user: AuthenticatedUser, 
        id: i32
    )
    -> Response<Json<ResAuthor>>
    {
        let conn = Arc::clone(&self.db_pool);

        let author = Author::find_by_id(id).one(&*conn).await;
        match author {
            Ok(Some(the_author)) => {
                Ok(SuccessResponse((
                    Status::Ok,
                    Json(ResAuthor::from(&the_author))
                )))
            }
            Ok(None) => {
                Err(ErrorResponse((
                    Status::NotFound,
                    "no author found with the specified Id".to_string()
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
        &self,_user: AuthenticatedUser, 
        id: i32,
        req_author: Json<ReqAuthor>
    )
    -> Response<Json<ResAuthor>>
    {
        let conn = Arc::clone(&self.db_pool);

        // find the author with specific id
        let author_result = Author::find_by_id(id).one(&*conn).await;

        match author_result {
            Ok(Some(author)) => {
                // get book model
                let mut author_active: author::ActiveModel = author.into();
                // update field
                author_active.firstname = Set(req_author.firstname.to_string());
                author_active.lastname = Set(req_author.lastname.to_string());
                author_active.bio = Set(req_author.bio.to_string());
                // save update
                match author_active.update(&*conn).await {
                   Ok(updated_author) => {
                    Ok(SuccessResponse((Status::Ok,Json(ResAuthor::from(&updated_author)))))
                   },
                   Err(_) => {
                    Err(ErrorResponse((
                        Status::InternalServerError,
                        "Failed to update author".to_string()
                    )))
                   }
                }
            }
            Ok(None) => {
                Err(ErrorResponse((
                    Status::NotFound,
                    "Author not found".to_string()
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
        &self,_user: AuthenticatedUser,
        id: i32,
    )
    -> Response<String>
    {
        // get connection
        let conn = Arc::clone(&self.db_pool);

        // find the author wiht this id
        match Author::find_by_id(id).one(&*conn).await {
            Ok(Some(author)) => {
                // delete author
                match author.delete(&*conn).await {
                    Ok(res) => {
                        if res.rows_affected == 1 {
                            Ok(SuccessResponse((
                                Status::Ok,
                                "Author successfully deleted".to_string()
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
                            "Error occurred while deleting author".to_string()
                        )))
                    }
                }
            }
            Ok(None) => {
                Err(ErrorResponse((
                    Status::NotFound,
                    "Author not found with specific id".to_string()
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

    async fn create(
        &self, _user: AuthenticatedUser,
        req_author: Json<ReqAuthor>
    )
    -> Response<Json<ResAuthor>>
    {
        // create connection to database
        let conn = Arc::clone(&self.db_pool);

        // create object to persist to db
        let author = author::ActiveModel{
            firstname: Set(req_author.firstname.to_owned()),
            lastname: Set(req_author.lastname.to_owned()),
            bio: Set(req_author.bio.to_owned()),
            ..Default::default()
        };

        // insert and catch the error
        match author.insert(&*conn).await {
            Ok(inserted_author) => {
                Ok(SuccessResponse((
                    Status::Created,
                    Json(ResAuthor::from(&inserted_author))
                )))
            }
            Err(_) => {
                Err(ErrorResponse((
                    Status::InternalServerError,
                    "Failed to create author".to_string()
                )))
            }
        }

    }

    async fn get_books(
        &self, _user:AuthenticatedUser, 
        author_id:i32
    ) 
    -> Response<Json<ResBookList>>
    {
        let conn = Arc::clone(&self.db_pool);
        // Find author and load related books
    match Author::find_by_id(author_id)
    .find_with_related(book::Entity)
    .all(&*conn)
    .await {
        Ok(author_with_books) => {
            if author_with_books.is_empty() {
                return Err(ErrorResponse((
                    Status::NotFound,
                    "Author not found".to_string()
                )));
            }

            // Convert books to ResBook format
            let books: Vec<ResBook> = author_with_books[0].1
                .iter()
                .map(|book| ResBook::from(book))
                .collect();

            Ok(SuccessResponse((
                Status::Ok,
                Json(ResBookList {
                    total: books.len(),
                    books
                })
            )))
        },
        Err(_) => {
            Err(ErrorResponse((
                Status::InternalServerError,
                "Failed to fetch books".to_string()
            )))
        }
}
    }
}