use std::sync::Arc;

use rocket::serde::json::Json;

use crate::{domain::{repositories::auth_repository::AuthRepository, value_object::auth::{ReqSignIn, ReqSignUp, ResMe, ResSignIn}}, infrastructure::rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}};

pub struct AuthService<T> 
where 
    T: AuthRepository + Send + Sync,
{
    auth_service: Arc<T>
}

impl<T> AuthService<T>
where 
    T: AuthRepository + Send + Sync,
{
    pub fn new(auth_service: Arc<T>) -> Self {
        Self {
            auth_service
        }
    }

    pub async fn sign_up(
        &self, register_data: Json<ReqSignUp>
    )
    -> Response<Json<String>>
    {
        // Simply delegate to the repository implementation
        self.auth_service.sign_up(register_data).await
    }
    pub  async fn sign_in(
        &self, sign_in_data: Json<ReqSignIn>
    )
    -> Response<Json<ResSignIn>>
    {
        self.auth_service.sign_in(sign_in_data).await
    }

    pub async fn me(&self, user: AuthenticatedUser) -> Response<Json<ResMe>>{
        self.auth_service.me(user).await
    }
}

