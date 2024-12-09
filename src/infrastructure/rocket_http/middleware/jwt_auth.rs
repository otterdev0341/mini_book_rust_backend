use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{
    http::Status,
    request::{self, FromRequest, Request}
};

use crate::{config::jwt_config::JwtSecret, infrastructure::rocket_http::jwt::jwt_claim::Claims};


pub struct AuthenticatedUser {
    pub id: u32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = req.headers().get_one("token") {
            let config = JwtSecret::default();

            let data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            let claims = match data {
                Ok(p) => p.claims,
                Err(_) => {
                    return request::Outcome::Error((Status::Unauthorized, "Invalid token".to_string()))
                }
            };

            request::Outcome::Success(AuthenticatedUser { id: claims.sub as u32 })
        } else {
            request::Outcome::Error((Status::Unauthorized, "Token absent".to_string()))
        }
    }}