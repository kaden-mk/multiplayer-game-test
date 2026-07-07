use mlua::prelude::*;

use crate::{bind_func, core::util::to_keyboard_key_i32};

pub struct InputModule;

impl InputModule {
    pub fn new() -> Self {
        Self
    }

    fn is_key_down(key: String) -> bool {
        let key_code = to_keyboard_key_i32(&key.to_uppercase().as_str());
        unsafe { raylib::ffi::IsKeyDown(key_code) }
    }
}

impl InputModule {
    pub fn register(&self, lua: &Lua) -> LuaResult<()> {
        let input_table = lua.create_table()?;

        bind_func!(lua, input_table, "is_key_down", static Self, is_key_down, (key: String));

        lua.globals().set("input", input_table)?;
        Ok(())
    }
}
