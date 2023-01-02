use std::env;

use jsonwebtoken::{Header, EncodingKey, encode};
use log::error;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub telegram_id: String,
    pub name: String,
}

pub struct Api {
    base_url: String,
    client: Client,
    user_id: String,
    jwt: String
}

#[derive(Serialize)]
pub struct Claims {
    sub: String,
    exp: usize
}

impl Api {
    pub fn new(user_id: &str) -> Self {
        let base_url = env::var("API_URL").expect("The api url is necessary");
        let client = Client::new();
        let jwt = Api::generate_jwt(user_id).unwrap();

        Api { user_id: user_id.to_string(), base_url, client, jwt }
    }

    pub async fn add(&self, file: &File) -> Result<(), ()> {
        let url = format!(
            "{}/add?user_id={}&id={}&telegram_id={}&name={}",
            self.base_url,
            self.user_id,
            file.id,
            file.telegram_id,
            file.name
        );
    
        if let Err(err) = self.client.post(url)
            .bearer_auth(&self.jwt)
            .send()
            .await
        {
            error!("Failed to add file: {err}");
            return Err(());
        }
    
        Ok(())
    }

    pub async fn get(&self, file_id: i32) -> Result<File, ()> {
        let url = format!(
            "{}/get?user_id={}&file_id={}",
            self.base_url,
            self.user_id,
            file_id
        );

        match self.client.get(url)
            .bearer_auth(&self.jwt)
            .send()
            .await
        {
            Ok(res) => {
                let file: File = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                return Ok(file);
            },
            Err(err) => {
                error!("Failed to get file: {err}");
                return Err(());
            }
        }
    }

    pub async fn remove(&self, file_id: i32) -> Result<(), ()> {
        let url = format!(
            "{}/remove?user_id={}&file_id={}",
            self.base_url,
            self.user_id,
            file_id
        );

        if let Err(err) = self.client.post(url)
            .bearer_auth(&self.jwt)
            .send()
            .await
        {
            error!("Failed to remove file: {err}");
            return Err(());
        }

        Ok(())
    }

    pub async fn get_all(&self, page: i32) -> Result<Vec<File>, ()> {
        let url = format!(
            "{}/get_all?user_id={}&page={}",
            self.base_url,
            self.user_id,
            page
        );

        match self.client.get(url)
            .bearer_auth(&self.jwt)
            .send()
            .await
        {
            Ok(res) => {
                let files: Vec<File> = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                return Ok(files);
            },
            Err(err) => {
                error!("Failed to get all files: {err}");
                return Err(());
            }
        }
    }

    pub async fn search(&self, search: &str) -> Result<Vec<File>, ()> {
        let url = format!(
            "{}/search?user_id={}&search={}",
            self.base_url,
            self.user_id,
            search
        );

        match self.client.get(url)
            .bearer_auth(&self.jwt)
            .send()
            .await
        {
            Ok(res) => {
                let files: Vec<File> = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                return Ok(files);
            },
            Err(err) => {
                error!("Failed to search files: {err}");
                return Err(());
            }
        }
    }

    fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let claims = Claims {
            sub: user_id.to_string(),
            exp: 10000000000
        };

        let token = env::var("TELEGRAM_TOKEN").expect("The telegram token is necessary");
        match encode(&header, &claims, &EncodingKey::from_secret(token.as_bytes())) {
            Ok(jwt) => return Ok(jwt),
            Err(err) => {
                error!("Failed to generate jwt: {err}");
                return Err(err);
            }
        }
    }
}