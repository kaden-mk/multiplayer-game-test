use mlua::prelude::*;
use raylib::prelude::*;

pub struct AssetModule;

impl AssetModule {
    pub fn new() -> Self {
        Self
    }

    pub fn register(
        &self,
        lua: &Lua,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> LuaResult<()> {
        let assets_table = lua.create_table()?;

        lua.globals().set("assets", assets_table)?;

        Ok(())
    }
}

impl AssetModule {
    fn load_texture(
        &self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        filename: &str,
    ) -> LuaResult<()> {
        Ok(())
    }
}
