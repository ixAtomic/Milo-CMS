use crate::domain::models::field::Field;

pub trait FieldTrait {
    async fn get_field(&self, _id: i32) -> Result<Field, sqlx::Error>;
    async fn get_fields_by_collection(
        &self,
        _collection_id: i32,
    ) -> Result<Vec<Field>, sqlx::Error>;
}
