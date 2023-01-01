use frankenstein::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn escape_filename(name: &str) -> String {
    let res = String::from(name);
    res.replace(".", "\\.")
        .replace("_", "\\_")
}

pub fn create_inline_keyboard(btns: Vec<(&str, &str)>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    let mut row: Vec<InlineKeyboardButton> = Vec::new();

    for btn in btns {
        let button = InlineKeyboardButton::builder()
            .text(btn.0)
            .callback_data(btn.1)
            .build();
        
        row.push(button);
    }

    keyboard.push(row);

    let inline_keyboard = InlineKeyboardMarkup::builder()
        .inline_keyboard(keyboard)
        .build();
    
    inline_keyboard
}