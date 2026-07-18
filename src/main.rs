use std::rc::Rc;

use mlua::prelude::*;

use crate::core::application::Application;

mod api;
mod core;
mod macros;

fn main() -> LuaResult<()> {
    let mut app = Application::new()?;
    app.load_scripts("scripts")?;
    app.run()?;

    Ok(())
}
