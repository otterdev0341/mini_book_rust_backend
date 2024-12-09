use rocket::serde::{Deserialize, Serialize};

use crate::domain::entities::author;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthor {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub bio: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorList {
    pub total: usize,
    pub authors: Vec<ResAuthor>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqAuthor {
    pub firstname: String,
    pub lastname: String,
    pub bio: String,
}

impl From<&author::Model> for ResAuthor {
    fn from(a: &author::Model) -> Self {
        Self {
            id: a.id,
            firstname: a.firstname.to_owned(),
            lastname: a.lastname.to_owned(),
            bio: a.bio.to_owned(),
        }
    }
}