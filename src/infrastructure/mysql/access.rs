use sqlx::mysql::MySqlPoolOptions;
use sqlx::Pool;

use crate::application::traits::queryable::Queryable;

pub struct MySQLAccess {
    pool: Pool<sqlx::MySql>,
}

impl Queryable for MySQLAccess {
    type ConnType = sqlx::MySql;

    async fn new(connection: &str) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(connection)
            .await?;

        Ok(Self { pool })
    }
}
