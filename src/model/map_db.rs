use std::collections::HashMap;

use super::{file::File, db::Database};

pub struct MapDatabase {
    map: HashMap<String, HashMap<String, File>>
}

impl Database for MapDatabase {
    fn add(&mut self, user_id: &str, file: &File) -> Result<(), ()> {
        if !self.map.contains_key(user_id) {
            self.map.insert(user_id.to_string(), HashMap::new());
        }

        let user_map = self.map.get_mut(user_id).unwrap();
        user_map.insert(file.id.clone(), file.clone());

        Ok(())
    }

    fn remove(&mut self, user_id: &str, file_id: &str) -> Result<(), ()> {
        self.check_user(user_id)?;

        let user_map = self.map.get_mut(user_id).unwrap();
        user_map.remove(file_id);

        Ok(())
    }

    fn get(&self, user_id: &str, file_id: &str) -> Result<File, ()> {
        self.check_user(user_id)?;

        let user_map = self.map.get(user_id).unwrap();
        if let Some(file) = user_map.get(file_id) {
            return Ok(file.clone());
        }

        Err(())
    }

    fn get_all(&self, user_id: &str) -> Result<HashMap<String, File>, ()> {
        self.check_user(user_id)?;

        let user_map = self.map.get(user_id).unwrap();
        
        Ok(user_map.clone())
    }
}

impl MapDatabase {
    fn check_user(&self, user_id: &str) -> Result<(), ()> {
        if !self.map.contains_key(user_id) {
            return Err(());
        }

        Ok(())
    }
}