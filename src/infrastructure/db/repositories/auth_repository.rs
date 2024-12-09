
use std::sync::Arc;
use std::time::SystemTime;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::async_trait;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};

use crate::config::jwt_config::JwtSecret;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::domain::value_object::auth::{ReqSignIn, ReqSignUp, ResMe, ResSignIn};
use crate::infrastructure::rocket_http::jwt::jwt_claim::Claims;
use crate::infrastructure::rocket_http::middleware::jwt_auth::AuthenticatedUser;
use crate::infrastructure::rocket_http::response_type::custom_response::{ErrorResponse, Response, SuccessResponse};
use crate::domain::entities::{prelude::*, user};
pub struct AuthRepositoryImplSql {
    pub db_pool: Arc<DatabaseConnection>
}

impl AuthRepositoryImplSql {
    pub fn new(db_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            db_pool
        }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImplSql {
    // SIGN UP == REGISTER
    async fn sign_up(
        &self,
        register_data: Json<ReqSignUp>
    )
    -> Response<Json<String>>
{
        let conn = Arc::clone(&self.db_pool);
          // Start transaction
    let txn = conn.begin().await.map_err(|e| ErrorResponse((
        Status::InternalServerError,
        format!("Database error: {}", e),
    )))?;

           // Check existing user
    if let Ok(Some(_)) = User::find()
    .filter(user::Column::Email.eq(&register_data.email))
    .one(&*conn)
    .await 
{
    return Err(ErrorResponse((
        Status::UnprocessableEntity,
        "An account exists with this email address".to_string(),
    )));
}
    // Create user
    match User::insert(user::ActiveModel {
        email: Set(register_data.email.to_owned()),
        password: Set(hash(&register_data.password, DEFAULT_COST).map_err(|e| ErrorResponse((
            Status::InternalServerError,
            format!("Password hashing error: {}", e),
        )))?,),
        firstname: Set(register_data.firstname.to_owned()),
        lastname: Set(register_data.lastname.to_owned()),
        ..Default::default()
    })
    .exec(&*conn)
    .await
    {
        Ok(_) => {
            txn.commit().await.map_err(|e| ErrorResponse((
                Status::InternalServerError,
                format!("Transaction commit error: {}", e),
            )))?;
            
            Ok(SuccessResponse((
                Status::Created,
                Json("Account created successfully!".to_string())
            )))
        }
        Err(e) => Err(ErrorResponse((
            Status::InternalServerError,
            format!("Failed to create account: {}", e),
        )))
    }
    }






    // SIGN_IN == LOGIN
    async fn sign_in(&self, sign_in_data: Json<ReqSignIn>) -> Response<Json<ResSignIn>> {
        let conn = Arc::clone(&self.db_pool);
        let jwt_config = JwtSecret::default();

        // 1. Find user and handle database errors explicitly
        let user = User::find()
        .filter(user::Column::Email.eq(&sign_in_data.email))
        .one(&*conn)
        .await
        .map_err(|e| ErrorResponse((
            Status::InternalServerError,
            format!("Database error: {}", e)
        )))?;

    // 2. Check if user exists with better error message
    let user = user.ok_or_else(|| ErrorResponse((
        Status::Unauthorized,
        "Email or password is incorrect".to_string()
    )))?;

    // 3. Verify password with proper error handling
    let is_valid = verify(&sign_in_data.password, &user.password)
        .map_err(|e| ErrorResponse((
            Status::InternalServerError,
            format!("Password verification error: {}", e)
        )))?;

    if !is_valid {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Email or password is incorrect".to_string()
        )));
    }

    // 4. Generate JWT token with error handling
    let claims = Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| ErrorResponse((
                Status::InternalServerError,
                format!("Time error: {}", e)
            )))?
            .as_secs() + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.jwt_secret.as_bytes())
    ).map_err(|e| ErrorResponse((
        Status::InternalServerError,
        format!("Token generation error: {}", e)
    )))?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResSignIn { token })
    )))
    
    }



    async fn me(&self, user: AuthenticatedUser) -> Response<Json<ResMe>> {
        let conn = Arc::clone(&self.db_pool);
        
        // Find user by ID and handle potential errors
        let user_data = User::find_by_id(user.id as i32)
            .one(&*conn)
            .await
            .map_err(|e| ErrorResponse((
                Status::InternalServerError,
                format!("Database error: {}", e)
            )))?;
    
        // Check if user exists
        let user_data = user_data.ok_or_else(|| ErrorResponse((
            Status::NotFound,
            "User not found".to_string()
        )))?;
    
        // Return user data
        Ok(SuccessResponse((
            Status::Ok,
            Json(ResMe {
                id: user_data.id as u32,
                email: user_data.email,
                firstname: user_data.firstname,
                lastname: user_data.lastname
            })
        )))
    }


    
}