use frankenstein::{AsyncApi, Message, SendDocumentParams, AsyncTelegramApi, ReplyMarkup};
use log::error;

use crate::{utils::create_inline_keyboard, api::Api};

pub async fn handle_id(bot: &AsyncApi, msg: &Message) {
    let msg_values: Vec<_> = msg.text.as_ref().unwrap().split("/id").collect();
    let file_id = msg_values[1].parse::<i32>().unwrap();

    let api = Api::new(&msg.from.as_ref().unwrap().id.to_string());
    
    if let Ok(file) = api.get(file_id).await {
        let keyboard = create_inline_keyboard(vec![
            ("Delete", &format!("delete,{},{}", file_id, msg.chat.as_ref().id))
        ]);

        let params = SendDocumentParams::builder()
            .chat_id(msg.chat.id)
            .document(file.telegram_id)
            .reply_markup(ReplyMarkup::InlineKeyboardMarkup(keyboard))
            .reply_to_message_id(msg.message_id)
            .build();
    
        if let Err(err) = bot.send_document(&params).await {
            error!("Failed to send file: {err}");
        }
    }
}