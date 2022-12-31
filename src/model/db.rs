use std::collections::HashMap;

use log::error;

use crate::DbPool;

use super::file::File;

pub struct Database {
    pool: DbPool
}

impl Database {
    pub fn new(pool: DbPool) -> Self {
        Database { pool }
    }

    pub async fn add(&self, user_id: &str, file: &File) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();
        
        if let Err(err) = conn.execute(
            "insert into file values ($1::TEXT, $2::TEXT)",
            &[&file.id, &file.name]
        ).await {
            error!("Failed to insert file: {err}");
            return Err(());
        }

        if let Err(err) = conn.execute(
            "insert into file_user values ($1::TEXT, $2::TEXT)",
            &[&user_id, &file.id]
        ).await {
            error!("Failed to bind file to user: {err}");
            return Err(());
        }

        Ok(())
    }

    pub async fn remove(&self, user_id: &str, file_id: &str) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();

        if let Err(err) = conn.execute(
            "delete from file_user where id_user = $1::TEXT and id_file = $2::TEXT",
            &[&user_id, &file_id]
        ).await {
            error!("Failed to remove file from user: {err}");
            return Err(());
        }

        Ok(())
    }

    pub async fn get(&self, file_id: &str) -> Result<File, ()> {
        let conn = self.pool.get().await.unwrap();

        match conn.query(
            "select from file where id = $1::TEXT",
            &[&file_id]
        ).await {
            Ok(rows) => {
                return Ok(File::new(rows[0].get("id"), rows[0].get("name")));
            },
            Err(err) => {
                error!("Failed to get file: {err}");
                return Err(());
            }
        }
    }

    pub fn get_all(&self, _user_id: &str) -> Result<HashMap<String, File>, ()> {
        todo!()
    }
}