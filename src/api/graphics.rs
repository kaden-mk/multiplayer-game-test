use mlua::prelude::*;
use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::{
    api::assets::AssetModule,
    bind_func,
    core::{types::LuaColor, util::to_color},
};

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

pub struct TextureDataEx {
    texture: Rc<Texture2D>,
    pos: Vector2,
    rot: f32,
    scale: f32,
    tint: Color,
}

pub enum DrawCommand {
    ClearBackground(Color),
    DrawText(TextData),
    DrawTexture(TextureData),
    DrawTextureEx(TextureDataEx),
}

pub struct GraphicsModule {
    commands: Rc<RefCell<Vec<DrawCommand>>>,
    assets: Rc<AssetModule>,
}

impl GraphicsModule {
    pub fn new(assets: Rc<AssetModule>) -> Self {
        Self {
            commands: Rc::new(RefCell::new(Vec::<DrawCommand>::new())),
            assets,
        }
    }

    fn clear_background(&self, color: String) -> LuaResult<()> {
        let color = to_color(&color.to_uppercase().as_str())?;
        self.commands
            .borrow_mut()
            .push(DrawCommand::ClearBackground(color));

        Ok(())
    }

    fn draw_text(
        &self,
        text: String,
        x: i32,
        y: i32,
        font_size: i32,
        color: LuaColor,
    ) -> LuaResult<()> {
        let text_data = TextData {
            text,
            x,
            y,
            font_size,
            color: color.0,
        };

        self.commands
            .borrow_mut()
            .push(DrawCommand::DrawText(text_data));

        Ok(())
    }

    fn draw_texture(&self, texture: String, x: i32, y: i32, tint: LuaColor) -> LuaResult<()> {
        match self.assets.get_texture(texture.as_str()) {
            Some(texture) => {
                let texture_data = TextureData {
                    texture,
                    x,
                    y,
                    tint: tint.0,
                };

                self.commands
                    .borrow_mut()
                    .push(DrawCommand::DrawTexture(texture_data));
            }
            None => {
                eprintln!("texture '{}' doesn't exist", texture);
            }
        }

        Ok(())
    }

    fn draw_texture_ex(
        &self,
        texture: String,
        pos: LuaVector,
        rot: f32,
        scale: f32,
        tint: LuaColor,
    ) -> LuaResult<()> {
        match self.assets.get_texture(texture.as_str()) {
            Some(texture) => {
                let texture_data = TextureDataEx {
                    texture,
                    pos: Vector2::new(pos.x(), pos.y()),
                    rot,
                    scale,
                    tint: tint.0,
                };

                self.commands
                    .borrow_mut()
                    .push(DrawCommand::DrawTextureEx(texture_data));
            }
            None => {
                eprintln!("texture '{}' doesn't exist", texture);
            }
        }

        Ok(())
    }
}

impl GraphicsModule {
    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let graphics_table = lua.create_table()?;

        bind_func!(lua, graphics_table, "clear_background", self, clear_background, (color: String) -> ());
        bind_func!(lua, graphics_table, "draw_text", self, draw_text, (text: String, x: i32, y: i32, font_size: i32, color: LuaColor) -> ());
        bind_func!(lua, graphics_table, "draw_texture", self, draw_texture, (texture: String, x: i32, y: i32, tint: LuaColor) -> ());
        bind_func!(lua, graphics_table, "draw_texture_ex", self, draw_texture_ex, (texture: String, pos: LuaVector, rot: f32, scale: f32, tint: LuaColor) -> ());

        let engine: LuaTable = lua.globals().get("engine")?;
        engine.set("graphics", graphics_table)?;

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

                DrawCommand::DrawTextureEx(texture) => {
                    d.draw_texture_ex(
                        texture.texture.as_ref(),
                        texture.pos,
                        texture.rot,
                        texture.scale,
                        texture.tint,
                    );
                }
            }
        }

        commands.clear();
    }
}
