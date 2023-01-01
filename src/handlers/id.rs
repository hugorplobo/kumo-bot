use frankenstein::{AsyncApi, Message, SendDocumentParams, AsyncTelegramApi, ReplyMarkup};
use log::error;

use crate::{model::db::Database, utils::create_inline_keyboard};

pub async fn handle_id(bot: &AsyncApi, msg: &Message, db: &Database) {
    let msg_values: Vec<_> = msg.text.as_ref().unwrap().split("/id").collect();
    let file_id = msg_values[1].parse::<i32>().unwrap();
    
    if let Ok(file) = db.get(file_id, &msg.from.as_ref().unwrap().id.to_string()).await {
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