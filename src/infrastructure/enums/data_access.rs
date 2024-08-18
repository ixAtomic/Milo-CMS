use crate::{
    application::traits::collection_trait::{self, CollectionTrait},
    application::traits::field_trait::FieldTrait,
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

impl FieldTrait for DataAccess {
    async fn get_field(
        &self,
        _id: i32,
    ) -> Result<crate::domain::models::field::Field, sqlx::Error> {
        todo!()
    }

    async fn get_fields_by_collection(
        &self,
        _collection_id: i32,
    ) -> Result<Vec<crate::domain::models::field::Field>, sqlx::Error> {
        match self {
            DataAccess::Postgres(pg) => pg.get_fields_by_collection(_collection_id).await,
            DataAccess::MYSQL(_) => todo!(),
        }
    }
}
