use frankenstein::{AsyncApi, SendMessageParams, Message, AsyncTelegramApi, ParseMode, InlineKeyboardButton, ReplyMarkup, InlineKeyboardMarkup};
use log::error;

pub async fn handle_help(bot: &AsyncApi, msg: &Message) {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    let mut row: Vec<InlineKeyboardButton> = Vec::new();

    row.push(
        InlineKeyboardButton::builder()
            .text("Try out!")
            .switch_inline_query_current_chat("")
            .build()
    );
    keyboard.push(row);

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();

    let send_message_params = SendMessageParams::builder()
        .chat_id(msg.chat.id)
        .text(
"📤 *Add files*:
Send me any file to save

📂 *Show all files*:
Use _/list_ to view your collection

⚙️ *View and delete a single file*:
The _/list_ command will give you a command like \"/id123\" for each file, click in this command to access the file and to get the option to delete the file if you want to

🌎 *Access in other chats*:
Type @kumoo\\_bot in any chat to access your files

_You can report any problem to @hugorplobo\\!_"
        )
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(inline_keyboard))
        .build();
    
    if let Err(err) = bot.send_message(&send_message_params).await {
        error!("Failed to respond help command: {err}");
    }
}