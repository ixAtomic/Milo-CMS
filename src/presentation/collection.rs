use rocket::{serde::json::Json, State};

use crate::{
    application::collection_logic, domain::models::collection::Collection,
    infrastructure::enums::data_access::DataAccess,
};

#[get("/")]
pub fn index(data_access: &State<DataAccess>) -> Json<Collection> {
    // let test = match data_access.inner() {
    //     DataAccess::Postgres(pg_access) => 1,
    //     DataAccess::MYSQL(my_access) => 2,
    // };

    return Json(collection_logic::get_collection(data_access));
}
