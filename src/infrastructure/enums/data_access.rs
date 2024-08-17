use crate::{
    application::traits::collection_trait::{self, CollectionTrait},
    domain::models::collection::Collection,
    infrastructure::{mysql::access::MySQLAccess, postgres::access::PostgresAccess},
};

pub enum DataAccess {
    Postgres(PostgresAccess),
    MYSQL(MySQLAccess),
}

impl CollectionTrait for DataAccess {
    async fn get_collection_by_id(&self, _id: i32) -> Result<Collection, sqlx::Error> {
        match self {
            DataAccess::Postgres(pg_access) => pg_access.get_collection_by_id(_id).await,
            DataAccess::MYSQL(my_access) => todo!(),
        }
    }

    async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        match self {
            DataAccess::Postgres(pg_access) => pg_access.get_collections().await,
            DataAccess::MYSQL(my_access) => todo!(),
        }
    }
}
