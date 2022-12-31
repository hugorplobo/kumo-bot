use frankenstein::{AsyncApi, SendMessageParams, Message, AsyncTelegramApi};
use log::error;

pub async fn handle_help(bot: &AsyncApi, msg: &Message) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(msg.chat.id)
        .text("Lorem ipsum")
        .build();
    
    if let Err(err) = bot.send_message(&send_message_params).await {
        error!("Failed to respond help command: {err}");
    }
}