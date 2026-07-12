use mlua::prelude::*;
use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{api::assets::AssetModule, bind_func, core::util::to_color};

pub struct TextData {
    text: String,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
}

pub struct TextureData {
    texture: Rc<Texture2D>,
    x: i32,
    y: i32,
    tint: Color,
}

pub enum DrawCommand {
    ClearBackground(Color),
    DrawText(TextData),
    DrawTexture(TextureData),
}

pub struct DrawModule {
    commands: Rc<RefCell<Vec<DrawCommand>>>,
    assets: Rc<AssetModule>,
}

impl DrawModule {
    pub fn new(assets: Rc<AssetModule>) -> Self {
        Self {
            commands: Rc::new(RefCell::new(Vec::<DrawCommand>::new())),
            assets,
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

    fn draw_texture(&self, texture: String, x: i32, y: i32, tint: String) {
        match self.assets.get_texture(texture.as_str()) {
            Some(texture) => {
                let tint = to_color(&tint.to_uppercase().as_str());
                let texture_data = TextureData {
                    texture,
                    x,
                    y,
                    tint,
                };

                self.commands
                    .borrow_mut()
                    .push(DrawCommand::DrawTexture(texture_data));
            }
            None => {
                eprintln!("texture '{}' doesn't exist", texture);
            }
        }
    }
}

impl DrawModule {
    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let draw_table = lua.create_table()?;

        bind_func!(lua, draw_table, "clear", instance self, clear_background, (color: String));
        bind_func!(lua, draw_table, "text", instance self, draw_text, (text: String, x: i32, y: i32, font_size: i32, color: String));
        bind_func!(lua, draw_table, "texture", instance self, draw_texture, (texture: String, x: i32, y: i32, tint: String));

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

                DrawCommand::DrawTexture(texture) => {
                    d.draw_texture(texture.texture.as_ref(), texture.x, texture.y, texture.tint);
                }
            }
        }

        commands.clear();
    }
}
