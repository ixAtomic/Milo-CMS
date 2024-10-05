use rocket::serde::{self, Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub singleton: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RCollection {
    pub name: String,
    pub singleton: bool,
}
