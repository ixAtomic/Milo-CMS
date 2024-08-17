use crate::domain::models::collection::Collection;
pub trait CollectionTrait {
    async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error>;
    async fn get_collection_by_id(&self, _id: i32) -> Result<Collection, sqlx::Error>;
}
