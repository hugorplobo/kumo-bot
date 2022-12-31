use std::env;

use bb8::Pool;
use bb8_postgres::{tokio_postgres::{Config, NoTls}, PostgresConnectionManager};
use dotenvy::dotenv;
use log::info;
use frankenstein::{AsyncTelegramApi, AsyncApi, GetUpdatesParams};

pub mod model;
pub mod handlers;

use handlers::command_handler::parse_update;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
    info!("Starting api");
    let token = env::var("TOKEN").expect("The telegram token is necessary");
    let bot = AsyncApi::new(&token);

    info!("Starting database connection pool");
    let mut db_config = Config::new();

    db_config
        .user("postgres")
        .password("postgres")
        .dbname("kumo")
        .host("localhost");

    let manager = bb8_postgres::PostgresConnectionManager::new(
        db_config,
        NoTls
    );

    let pool = bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap();

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        if let Ok(response) = bot.get_updates(&update_params).await {
            for update in response.result {
                let update_id = update.update_id;
                let bot_clone = bot.clone();
                let pool_clone = pool.clone();

                tokio::spawn(async move {
                    parse_update(bot_clone, update, pool_clone).await;
                });

                update_params = update_params_builder
                    .clone()
                    .offset(update_id + 1)
                    .build();
            }
        }
    }
}
