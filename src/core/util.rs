use raylib::ffi::{Color, KeyboardKey};

pub fn to_color(color: &str) -> Color {
    match color {
        "WHITE" => Color::WHITE,
        "BLACK" => Color::BLACK,
        "RED" => Color::RED,
        "BLUE" => Color::BLUE,
        "GREEN" => Color::GREEN,

        _ => Color::MAGENTA,
    }
}

pub fn to_keyboard_key(key: &str) -> KeyboardKey {
    match key {
        "A" => KeyboardKey::KEY_A,
        _ => KeyboardKey::KEY_ESCAPE,
    }
}
