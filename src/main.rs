use std::rc::Rc;

use mlua::prelude::*;

use crate::{api::API, core::loader::load_scripts};

mod api;
mod core;
mod macros;

// TODO: create an "Application" struct

fn main() -> LuaResult<()> {
    let api = API::new();

    let lua = Lua::new();
    api.init(&lua)?;

    load_scripts(&lua, "scripts", &api)?;

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World!").build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        api.update(&mut d)?;
    }

    Ok(())
}
