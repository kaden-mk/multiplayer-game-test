use mlua::prelude::*;
use paste::paste;
use raylib::ffi::{Color, KeyboardKey};

use crate::{colors, keys};

colors!(
    LIGHTGRAY, GRAY, DARKGRAY, YELLOW, GOLD, ORANGE, PINK, RED, MAROON, GREEN, LIME, DARKGREEN,
    SKYBLUE, BLUE, DARKBLUE, PURPLE, VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, BLACK,
    BLANK, MAGENTA, RAYWHITE
);
keys!(
    // letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // numbers
    ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE,
    // arrows
    UP, DOWN, LEFT, RIGHT,
    // editing / whitespace
    SPACE, ENTER, ESCAPE, TAB, BACKSPACE, DELETE, INSERT,
    // navigation
    HOME, END, PAGE_UP, PAGE_DOWN,
    // modifiers
    LEFT_SHIFT, LEFT_CONTROL, LEFT_ALT, RIGHT_SHIFT, RIGHT_CONTROL, RIGHT_ALT,
    // misc
    GRAVE, CAPS_LOCK,
    // function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12
);

pub fn to_color(name: &str) -> Result<Color, LuaError> {
    COLORS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
        .ok_or_else(|| LuaError::RuntimeError(format!("unknown color '{name}'")))
}

pub fn to_keyboard_key(name: &str) -> Result<KeyboardKey, LuaError> {
    KEYS.iter()
        .find(|(n, _)| *n == name)
        .map(|(_, k)| *k)
        .ok_or_else(|| LuaError::RuntimeError(format!("unknown key '{name}'")))
}
