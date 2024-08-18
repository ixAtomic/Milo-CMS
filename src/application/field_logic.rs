use rocket::State;

use crate::{domain::models::field::Field, infrastructure::enums::data_access::DataAccess};

use super::traits::field_trait::FieldTrait;

pub async fn get_fields_by_collection(
    access: &State<DataAccess>,
    collection_id: i32,
) -> Result<Vec<Field>, sqlx::Error> {
    return access.inner().get_fields_by_collection(collection_id).await;
}
