use std::{fs, path::Path};

use mlua::prelude::*;

use crate::api::API;

pub fn load_scripts(lua: &Lua, dir: &str, api: &API) -> LuaResult<()> {
    let path = Path::new(dir);

    if path.is_dir() {
        for entry in
            fs::read_dir(path).map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?
        {
            let entry = entry.map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("luau") {
                println!("Loading script: {:?}", path);

                let content = fs::read_to_string(&path)
                    .map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?;

                api.register_script(lua, content.as_str())?;
            }
        }
    }

    Ok(())
}
