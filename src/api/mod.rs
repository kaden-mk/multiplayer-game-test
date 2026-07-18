use crate::Rc;
use crate::api::assets::AssetModule;
use crate::api::draw::DrawModule;
use crate::api::game::GameModule;
use crate::api::input::InputModule;
use std::cell::RefCell;

use mlua::prelude::*;
use raylib::prelude::*;

pub mod assets;
pub mod draw;
pub mod game;
pub mod input;

pub struct API {
    draw: Rc<DrawModule>,
    input: Rc<InputModule>,
    game: Rc<RefCell<GameModule>>,
    assets: Rc<AssetModule>,
}

impl API {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>, thread: Rc<RaylibThread>) -> Self {
        let assets = Rc::new(AssetModule::new(rl.clone(), thread.clone()));
        Self {
            draw: Rc::new(DrawModule::new(assets.clone())),
            input: Rc::new(InputModule::new(rl.clone())),
            game: Rc::new(RefCell::new(GameModule::new())),
            assets,
        }
    }

    pub fn init(&self, lua: &Lua) -> LuaResult<()> {
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
