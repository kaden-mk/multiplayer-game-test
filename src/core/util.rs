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

pub fn to_keyboard_key_i32(key: &str) -> i32 {
    match key {
        "A" => KeyboardKey::KEY_A as i32,

        _ => 0,
    }
}
