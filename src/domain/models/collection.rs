pub struct Collection {
    id: i32,
    name: String,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: String::from("postgres"),
        }
    }
}
