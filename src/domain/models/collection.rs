use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub singleton: bool,
}
