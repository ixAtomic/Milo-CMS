pub struct Field {
    id: i32,
    name: String,
}

impl Field {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: String::from("postgres"),
        }
    }
}
