use crate::Rc;
use crate::api::draw::DrawModule;
use crate::api::input::InputModule;

use mlua::prelude::*;
use raylib::prelude::*;

pub mod draw;
pub mod input;

pub struct API {
    draw: Rc<DrawModule>,
    input: Rc<InputModule>,
}

impl API {
    pub fn new() -> Self {
        Self {
            draw: Rc::new(DrawModule::new()),
            input: Rc::new(InputModule::new()),
        }
    }

    pub fn init(&self, lua: &Lua) -> LuaResult<()> {
        self.draw.register(lua)?;
        self.input.register(lua)?;

        Ok(())
    }

    pub fn run_draw(&self, d: &mut RaylibDrawHandle) {
        self.draw.execute_commands(d);
    }
}
