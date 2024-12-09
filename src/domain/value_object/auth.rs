use rocket::{serde::{Deserialize, Serialize}, Responder};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn{
    pub token: String
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}


#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    pub id: u32,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>
}