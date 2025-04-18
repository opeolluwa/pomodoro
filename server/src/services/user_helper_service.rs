use crate::errors::user_service::UserServiceError;
use crate::services::user::UserService;

pub struct UserHelperService {}

impl UserService {
    pub fn new() -> UserService {}
}

#[async_trait::async_trait]
pub trait UserHelperServiceTrait {
    async fn hash_password(raw_password: &str) -> Result<String, UserServiceError>;
    async fn validate_password(raw_password: &str, hash: &str) -> Result<bool, UserServiceError>;
}

impl UserHelperServiceTrait for UserService {
    async fn hash_password(raw_password: &str) -> String {
        todo!()
    }
    async fn validate_password(raw_password: &str, hash: &str) -> Result<bool, UserServiceError> {
        todo!()
    }
}
