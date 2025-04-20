use sqlx::{Pool, Postgres};

use crate::adapters::requests::user::CreateUserRequest;
use crate::errors::user_service::UserServiceError;
use crate::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
        }
    }
}

trait UserServiceTrait {
    async fn create_user_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<bool, UserServiceError>;
}

impl UserServiceTrait for UserService {
    async fn create_user_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<bool, UserServiceError> {
        todo!()
    }
}
