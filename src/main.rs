use std::env;

use bb8::Pool;
use bb8_postgres::{tokio_postgres::{Config, NoTls}, PostgresConnectionManager};
use dotenvy::dotenv;
use log::info;
use frankenstein::{AsyncTelegramApi, AsyncApi, GetUpdatesParams};

pub mod model;
pub mod handlers;
pub mod utils;

use handlers::command_handler::parse_update;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
    info!("Starting api");
    let token = env::var("TELEGRAM_TOKEN").expect("The telegram token is necessary");
    let bot = AsyncApi::new(&token);

    info!("Starting database connection pool");
    let mut db_config = Config::new();
    let pg_user = env::var("PG_USER").expect("The postgres username is necessary");
    let pg_password = env::var("PG_PASSWORD").expect("The postgres password is necessary");
    let pg_db_name = env::var("PG_DB_NAME").expect("The postgres db name is necessary");
    let pg_hostname = env::var("PG_HOSTNAME").expect("The postgres hostname is necessary");

    db_config
        .user(&pg_user)
        .password(&pg_password)
        .dbname(&pg_db_name)
        .host(&pg_hostname);

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

    info!("Running");

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
