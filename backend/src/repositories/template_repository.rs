use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct TemplateRepository {
    pool: Arc<Pool<Postgres>>,
}

impl TemplateRepository {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            pool: Arc::new(pool.clone()),
        }
    }
}
