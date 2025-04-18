use sqlx::Database;

pub struct UserRepository {
    database: Database,
}