use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Collection {
    pub id: i32,
    pub name: String,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: String::from("postgres"),
        }
    }
}
