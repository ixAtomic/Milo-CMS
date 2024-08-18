use rocket::{http::Status, serde::json::Json, State};

use crate::{
    application::field_logic,
    domain::models::field::{self, Field},
    infrastructure::enums::data_access::DataAccess,
};

#[get("/<id>")]
pub async fn get_fields(access: &State<DataAccess>, id: i32) -> Result<Json<Field>, Status> {
    let result = field_logic::get_field(access, id).await;

    match result {
        Ok(field) => Ok(Json(field)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>/fields")]
pub async fn get_fields_by_collection(
    access: &State<DataAccess>,
    id: i32,
) -> Result<Json<Vec<Field>>, Status> {
    let result = field_logic::get_fields_by_collection(access, id).await;

    match result {
        Ok(fields) => Ok(Json(fields)),
        Err(_) => Err(Status::InternalServerError),
    }
}
