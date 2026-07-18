use mlua::prelude::*;

pub struct GameModule {
    scripts: Vec<LuaTable>,
}

impl GameModule {
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
        }
    }

    pub fn register_script(&mut self, lua: &Lua, script: &str, name: &str) -> LuaResult<()> {
        let table: LuaTable = lua.load(script).set_name(name).eval()?;
        self.scripts.push(table);
        Ok(())
    }

    pub fn update(&self, dt: f32) -> LuaResult<()> {
        for script in &self.scripts {
            if let Ok(func) = script.get::<LuaFunction>("on_update") {
                let result = func.call::<()>(dt);

                match result {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Script Error: {}", err)
                    }
                }
            }
        }

        Ok(())
    }
}
