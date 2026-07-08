use mlua::prelude::*;
use raylib::{ffi::RaylibPalette, prelude::*};

pub struct GameModule {
    scripts: Vec<LuaTable>,
}

impl GameModule {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
        }
    }

    pub fn register_script(&mut self, lua: &Lua, script: &str) -> LuaResult<()> {
        let table: LuaTable = lua.load(script).eval()?;
        self.scripts.push(table);
        Ok(())
    }

    pub fn update(&self, d: &mut RaylibDrawHandle) -> LuaResult<()> {
        d.clear_background(Color::BLACK);
        for script in &self.scripts {
            if let Ok(func) = script.get::<LuaFunction>("on_update") {
                func.call::<()>(d.get_frame_time())?;
            }
        }
        Ok(())
    }
}
