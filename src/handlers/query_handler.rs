use frankenstein::{CallbackQuery, AsyncApi, InlineQuery, AnswerInlineQueryParams, InlineQueryResultCachedDocument, InlineQueryResult, AsyncTelegramApi, InlineQueryResultArticle, InputTextMessageContent};
use log::{error, info};

use crate::{handlers::{back::handle_back, delete::handle_delete}, api::Api};

use super::next::handle_next;

pub async fn parse_callback_query(bot: &AsyncApi, query: &CallbackQuery) {
    if let Some(_) = query.message {
        if let Some(ref data) = query.data {
            let values: Vec<_> = data.split(",").collect();

            match values[0] {
                "back" => {
                    info!("Back callback query received");
                    handle_back(bot, query).await;
                },
                "next" => {
                    info!("Next callback query received");
                    handle_next(bot, query).await;
                },
                "delete" => {
                    info!("Delete callback query received");
                    handle_delete(bot, query).await;
                },
                _ => {
                    error!("Unknown callback query type");
                }
            }
        }
    }
}

pub async fn parse_inline_query(bot: &AsyncApi, query: &InlineQuery) {
    let user_id = query.from.id.to_string();
    let api = Api::new(&user_id);

    if let Ok(files) = api.search(&query.query).await {
        let mut results: Vec<InlineQueryResult> = files.iter().map(|file| {
            let result = InlineQueryResult::CachedDocument(
                InlineQueryResultCachedDocument::builder()
                    .id(file.id.to_string())
                    .title(file.name.clone())
                    .document_file_id(file.telegram_id.clone())
                    .build()
            );
            
            result

        }).collect();

        if results.len() < 1 {
            results.push(
                InlineQueryResult::Article(
                    InlineQueryResultArticle::builder()
                        .id("0")
                        .title(if query.query == "" { "No files saved yet" } else { "No file found for this search" })
                        .input_message_content(
                            frankenstein::InputMessageContent::Text(
                                InputTextMessageContent::builder()
                                    .message_text(if query.query == "" { "Send me a file to be saved :D" } else { "Make sure you didn't make a typo, and remember that you can see all your files with /list" })
                                    .build()
                            )
                        )
                        .build()
                )
            );
        }

        let params = AnswerInlineQueryParams::builder()
            .inline_query_id(query.id.clone())
            .is_personal(true)
            .cache_time(0u32)
            .results(results)
            .build();
        
        if let Err(err) = bot.answer_inline_query(&params).await {
            error!("Failed to show files to user: {err}");
        }
    }
}