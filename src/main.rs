use mlua::prelude::*;
use multiplayer_game_test::core::application::Application;

fn main() -> LuaResult<()> {
    let mut app = Application::new()?;
    app.load_scripts("scripts")?;
    app.run()?;

    Ok(())
}
