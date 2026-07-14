use mlua::prelude::*;
use raylib::prelude::*;
use std::{cell::RefCell, fs, path::Path, rc::Rc};

use crate::api::API;

pub struct Application {
    api: Rc<API>,
    rl: Rc<RefCell<RaylibHandle>>,
    rl_thread: Rc<RaylibThread>,
    lua: Rc<Lua>,
}

impl Application {
    pub fn new() -> LuaResult<Self> {
        let (rl, rl_thread) = raylib::init()
            .size(1920, 1080)
            .title("Multiplayer Shooter")
            .build();

        rl.toggle_borderless_windowed();

        let rl = Rc::new(RefCell::new(rl));
        let rl_thread = Rc::new(rl_thread);

        let lua = Rc::new(Lua::new());
        let api = Rc::new(API::new(rl.clone(), rl_thread.clone()));
        api.init(&lua)?;

        Ok(Self {
            api,
            rl,
            rl_thread,
            lua,
        })
    }

    pub fn load_scripts(&self, dir: &str) -> LuaResult<()> {
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

                    self.api.register_script(&self.lua, content.as_str())?;
                }
            }
        }

        Ok(())
    }

    pub fn run(&self) -> LuaResult<()> {
        loop {
            let should_close = { self.rl.borrow().window_should_close() };

            if should_close {
                break;
            }

            let dt = self.rl.borrow().get_frame_time();
            self.api.update(dt)?;

            let mut rl = self.rl.borrow_mut();
            let mut d = rl.begin_drawing(&self.rl_thread);
            self.api.draw(&mut d);
        }

        Ok(())
    }
}
