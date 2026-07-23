use mlua::prelude::*;
use multiplayer_game_test::core::application::Application;

fn main() -> LuaResult<()> {
    let app = Application::new()?;
    app.run()?;

    Ok(())
}
