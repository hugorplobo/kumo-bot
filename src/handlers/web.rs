use frankenstein::{AsyncApi, Message, InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo, SendMessageParams, ReplyMarkup, AsyncTelegramApi, ParseMode};
use std::env;

pub async fn handle_web(bot: &AsyncApi, msg: &Message) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    let mut row: Vec<InlineKeyboardButton> = Vec::new();

    row.push(
        InlineKeyboardButton::builder()
            .text("Open web app 🌐")
            .web_app(WebAppInfo::builder().url(env::var("WEB_APP_URL").expect("The web app url is necessary")).build())
            .build()
    );
    keyboard.push(row);

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();
    
    let params = SendMessageParams::builder()
        .text("Click the button bellow to open the web app\\!\n\n_This feature is still under development and can be buggy on desktop clients\\!_")
        .chat_id(msg.chat.id)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
        .build();
    
    bot.send_message(&params).await.unwrap();
}