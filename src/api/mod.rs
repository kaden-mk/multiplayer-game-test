use crate::api::assets::AssetModule;
use crate::api::game::GameModule;
use crate::api::graphics::GraphicsModule;
use crate::api::input::InputModule;
use std::cell::RefCell;
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
    game: Rc<RefCell<GameModule>>,
    assets: Rc<AssetModule>,
}

impl API {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>, thread: Rc<RaylibThread>) -> Self {
        let assets = Rc::new(AssetModule::new(rl.clone(), thread.clone()));
        Self {
            draw: Rc::new(GraphicsModule::new(assets.clone())),
            input: Rc::new(InputModule::new(rl.clone())),
            game: Rc::new(RefCell::new(GameModule::new())),
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

        Ok(())
    }

    pub fn register_script(&self, lua: &Lua, content: &str, name: &str) -> LuaResult<()> {
        self.game.borrow_mut().register_script(lua, content, name)
    }

    pub fn update(&self, dt: f32) -> LuaResult<()> {
        self.game.borrow().update(dt)?;

        Ok(())
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        self.draw.execute_commands(d);
    }
}
