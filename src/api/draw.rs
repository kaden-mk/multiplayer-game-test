use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{bind_func, core::util::to_color};

pub struct TextData {
    text: String,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
}

pub enum DrawCommand {
    ClearBackground(Color),
    DrawText(TextData),
}

pub struct DrawModule {
    commands: Rc<RefCell<Vec<DrawCommand>>>,
}

impl DrawModule {
    pub fn new() -> Self {
        Self {
            commands: Rc::new(RefCell::new(Vec::<DrawCommand>::new())),
        }
    }

    fn clear_background(&self, color: String) {
        let color = to_color(&color.to_uppercase().as_str());
        self.commands
            .borrow_mut()
            .push(DrawCommand::ClearBackground(color));
    }

    fn draw_text(&self, text: String, x: i32, y: i32, font_size: i32, color: String) {
        let color = to_color(&color.to_uppercase().as_str());
        let text_data = TextData {
            text,
            x,
            y,
            font_size,
            color,
        };

        self.commands
            .borrow_mut()
            .push(DrawCommand::DrawText(text_data));
    }
}

impl DrawModule {
    pub fn register(&self, lua: &mlua::prelude::Lua) -> mlua::prelude::LuaResult<()> {
        let draw_table = lua.create_table()?;
        let this = Rc::new(Self {
            commands: Rc::clone(&self.commands),
        });

        bind_func!(lua, draw_table, "clear", instance this, clear_background, (color: String));
        bind_func!(lua, draw_table, "text", instance this, draw_text, (text: String, x: i32, y: i32, font_size: i32, color: String));

        lua.globals().set("draw", draw_table)?;
        Ok(())
    }

    pub fn execute_commands(&self, d: &mut RaylibDrawHandle) {
        let mut commands = self.commands.borrow_mut();

        for command in commands.iter() {
            match command {
                DrawCommand::ClearBackground(color) => {
                    d.clear_background(*color);
                }

                DrawCommand::DrawText(data) => {
                    d.draw_text(&data.text, data.x, data.y, data.font_size, data.color);
                }
            }
        }

        commands.clear();
    }
}
