use async_trait::async_trait;
use crate::errors::user_service::UserServiceError;
use crate::repositories::user::UserRepository;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

}

#[async_trait]
trait  UserServiceTrait{
    async fn create_user(&self, request: &CreateUserRequest) -> Result<bool, UserServiceError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user_account()
}
