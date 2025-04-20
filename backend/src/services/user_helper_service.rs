use async_trait::async_trait;

use crate::errors::user_service::UserServiceError;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

pub struct UserHelperService {
    user_repository: UserRepository,
}

impl UserHelperService {
    pub fn new() -> UserHelperService {
        UserHelperService {
            user_repository: UserRepository::new(),
        }
    }
}

#[async_trait]
pub trait UserHelperServiceTrait {
    // async fn hash_password(&self, raw_password: &String) -> Result<String, UserServiceError>;
    // async fn validate_password(
    //     &self,
    //     raw_password: &String,
    //     hash: &String,
    // ) -> Result<bool, UserServiceError>;
}

impl UserHelperServiceTrait for UserService {
    // async fn hash_password(&self, raw_password: &String) -> Result<String, UserServiceError> {
    //     todo!()
    // }
    // async fn validate_password(
    //     &self,
    //     raw_password: &String,
    //     hash: &String,
    // ) -> Result<bool, UserServiceError> {
    //     todo!()
    // }
}
