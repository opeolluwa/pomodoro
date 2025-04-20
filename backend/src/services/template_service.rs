use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::repositories::template_repository::TemplateRepository;

#[derive(Clone)]

pub struct TemplateService {
    template_repository: TemplateRepository,
}

impl TemplateService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            template_repository: TemplateRepository::init(pool),
        }
    }
}
