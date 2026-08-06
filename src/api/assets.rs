use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

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
    fonts: RefCell<AssetCache<Font>>,
    music_streams: RefCell<AssetCache<Music<'static>>>,
    sounds: RefCell<AssetCache<Sound<'static>>>,

    rl: Rc<RefCell<RaylibHandle>>,
    rl_audio: &'static RaylibAudio,
    thread: Rc<RaylibThread>,
}

impl AssetModule {
    pub fn new(
        rl: Rc<RefCell<RaylibHandle>>,
        thread: Rc<RaylibThread>,
        rl_audio: &'static RaylibAudio,
    ) -> Self {
        Self {
            textures: RefCell::new(AssetCache::new()),
            fonts: RefCell::new(AssetCache::new()),
            music_streams: RefCell::new(AssetCache::new()),
            sounds: RefCell::new(AssetCache::new()),
            rl,
            rl_audio,
            thread,
        }
    }

    pub fn get_texture(&self, key: &str) -> Option<Rc<Texture2D>> {
        self.textures.borrow().get(key)
    }

    pub fn get_font(&self, key: &str) -> Option<Rc<Font>> {
        self.fonts.borrow().get(key)
    }

    pub fn get_music_stream(&self, key: &str) -> Option<Rc<Music<'static>>> {
        self.music_streams.borrow().get(key)
    }

    pub fn get_sound(&self, key: &str) -> Option<Rc<Sound<'static>>> {
        self.sounds.borrow().get(key)
    }

    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let assets_table = lua.create_table()?;

        bind_func!(lua, assets_table, "load_texture", self, load_texture, (filename: String) -> String);
        bind_func!(lua, assets_table, "load_font", self, load_font, (filename: String) -> String);
        bind_func!(lua, assets_table, "load_music_stream", self, load_music_stream, (filename: String) -> String);
        bind_func!(lua, assets_table, "load_sound", self, load_sound, (filename: String) -> String);

        let engine: LuaTable = lua.globals().get("engine")?;
        engine.set("assets", assets_table)?;

        Ok(())
    }
}

impl AssetModule {
    fn load_texture(&self, filename: String) -> LuaResult<String> {
        let mut textures = self.textures.borrow_mut();

        let filename = fs::canonicalize(&filename)?.to_string_lossy().to_string();

        if !textures.items.contains_key(&filename) {
            let result = self.rl.borrow_mut().load_texture(&self.thread, &filename);

            let texture = match result {
                Ok(tex) => tex,
                Err(err) => {
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to load texture: '{}': {:?}",
                        filename, err
                    )));
                }
            };

            let texture = Rc::new(texture);

            textures.items.insert(filename.clone(), texture);
        }

        Ok(filename)
    }

    fn load_font(&self, filename: String) -> LuaResult<String> {
        let mut fonts = self.fonts.borrow_mut();

        let filename = fs::canonicalize(&filename)?.to_string_lossy().to_string();

        if !fonts.items.contains_key(&filename) {
            let result = self.rl.borrow_mut().load_font(&self.thread, &filename);

            let font = match result {
                Ok(font) => font,
                Err(err) => {
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to load font: '{}': {:?}",
                        filename, err
                    )));
                }
            };

            let font = Rc::new(font);

            fonts.items.insert(filename.clone(), font);
        }

        Ok(filename)
    }

    fn load_music_stream(&self, filename: String) -> LuaResult<String> {
        let filename = fs::canonicalize(&filename)?.to_string_lossy().to_string();

        {
            let songs = self.music_streams.borrow();
            if songs.items.contains_key(&filename) {
                return Ok(filename);
            }
        }

        let music = match self.rl_audio.new_music(&filename) {
            Ok(music) => music,
            Err(err) => {
                return Err(LuaError::RuntimeError(format!(
                    "Failed to load music stream: '{}' {:?}",
                    filename, err
                )));
            }
        };

        let music = Rc::new(music);

        {
            let mut songs = self.music_streams.borrow_mut();
            songs.items.insert(filename.clone(), music);
        }

        Ok(filename)
    }

    fn load_sound(&self, filename: String) -> LuaResult<String> {
        let filename = fs::canonicalize(&filename)?.to_string_lossy().to_string();

        {
            let sounds = self.sounds.borrow();
            if sounds.items.contains_key(&filename) {
                return Ok(filename);
            }
        }

        let sound = match self.rl_audio.new_sound(&filename) {
            Ok(sound) => sound,
            Err(err) => {
                return Err(LuaError::RuntimeError(format!(
                    "Failed to load sound: '{}' {:?}",
                    filename, err
                )));
            }
        };

        let sound = Rc::new(sound);

        {
            let mut sounds = self.sounds.borrow_mut();
            sounds.items.insert(filename.clone(), sound);
        }

        Ok(filename)
    }
}
