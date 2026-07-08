use crate::Rc;
use crate::api::draw::DrawModule;
use crate::api::game::GameModule;
use crate::api::input::InputModule;
use std::cell::RefCell;

use mlua::prelude::*;
use raylib::prelude::*;

pub mod draw;
pub mod game;
pub mod input;

pub struct API {
    draw: Rc<DrawModule>,
    input: Rc<InputModule>,
    game: Rc<RefCell<GameModule>>,
}

impl API {
    pub fn new() -> Self {
        Self {
            draw: Rc::new(DrawModule::new()),
            input: Rc::new(InputModule::new()),
            game: Rc::new(RefCell::new(GameModule::new())),
        }
    }

    pub fn init(&self, lua: &Lua) -> LuaResult<()> {
        self.draw.register(lua)?;
        self.input.register(lua)?;

        Ok(())
    }

    pub fn register_script(&self, lua: &Lua, content: &str) -> LuaResult<()> {
        self.game.borrow_mut().register_script(lua, content)
    }

    pub fn update(&self, d: &mut RaylibDrawHandle) -> LuaResult<()> {
        self.game.borrow().update(d)?;
        self.draw.execute_commands(d);

        Ok(())
    }
}
