use std::collections::HashMap;

use super::file::File;

pub trait Database {
    fn add(&mut self, user_id: &str, file: &File) -> Result<(), ()>;
    fn remove(&mut self, user_id: &str, file_id: &str) -> Result<(), ()>;
    fn get(&self, user_id: &str, file_id: &str) -> Result<File, ()>;
    fn get_all(&self, user_id: &str) -> Result<HashMap<String, File>, ()>;
}