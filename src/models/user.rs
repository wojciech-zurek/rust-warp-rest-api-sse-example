#[derive(Clone)]
pub struct User {
    pub name: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        User {
            name,
            age,
        }
    }
}