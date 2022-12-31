use dotenvy::dotenv;
use std::env;
use frankenstein::{AsyncTelegramApi, AsyncApi, GetUpdatesParams, UpdateContent};

pub mod model;

#[tokio::main] 
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("The telegram token is necessary");
    let bot = AsyncApi::new(&token);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        if let Ok(response) = bot.get_updates(&update_params).await {
            for update in response.result {
                if let UpdateContent::Message(message) = update.content {
                    println!("{:?}", message.text);
                }

                update_params = update_params_builder
                    .clone()
                    .offset(update.update_id + 1)
                    .build();
            }
        }
    }
}
