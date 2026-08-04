use crate::api::Rc;
use crate::api::RefCell;
use std::{fs, path::Path};

use mlua::prelude::*;

pub struct FsModule;

fn handle_dir(modules: Rc<RefCell<Vec<LuaString>>>, dir: &Path, lua: &Lua) -> LuaResult<()> {
    if dir.is_dir() {
        for entry in
            fs::read_dir(dir).map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?
        {
            let entry = entry.map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?;
            let path = entry.path();

            if path.is_dir() {
                handle_dir(modules.clone(), &path, lua)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("luau") {
                let path = path.to_string_lossy().to_string();
                let path = lua.create_string(path)?;

                modules.borrow_mut().push(path);
            }
        }
    }

    Ok(())
}

impl FsModule {
    pub fn init(lua: &Lua) -> LuaResult<()> {
        let func = lua.create_function(|lua: &Lua, dir: String| {
            let modules: Rc<RefCell<Vec<LuaString>>> = Rc::new(RefCell::new(vec![]));
            let path = Path::new(&dir);

            handle_dir(modules.clone(), &path, &lua)?;

            Ok(lua.create_sequence_from(modules.borrow().iter()))
        })?;
        lua.globals().set("discover", func)?;

        let func = lua.create_function(|lua: &Lua, path: String| {
            let path = Path::new(&path).to_path_buf();
            let content = fs::read_to_string(&path)
                .map_err(|e| LuaError::ExternalError(std::sync::Arc::new(e)))?;

            let name = format!(
                "@{}",
                path.with_extension("").to_string_lossy().replace('\\', "/")
            );

            lua.load(content.as_str()).set_name(name).exec()?;

            Ok(())
        })?;
        lua.globals().set("execute", func)?;

        Ok(())
    }
}
