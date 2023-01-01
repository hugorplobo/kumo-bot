use frankenstein::{CallbackQuery, AsyncApi};
use log::{error, info};

use crate::{model::db::Database, handlers::{back::handle_back, delete::handle_delete}};

use super::next::handle_next;

pub async fn parse_query(bot: &AsyncApi, query: &CallbackQuery, db: &Database) {
    if let Some(_) = query.message {
        if let Some(ref data) = query.data {
            let values: Vec<_> = data.split(",").collect();

            match values[0] {
                "back" => {
                    info!("Back callback query received");
                    handle_back(bot, query, db).await;
                },
                "next" => {
                    info!("Next callback query received");
                    handle_next(bot, query, db).await;
                },
                "delete" => {
                    info!("Delete callback query received");
                    handle_delete(bot, query, db).await;
                },
                _ => {
                    error!("Unknown callback query type");
                }
            }
        }
    }
}