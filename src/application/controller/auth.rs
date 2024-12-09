use std::sync::Arc;

use rocket::{get, post, serde::json::Json, State};

use crate::{domain::value_object::auth::{ReqSignIn, ReqSignUp, ResMe, ResSignIn}, infrastructure::{db::repositories::auth_repository::AuthRepositoryImplSql, rocket_http::{middleware::jwt_auth::AuthenticatedUser, response_type::custom_response::Response}}, };
use crate::application::service::auth_service::AuthService;


#[post("/sign-in", data= "<req_sign_in>")]
pub async fn sign_in(
    req_sign_in: Json<ReqSignIn>,
    auth_service: &State<Arc<AuthService<AuthRepositoryImplSql>>>,
) 
-> Response<Json<ResSignIn>>
{
    auth_service.sign_in(req_sign_in).await
}

#[post("/sign-up", data= "<req_sign_up>")]
pub async fn sign_up(
    req_sign_up: Json<ReqSignUp>,
    auth_service: &State<Arc<AuthService<AuthRepositoryImplSql>>>,
) -> Response<Json<String>>
{
    auth_service.sign_up(req_sign_up).await
}

#[get("/me")]
pub async fn me(
    user: AuthenticatedUser,
    auth_service: &State<Arc<AuthService<AuthRepositoryImplSql>>>
) 
-> Response<Json<ResMe>>
{
    auth_service.me(user).await
}