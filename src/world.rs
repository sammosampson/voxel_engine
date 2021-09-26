pub struct WorldEntityId {
    pub name: String
}

pub struct WorldEntitySelected;

pub struct Visible(pub bool);

impl From<&str> for WorldEntityId {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}