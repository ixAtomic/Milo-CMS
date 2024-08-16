use rocket::{http::Status, serde::json::Json, State};

use crate::{
    application::collection_logic, domain::models::collection::Collection,
    infrastructure::enums::data_access::DataAccess,
};

#[get("/GetCollectionById/<id>")]
pub async fn index(data_access: &State<DataAccess>, id: i32) -> Result<Json<Collection>, Status> {
    let result = collection_logic::get_collection_by_id(data_access, id).await;

    match result {
        Ok(collection) => Ok(Json(collection)),
        Err(_) => Err(Status::InternalServerError),
    }
}
