use frankenstein::{AsyncApi, Message, SendMessageParams, AsyncTelegramApi, ParseMode, ReplyMarkup};
use log::error;

use crate::{utils::{escape_markdown, create_inline_keyboard}, api::Api};

pub async fn handle_list(bot: &AsyncApi, msg: &Message) {
    if let Some(ref user) = msg.from {
        let send_message_builder = SendMessageParams::builder()
            .chat_id(msg.chat.id);
        
        let api = Api::new(&user.id.to_string());

        match api.get_all(1).await {
            Ok(files) => {
                let mut text = String::from("💾 Your files");

                for file in files {
                    text += &format!("\n\n*Name:* {}\n*View:* _/id{}_", escape_markdown(&file.name), file.id);
                }

                text += "\n\nPage: *1*";

                let keyboard = create_inline_keyboard(vec![
                    ("Back", "back,1"),
                    ("Next", "next,1")
                ]);

                let params = send_message_builder
                    .text(text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .disable_web_page_preview(true)
                    .reply_markup(ReplyMarkup::InlineKeyboardMarkup(keyboard))
                    .build();
                
                if let Err(err) = bot.send_message(&params).await {
                    error!("Failed to send files message: {err}");
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