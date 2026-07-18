use std::{cell::RefCell, rc::Rc};

use mlua::{Vector, prelude::*};
use raylib::prelude::*;

use crate::{bind_func, core::util::to_keyboard_key};

pub struct InputModule {
    rl: Rc<RefCell<RaylibHandle>>,
}

impl InputModule {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>) -> Self {
        Self { rl }
    }

    fn is_key_down(&self, key: String) -> LuaResult<bool> {
        let key_code = to_keyboard_key(&key.to_uppercase().as_str())?;
        Ok(self.rl.borrow_mut().is_key_down(key_code))
    }

    fn set_cursor_visible(&self, visible: bool) -> LuaResult<()> {
        let mut rl = self.rl.borrow_mut();
        if visible {
            rl.show_cursor();
        } else {
            rl.hide_cursor();
        }

        Ok(())
    }

    fn get_mouse_position(&self) -> LuaResult<Vector> {
        let vector = self.rl.borrow_mut().get_mouse_position();
        let luau_vector = Vector::new(vector.x, vector.y, 0.0);

        Ok(luau_vector)
    }
}

impl InputModule {
    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let input_table = lua.create_table()?;

        bind_func!(lua, input_table, "is_key_down", instance self, is_key_down, (key: String));
        bind_func!(lua, input_table, "set_cursor_visible", instance self, set_cursor_visible, (visible: bool));
        bind_func!(lua, input_table, "get_mouse_position", instance self, get_mouse_position, ());

        lua.globals().set("input", input_table)?;
        Ok(())
    }
}
