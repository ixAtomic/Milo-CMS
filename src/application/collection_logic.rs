use rocket::State;

use crate::{
    domain::models::collection::{Collection, RCollection},
    infrastructure::enums::data_access::DataAccess,
};

use super::traits::collection_trait::CollectionTrait;

pub async fn get_collections(access: &State<DataAccess>) -> Result<Vec<Collection>, sqlx::Error> {
    return access.inner().get_collections().await;
}

pub async fn get_collection_by_id(
    access: &State<DataAccess>,
    _id: i32,
) -> Result<Collection, sqlx::Error> {
    return access.inner().get_collection_by_id(_id).await;
}

pub async fn create_collection(
    access: &State<DataAccess>,
    collection: RCollection,
) -> Result<i32, sqlx::Error> {
    return access.inner().create_collection(collection).await;
}
