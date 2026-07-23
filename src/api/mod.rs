use crate::api::assets::AssetModule;
use crate::api::game::GameModule;
use crate::api::graphics::GraphicsModule;
use crate::api::input::InputModule;
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use mlua::prelude::*;
use raylib::prelude::*;

pub mod assets;
pub mod game;
pub mod graphics;
pub mod input;

pub struct API {
    draw: Rc<GraphicsModule>,
    input: Rc<InputModule>,
    assets: Rc<AssetModule>,
}

impl API {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>, thread: Rc<RaylibThread>) -> Self {
        let assets = Rc::new(AssetModule::new(rl.clone(), thread.clone()));
        Self {
            draw: Rc::new(GraphicsModule::new(assets.clone())),
            input: Rc::new(InputModule::new(rl.clone())),
            assets,
        }
    }

    pub fn init(&self, lua: &Lua) -> LuaResult<()> {
        lua.globals().set("engine", lua.create_table()?)?;

        self.assets.register(lua)?;
        self.draw.register(lua)?;
        self.input.register(lua)?;

        let require = lua.create_require_function(LuaFsRequirer::default())?;
        lua.globals().set("require", require)?;

        let init_script = Path::new("scripts/init.luau").to_path_buf();
        let init_script = fs::read_to_string(init_script)?;

        lua.load(init_script).exec()?;

        Ok(())
    }

    pub fn update(&self, dt: f32, lua: &Lua) -> LuaResult<()> {
        GameModule::update(dt, lua)
    }

    pub fn draw(&self, lua: &Lua) -> LuaResult<()> {
        GameModule::draw(lua)
    }

    pub fn flush(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        self.draw.execute_commands(d);
    }
}
