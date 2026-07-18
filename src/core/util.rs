use mlua::prelude::*;
use raylib::ffi::{Color, KeyboardKey};

pub fn to_color(color: &str) -> Result<Color, LuaError> {
    match color {
        "WHITE" => Ok(Color::WHITE),
        "BLACK" => Ok(Color::BLACK),
        "RED" => Ok(Color::RED),
        "BLUE" => Ok(Color::BLUE),
        "GREEN" => Ok(Color::GREEN),

        _ => Err(LuaError::RuntimeError(String::from(
            "Could not find color!",
        ))),
    }
}

pub fn to_keyboard_key(key: &str) -> Result<KeyboardKey, LuaError> {
    match key {
        "A" => Ok(KeyboardKey::KEY_A),

        _ => Err(LuaError::RuntimeError(String::from(
            "Could not find keyboard key!",
        ))),
    }
}
