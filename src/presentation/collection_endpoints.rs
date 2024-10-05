use rocket::{http::Status, serde::json::Json, State};

use crate::{
    application::collection_logic,
    domain::models::collection::{Collection, RCollection},
    infrastructure::enums::data_access::{self, DataAccess},
};

#[get("/")]
pub async fn get_collections(access: &State<DataAccess>) -> Result<Json<Vec<Collection>>, Status> {
    let result = collection_logic::get_collections(access).await;

    match result {
        Ok(collection) => Ok(Json(collection)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<id>")]
pub async fn get_collection_by_id(
    data_access: &State<DataAccess>,
    id: i32,
) -> Result<Json<Collection>, Status> {
    let result = collection_logic::get_collection_by_id(data_access, id).await;

    match result {
        Ok(collection) => Ok(Json(collection)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/create", data = "<rcollection>")]
pub async fn create_collection(
    data_access: &State<DataAccess>,
    rcollection: Json<RCollection>,
) -> Result<Json<i32>, Status> {
    let result = collection_logic::create_collection(data_access, rcollection.into_inner()).await;

    match result {
        Ok(id) => Ok(Json(id)),
        Err(_) => Err(Status::InternalServerError),
    }
}
