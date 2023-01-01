#[derive(Clone, Debug)]
pub struct File {
    pub id: i32,
    pub telegram_id: String,
    pub name: String,
}

impl File {
    pub fn new(telegram_id: &str, name: &str) -> Self {
        File { id: 0, telegram_id: telegram_id.to_string(), name: name.to_string() }
    }
}