use rocket::State;

use crate::infrastructure::enums::data_access::DataAccess;

use super::traits::queryable::Queryable;

pub fn get_collection(access: &State<DataAccess>) -> &'static str {
    return "test";
}
