use std::rc::Rc;

use mlua::{Function, prelude::*};

use crate::api::API;

mod api;
mod core;
mod macros;

// Note to self:
// I need to cleanup how draw_commands are handled and how we create/register functions to the api
// I think I should make a module that allows me to easily register functions to the api explicitly
// I also NEED to cleanup the expects omggg, i'll just use mlua's error handling?? idk
// And have another module that can handle the draw_commands bs for me, it's very awkward and messy
// I'll also make some sort of utility module that converts strings to raylib types, so "to_rl_color" "to_rl_bind", idk
// I'll either go roblox style with scripts or stick to a more sophisticated and simpler "on_update" "on_draw" and "on_init"
// After that I'll make some submodules that register functions and get to work on the actual game in luau (hype)

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
