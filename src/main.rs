use std::env;

use dotenvy::dotenv;
use log::info;
use frankenstein::{AsyncTelegramApi, AsyncApi, GetUpdatesParams, AllowedUpdate};

pub mod api;
pub mod handlers;
pub mod utils;

use handlers::command_handler::parse_update;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
    info!("Starting api");
    let token = env::var("TELEGRAM_TOKEN").expect("The telegram token is necessary");
    let bot = AsyncApi::new(&token);

    let update_params_builder = GetUpdatesParams::builder()
        .allowed_updates(vec![
            AllowedUpdate::Message,
            AllowedUpdate::CallbackQuery,
            AllowedUpdate::InlineQuery,
        ]);

    let mut update_params = update_params_builder.clone().build();

    info!("Running");

    loop {
        if let Ok(response) = bot.get_updates(&update_params).await {
            for update in response.result {
                let update_id = update.update_id;
                let bot_clone = bot.clone();

                tokio::spawn(async move {
                    parse_update(bot_clone, update).await;
                });

                update_params = update_params_builder
                    .clone()
                    .offset(update_id + 1)
                    .build();
            }
        }
    }
}
