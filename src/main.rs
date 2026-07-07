use std::rc::Rc;

use mlua::{Function, prelude::*};

use crate::api::API;

mod api;
mod core;
mod macros;

// TODO: create an "Application" struct
// TODO: automatically load luau files

fn main() -> LuaResult<()> {
    let api = API::new();

    let lua = Lua::new();
    api.init(&lua)?;

    lua.load(
        r#"
        function on_update(dt)
            draw.clear("BLACK")

            if input.is_key_down("A") then
                draw.text("KEY A HOLDING", 12, 30, 20, "WHITE")
            end
        end
    "#,
    )
    .exec()?;

    let on_update_fn: Function = lua
        .globals()
        .get("on_update")
        .expect("Failed to get on_update function");

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World!").build();
    while !rl.window_should_close() {
        let _: () = on_update_fn.call(())?;
        let mut d = rl.begin_drawing(&thread);

        api.run_draw(&mut d);
    }

    Ok(())
}
