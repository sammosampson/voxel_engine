pub struct WorldEntityId {
    pub name: String
}

#[derive(Default)]
pub struct WorldEntitySelected {
}

impl From<&str> for WorldEntityId {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}
