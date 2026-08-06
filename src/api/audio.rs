use std::rc::Rc;

use mlua::prelude::*;

use crate::{api::assets::AssetModule, bind_func};

pub struct AudioModule {
    assets: Rc<AssetModule>,
}

impl AudioModule {
    pub fn new(assets: Rc<AssetModule>) -> Self {
        Self { assets }
    }

    pub fn register(self: &Rc<Self>, lua: &Lua) -> LuaResult<()> {
        let audio_table = lua.create_table()?;

        // music streams
        bind_func!(lua, audio_table, "play_music_stream", self, play_music_stream, (filename: String) -> ());
        bind_func!(lua, audio_table, "stop_music_stream", self, stop_music_stream, (filename: String) -> ());
        bind_func!(lua, audio_table, "pause_music_stream", self, pause_music_stream, (filename: String) -> ());
        bind_func!(lua, audio_table, "resume_music_stream", self, resume_music_stream, (filename: String) -> ());
        bind_func!(lua, audio_table, "set_music_stream_volume", self, set_music_stream_volume, (filename: String, volume: f32) -> ());
        bind_func!(lua, audio_table, "is_music_stream_playing", self, is_music_stream_playing, (filename: String) -> bool);
        bind_func!(lua, audio_table, "update_music_stream", self, update_music_stream, (filename: String) -> ());

        // sounds
        bind_func!(lua, audio_table, "play_sound", self, play_sound, (filename: String) -> ());

        let engine: LuaTable = lua.globals().get("engine")?;
        engine.set("audio", audio_table)?;

        Ok(())
    }
}

// sounds
impl AudioModule {
    fn play_sound(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_sound(&filename) {
            Some(sound) => {
                sound.play();
            },
            None => {
                eprintln!("sound '{}' does not exist", filename);
            }
        };

        Ok(())
    }
}

// music streams
impl AudioModule {
    fn play_music_stream(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.play_stream();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn stop_music_stream(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.stop_stream();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn pause_music_stream(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.pause_stream();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn resume_music_stream(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.resume_stream();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn update_music_stream(&self, filename: String) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.update_stream();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn set_music_stream_volume(&self, filename: String, volume: f32) -> LuaResult<()> {
        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                music.set_volume(volume);
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(())
    }

    fn is_music_stream_playing(&self, filename: String) -> LuaResult<bool> {
        let mut is_playing = false;

        match self.assets.get_music_stream(&filename) {
            Some(music) => {
                is_playing = music.is_stream_playing();
            },
            None => {
                eprintln!("music stream '{}' does not exist", filename);
            }
        };

        Ok(is_playing)
    }
}
