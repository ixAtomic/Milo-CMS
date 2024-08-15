use sqlx::Database;

pub trait Queryable {
    type ConnType: Database;

    async fn new(connection: &str) -> Result<Self, sqlx::Error>
    where
        Self: Sized;
}
