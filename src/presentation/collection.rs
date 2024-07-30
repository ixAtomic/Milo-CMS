use crate::application::collection_logic;

#[get("/")]
pub fn index() -> &'static str {
    return collection_logic::get_collection();
}
