use rocket::async_trait;
use rocket::serde::json::Json;

use crate::domain::value_object::auth::{ReqSignIn, ReqSignUp, ResMe, ResSignIn};
use crate::infrastructure::rocket_http::middleware::jwt_auth::AuthenticatedUser;
use crate::infrastructure::rocket_http::response_type::custom_response::Response;
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn sign_up(&self, register_data: Json<ReqSignUp>) -> Response<Json<String>>;

    async fn sign_in(&self, sign_in_data: Json<ReqSignIn>) -> Response<Json<ResSignIn>>;

    async fn me(&self, user: AuthenticatedUser) -> Response<Json<ResMe>>;
}