use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Pool;
use sqlx::Row;

use crate::application::traits::collection_trait::CollectionTrait;
use crate::application::traits::queryable::Queryable;
use crate::domain::models::collection::Collection;

pub struct PostgresAccess {
    pool: Pool<sqlx::Postgres>,
}

impl Queryable for PostgresAccess {
    type ConnType = sqlx::Postgres;

    async fn new(connection: &str) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection)
            .await?;

        Ok(Self { pool })
    }
}

impl CollectionTrait for PostgresAccess {
    async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        sqlx::query("SELECT id, name, singleton FROM collection")
            .map(|row: PgRow| Collection {
                id: row.get("id"),
                name: row.get("name"),
                singleton: row.get("singleton"),
            })
            .fetch_all(&self.pool)
            .await
    }

    async fn get_collection_by_id(&self, _id: i32) -> Result<Collection, sqlx::Error> {
        sqlx::query("SELECT id, name, singleton FROM collection WHERE id = $1")
            .bind(_id)
            .map(|row: PgRow| Collection {
                id: row.get("id"),
                name: row.get("name"),
                singleton: row.get("singleton"),
            })
            .fetch_one(&self.pool)
            .await
    }
}
