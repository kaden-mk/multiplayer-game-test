use std::{cell::RefCell, collections::HashMap, rc::Rc};

use mlua::prelude::*;
use raylib::prelude::*;

use crate::bind_func;

pub struct AssetCache<T> {
    items: HashMap<String, Rc<T>>,
}

impl<T> AssetCache<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<T>> {
        self.items.get(key).cloned()
    }
}

pub struct AssetModule {
    textures: RefCell<AssetCache<Texture2D>>,

    rl: Rc<RefCell<RaylibHandle>>,
    thread: Rc<RaylibThread>,
}

impl AssetModule {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>, thread: Rc<RaylibThread>) -> Self {
        Self {
            textures: RefCell::new(AssetCache::new()),
            rl,
            thread,
        }
    }

    pub fn get_texture(&self, key: &str) -> Option<Rc<Texture2D>> {
        self.textures.borrow().get(key)
    }

    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let assets_table = lua.create_table()?;

        bind_func!(lua, assets_table, "load_texture", instance self, load_texture, (filename: String));

        lua.globals().set("assets", assets_table)?;

        Ok(())
    }
}

impl AssetModule {
    fn load_texture(&self, filename: String) -> String {
        let mut textures = self.textures.borrow_mut();

        if !textures.items.contains_key(&filename) {
            let texture = self
                .rl
                .borrow_mut()
                .load_texture(&self.thread, &filename)
                .expect("failed to load texture");

            let texture = Rc::new(texture);

            textures.items.insert(filename.clone(), texture);
        }

        filename
    }
}
