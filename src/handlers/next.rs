use frankenstein::{AsyncApi, CallbackQuery, EditMessageTextParams, ParseMode, AsyncTelegramApi, AnswerCallbackQueryParams};
use log::error;

use crate::{utils::{escape_markdown, create_inline_keyboard}, api::Api};

pub async fn handle_next(bot: &AsyncApi, query: &CallbackQuery) {
    let msg = query.message.as_ref().unwrap();
    let user = &query.from;
    let values: Vec<_> = query.data.as_ref().unwrap().split(",").collect();

    let params = EditMessageTextParams::builder()
        .message_id(msg.message_id)
        .chat_id(msg.chat.as_ref().id)
        .parse_mode(ParseMode::MarkdownV2)
        .disable_web_page_preview(true);
        
    let page = values[1].parse::<i32>().unwrap() + 1;
    let api = Api::new(&user.id.to_string());
    
    if let Ok(files) = api.get_all(page).await {
        if files.len() < 1 {
            let params = AnswerCallbackQueryParams::builder()
                .callback_query_id(&query.id)
                .cache_time(0u32)
                .text("This is the last page")
                .build();
            
            if let Err(err) = bot.answer_callback_query(&params).await {
                error!("Failed to warn last page: {err}");
            }

            return;
        }

        let mut text = String::from("💾 Your files");

        for file in files {
            text += &format!("\n\n*Name:* {}\n*View:* _/id{}_", escape_markdown(&file.name), file.id);
        }

        text += &format!("\n\nPage: *{page}*", );

        let keyboard = create_inline_keyboard(vec![
            ("Back", &format!("back,{page}")),
            ("Next", &format!("next,{page}"))
        ]);

        let params = params
            .text(text)
            .reply_markup(keyboard)
            .build();
        
        if let Err(err) = bot.edit_message_text(&params).await {
            error!("Failed to edit files message: {err}");
        }
    }
}