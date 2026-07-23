use mlua::prelude::*;

pub struct GameModule;

impl GameModule {
    pub fn update(dt: f32, lua: &Lua) -> LuaResult<()> {
        let engine: LuaTable = lua.globals().get("engine")?;

        if let Ok(func) = engine.get::<LuaFunction>("on_update") {
            let result = func.call::<()>(dt);

            match result {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Script Error: {}", err)
                }
            }
        }

        Ok(())
    }

    pub fn draw(lua: &Lua) -> LuaResult<()> {
        let engine: LuaTable = lua.globals().get("engine")?;

        if let Ok(func) = engine.get::<LuaFunction>("on_draw") {
            let result = func.call::<()>(());

            match result {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Script Error: {}", err)
                }
            }
        }

        Ok(())
    }
}
