pub struct JwtSecret {
    pub jwt_secret: String
}

impl Default for JwtSecret {
    fn default() -> Self {
        Self {
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET").unwrap_or("meowmeow".to_string())
        }
    }
}