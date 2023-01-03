use frankenstein::{AsyncApi, Update, UpdateContent};
use log::info;

use crate::handlers::{help::handle_help, add::handle_add, list::handle_list, query_handler::parse_callback_query, id::handle_id};

use super::{query_handler::parse_inline_query, web::handle_web};

pub async fn parse_update(bot: AsyncApi, update: Update) {
    match update.content {
        UpdateContent::Message(msg) => {
            if let Some(ref text) = msg.text {
                if text.starts_with("/help") || text.starts_with("/start") {
                    info!("Help message received");
                    handle_help(&bot, &msg).await;
                } else if text.starts_with("/list") {
                    info!("List message received");
                    handle_list(&bot, &msg).await;
                } else if text.starts_with("/id") {
                    info!("Id message received");
                    handle_id(&bot, &msg).await;
                } else if text.starts_with("/web") {
                    handle_web(&bot, &msg).await;
                }
            } else if let Some(_) = msg.document {
                info!("Document received");
                handle_add(&bot, &msg).await;
            }
        },
        UpdateContent::CallbackQuery(query) => {
            parse_callback_query(&bot, &query).await;
        },
        UpdateContent::InlineQuery(query) => {
            info!("Inline query received");
            parse_inline_query(&bot, &query).await;
        },
        _ => {
            info!("Unknown update received");
        }
    }
}