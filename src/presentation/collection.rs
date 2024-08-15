use rocket::State;

use crate::{application::collection_logic, infrastructure::enums::data_access::DataAccess};

#[get("/")]
pub fn index(data_access: &State<DataAccess>) -> &'static str {
    // let test = match data_access.inner() {
    //     DataAccess::Postgres(pg_access) => 1,
    //     DataAccess::MYSQL(my_access) => 2,
    // };

    return collection_logic::get_collection(data_access);
}
