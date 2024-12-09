use rocket::serde::{Deserialize, Serialize};

use crate::domain::entities::book;

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBookList {
    pub total: usize,
    pub books: Vec<ResBook>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
    pub author_id: i32,
    pub title: String,
    pub year: String,
    pub cover: String,
}

impl From<&book::Model> for ResBook {
    fn from(b: &book::Model) -> Self {
        Self {
            id: b.id,
            author_id: b.author_id,
            title: b.title.to_owned(),
            year: b.year.to_owned(),
            cover: b.cover.to_owned()
        }
    }
}