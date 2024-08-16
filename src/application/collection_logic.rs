use rocket::State;

use crate::{
    domain::models::collection::Collection, infrastructure::enums::data_access::DataAccess,
};

use super::traits::collection_trait::CollectionTrait;

pub fn get_collection(access: &State<DataAccess>) -> Collection {
    return access.inner().get_collection_by_id(1);
}
