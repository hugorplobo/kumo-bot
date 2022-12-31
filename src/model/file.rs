#[derive(Clone, Debug)]
pub struct File {
    pub id: String,
    pub name: String,
}

impl File {
    pub fn new(id: &str, name: &str) -> Self {
        File { id: id.to_string(), name: name.to_string() }
    }
}