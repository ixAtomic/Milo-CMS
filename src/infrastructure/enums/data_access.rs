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
    fn get_collection_by_id(&self, _id: i32) -> Collection {
        match self {
            DataAccess::Postgres(pg_access) => pg_access.get_collection_by_id(_id),
            DataAccess::MYSQL(my_access) => todo!(),
        }
    }
}
