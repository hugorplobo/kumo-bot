use frankenstein::{AsyncApi, CallbackQuery, AsyncTelegramApi, AnswerCallbackQueryParams, DeleteMessageParams};
use log::error;

use crate::model::db::Database;

pub async fn handle_delete(bot: &AsyncApi, query: &CallbackQuery, db: &Database) {
    let values: Vec<_> = query.data.as_ref().unwrap().split(",").collect();
    let file_id = values[1].parse::<i32>().unwrap();
    let chat_id = values[2].parse::<i64>().unwrap();
    let user_id = query.from.id.to_string();

    let params = AnswerCallbackQueryParams::builder()
        .callback_query_id(query.id.clone())
        .show_alert(true);

    if let Err(_) = db.remove(&user_id, file_id).await {
        let params = params
            .text("Failed do delete the file :(")
            .build();
        
        if let Err(err) = bot.answer_callback_query(&params).await {
            error!("Failed to send error message while deleting file: {err}");
        }

        return;
    }

    let params = params
        .text("File deleted :D")
        .build();

    if let Err(err) = bot.answer_callback_query(&params).await {
        error!("Failed to send delete file message: {err}");
    }

    if let Some(ref msg) = query.message {
        let params = DeleteMessageParams::builder()
            .chat_id(chat_id)
            .message_id(msg.message_id)
            .build();
        
        if let Err(err) = bot.delete_message(&params).await {
            error!("Failed to delete file message: {err}");
        }
    }
}