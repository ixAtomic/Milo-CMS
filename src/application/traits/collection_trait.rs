use crate::domain::models::collection::Collection;
pub trait CollectionTrait {
    async fn get_collection_by_id(&self, _id: i32) -> Result<Collection, sqlx::Error>;
}
