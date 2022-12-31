use frankenstein::{AsyncApi, Message, SendMessageParams, AsyncTelegramApi, ParseMode};
use log::error;

use crate::{model::db::Database, utils::escape_filename};

pub async fn handle_list(bot: &AsyncApi, msg: &Message, db: &Database) {
    if let Some(ref user) = msg.from {
        let send_message_builder = SendMessageParams::builder()
            .chat_id(msg.chat.id);

        match db.get_all(&user.id.to_string()).await {
            Ok(files) => {
                let mut text = format!("💾 You have {} items", files.len());

                for file in files {
                    text += &format!("\n\n*Name:* {}\n*View:* _in soon_", escape_filename(&file.name));
                }

                let params = send_message_builder
                    .text(text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .disable_web_page_preview(true)
                    .build();
                
                if let Err(err) = bot.send_message(&params).await {
                    error!("Failed to send error message while retrieving files: {err}");
                }
            },
            Err(_) => {
                let params = send_message_builder
                    .text("Could not retrieve your files :(")
                    .build();
                
                if let Err(err) = bot.send_message(&params).await {
                    error!("Failed to send error message while retrieving files: {err}");
                }
            }
        }
    }
}