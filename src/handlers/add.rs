use frankenstein::{AsyncApi, Message, SendMessageParams, AsyncTelegramApi};
use log::error;

use crate::api::{File, Api};

pub async fn handle_add(bot: &AsyncApi, msg: &Message) {
    if let Some(ref user) = msg.from {
        let doc = msg.document.as_ref().unwrap();
        let file = File {
            id: 0,
            telegram_id: doc.file_id.clone(),
            name: doc.file_name.as_ref().unwrap_or(&"unknown".to_string()).clone()
        };

        let api = Api::new(&user.id.to_string());

        let send_message_builder = SendMessageParams::builder()
            .chat_id(msg.chat.id)
            .reply_to_message_id(msg.message_id);

        if let Err(_) = api.add(&file).await {
            let params = send_message_builder
                .text("Could not save the file :(")
                .build();
            
            if let Err(err) = bot.send_message(&params).await {
                error!("Failed to send error message while saving file: {err}");
            }
        } else {
            let params = send_message_builder
                .text("The file was saved :D")
                .build();
            
            if let Err(err) = bot.send_message(&params).await {
                error!("Failed to send file added message: {err}");
            }
        }
    }
}