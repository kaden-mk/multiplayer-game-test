use mlua::prelude::*;
use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{
    api::API,
    core::types::{LuaColor, LuaNPatchInfo, LuaRect},
};

pub struct Application {
    api: Rc<API>,
    rl: Rc<RefCell<RaylibHandle>>,
    rl_thread: Rc<RaylibThread>,
    lua: Rc<Lua>,
}

impl Application {
    pub fn new() -> LuaResult<Self> {
        let (rl, rl_thread) = raylib::init()
            .size(1280, 720)
            .title("Multiplayer Shooter")
            .build();

        //rl.toggle_borderless_windowed();

        let rl = Rc::new(RefCell::new(rl));
        let rl_thread = Rc::new(rl_thread);

        let lua = Rc::new(Lua::new());

        LuaColor::create(&lua)?;
        LuaRect::create(&lua)?;
        LuaNPatchInfo::create(&lua)?;

        let api = Rc::new(API::new(rl.clone(), rl_thread.clone()));
        api.init(&lua)?;

        Ok(Self {
            api,
            rl,
            rl_thread,
            lua,
        })
    }

    pub fn run(&self) -> LuaResult<()> {
        loop {
            let should_close = { self.rl.borrow().window_should_close() };

            if should_close {
                break;
            }

            let dt = self.rl.borrow().get_frame_time();
            self.api.update(dt, &self.lua)?;
            self.api.draw(&self.lua)?;

            let mut rl = self.rl.borrow_mut();
            let mut d = rl.begin_drawing(&self.rl_thread);

            self.api.flush(&mut d);
        }

        Ok(())
    }
}
