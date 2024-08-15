use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;

use crate::application::traits::collection_trait::{self, CollectionTrait};
use crate::application::traits::queryable::Queryable;
use crate::domain::models::collection::Collection;
use crate::DataAccess;

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
    fn get_collection_by_id(&self, _id: i32) -> Collection {
        return Collection::new();
    }
}
