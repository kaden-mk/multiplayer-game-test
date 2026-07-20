use mlua::prelude::*;
use raylib::prelude::*;

use crate::core::util::to_color;

pub struct LuaColor(pub Color);

impl LuaColor {
    pub fn create(lua: &Lua) -> LuaResult<()> {
        let t = lua.create_table()?;

        let func = lua.create_function(|lua, value: LuaMultiValue| {
            let color = LuaColor::from_lua_multi(value, lua)?;
            let result = lua.create_table()?;
            result.set("r", color.0.r)?;
            result.set("g", color.0.g)?;
            result.set("b", color.0.b)?;
            result.set("a", color.0.a)?;

            Ok(result)
        })?;

        t.set("new", func)?;

        lua.globals().set("Color", t)?;

        Ok(())
    }
}

impl LuaColor {
    fn from_lua(value: LuaValue) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(LuaColor(to_color(&s.to_str()?.to_uppercase())?)),
            LuaValue::Table(t) => {
                let c = if t.contains_key(1)? {
                    Color {
                        r: t.get(1)?,
                        g: t.get::<Option<u8>>(2)?.unwrap_or(0),
                        b: t.get::<Option<u8>>(3)?.unwrap_or(0),
                        a: t.get::<Option<u8>>(4)?.unwrap_or(255),
                    }
                } else {
                    Color {
                        r: t.get::<Option<u8>>("r")?.unwrap_or(0),
                        g: t.get::<Option<u8>>("g")?.unwrap_or(0),
                        b: t.get::<Option<u8>>("b")?.unwrap_or(0),
                        a: t.get::<Option<u8>>("a")?.unwrap_or(255),
                    }
                };

                Ok(LuaColor(c))
            }

            other => Err(LuaError::FromLuaConversionError {
                from: other.type_name(),
                to: "Color".into(),
                message: Some("expected color name or { r, g, b, a }".into()),
            }),
        }
    }
}

impl FromLuaMulti for LuaColor {
    fn from_lua_multi(multi_value: LuaMultiValue, _lua: &Lua) -> LuaResult<Self> {
        let mut iter = multi_value.into_iter();

        match iter.next() {
            Some(LuaValue::Table(t)) => LuaColor::from_lua(LuaValue::Table(t)),
            Some(LuaValue::String(s)) => LuaColor::from_lua(LuaValue::String(s)),

            Some(LuaValue::Integer(r)) => {
                let r = r as u8;

                let g = match iter.next() {
                    Some(LuaValue::Integer(v)) => v as u8,
                    _ => r,
                };

                let b = match iter.next() {
                    Some(LuaValue::Integer(v)) => v as u8,
                    _ => r as u8,
                };

                let a = match iter.next() {
                    Some(LuaValue::Integer(v)) => v as u8,
                    _ => 255,
                };

                Ok(LuaColor(Color { r, g, b, a }))
            }

            Some(LuaValue::Nil) => Ok(LuaColor(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            })),

            Some(other) => Err(LuaError::FromLuaConversionError {
                from: other.type_name(),
                to: "Color".into(),
                message: Some("expected table, color name, or r,g,b,a values".into()),
            }),

            None => Ok(LuaColor(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            })),
        }
    }
}

pub struct LuaRect(pub Rectangle);

impl LuaRect {
    pub fn create(lua: &Lua) -> LuaResult<()> {
        let t = lua.create_table()?;

        let func = lua.create_function(|lua, values: LuaMultiValue| {
            let rect = LuaRect::from_lua_multi(values, lua)?;
            let result = lua.create_table()?;
            result.set("x", rect.0.x)?;
            result.set("y", rect.0.y)?;
            result.set("width", rect.0.width)?;
            result.set("height", rect.0.height)?;

            Ok(result)
        })?;

        t.set("new", func)?;
        lua.globals().set("Rectangle", t)?;

        Ok(())
    }
}

impl FromLuaMulti for LuaRect {
    fn from_lua_multi(values: LuaMultiValue, _lua: &Lua) -> LuaResult<Self> {
        let mut iter = values.into_iter();

        match iter.next() {
            Some(LuaValue::Table(t)) => Ok(LuaRect(Rectangle {
                x: t.get("x")?,
                y: t.get("y")?,
                width: t.get("width")?,
                height: t.get("height")?,
            })),

            Some(LuaValue::Number(x)) => {
                let x = x as f32;

                let y = match iter.next() {
                    Some(LuaValue::Number(v)) => v as f32,
                    _ => x,
                };

                let width = match iter.next() {
                    Some(LuaValue::Number(v)) => v as f32,
                    _ => 2.0,
                };

                let height = match iter.next() {
                    Some(LuaValue::Number(v)) => v as f32,
                    _ => 1.0,
                };

                Ok(LuaRect(Rectangle {
                    x,
                    y,
                    width,
                    height,
                }))
            }

            Some(other) => Err(LuaError::FromLuaConversionError {
                from: other.type_name(),
                to: "Rectangle".into(),
                message: Some("expected { x, y, width, height } or (x, y, width, height) ".into()),
            }),

            None => Ok(LuaRect(Rectangle {
                x: 0.0,
                y: 0.0,
                width: 2.0,
                height: 1.0,
            })),
        }
    }
}
