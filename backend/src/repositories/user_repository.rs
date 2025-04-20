use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}
