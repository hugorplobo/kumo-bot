use frankenstein::{AsyncApi, Update, UpdateContent};
use log::info;

use crate::{handlers::{help::handle_help, add::handle_add, list::handle_list, query_handler::parse_callback_query, id::handle_id}, DbPool, model::db::Database};

use super::query_handler::parse_inline_query;

pub async fn parse_update(bot: AsyncApi, update: Update, pool: DbPool) {
    let db = Database::new(pool);

    match update.content {
        UpdateContent::Message(msg) => {
            if let Some(ref text) = msg.text {
                if text.starts_with("/help") || text.starts_with("/start") {
                    info!("Help message received");
                    handle_help(&bot, &msg).await;
                } else if text.starts_with("/list") {
                    info!("List message received");
                    handle_list(&bot, &msg, &db).await;
                } else if text.starts_with("/id") {
                    info!("Id message received");
                    handle_id(&bot, &msg, &db).await;
                }
            } else if let Some(_) = msg.document {
                info!("Document received");
                handle_add(&bot, &msg, &db).await;
            }
        },
        UpdateContent::CallbackQuery(query) => {
            parse_callback_query(&bot, &query, &db).await;
        },
        UpdateContent::InlineQuery(query) => {
            info!("Inline query received");
            parse_inline_query(&bot, &query, &db).await;
        },
        _ => {
            info!("Unknown update received");
        }
    }
}