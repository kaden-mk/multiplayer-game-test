use mlua::prelude::*;
use raylib::prelude::*;

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

    pub fn update(&self, dt: f32) -> LuaResult<()> {
        //d.clear_background(Color::BLACK);
        for script in &self.scripts {
            if let Ok(func) = script.get::<LuaFunction>("on_update") {
                func.call::<()>(dt)?;
            }
        }

        Ok(())
    }
}
