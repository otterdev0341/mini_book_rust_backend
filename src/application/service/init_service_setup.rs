use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::infrastructure::db::repositories::{auth_repository::AuthRepositoryImplSql, author_repository::AuthorRepositoryImplSql, book_repository::BookRepositoryImplSql};

use super::{auth_service::AuthService, author_service::AuthorService, book_service::BookService};


pub fn init_service_setup(db: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("All Services initialize",  move | rocket |async move {
        
        // define repository & service
        let auth_repository = AuthRepositoryImplSql {
            db_pool : Arc::clone(&db)
        };
        let auth_service = Arc::new(AuthService::new(Arc::new(auth_repository)));

        let book_repository = BookRepositoryImplSql {
            db_pool: Arc::clone(&db)
        };
        let user_service = Arc::new(BookService::new(Arc::new(book_repository)));

        let author_repository = AuthorRepositoryImplSql {
            db_pool: Arc::clone(&db)
        };

        let author_service = Arc::new(AuthorService::new(Arc::new(author_repository)));

        // attach to rocket
        rocket.manage(Arc::clone(&db))
            .manage(auth_service)
            .manage(user_service)
            .manage(author_service)
    })
}