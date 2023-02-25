use std::path::PathBuf;

use mlua::lua_module;
use nvim_utils::prelude::*;

fn get_line_blame(lua: &Lua, (file, line): (String, usize)) -> mlua::Result<String> {
    Ok("Hello, world!".to_string())
}

struct Blam<'a> {
    lua: &'a Lua,
}

impl<'a> Blam<'a> {
    fn new(lua: &'a Lua) -> Self {
        Self { lua }
    }

    fn get_line_blame(&self, (file, line): (String, usize)) -> mlua::Result<String> {
        Ok("Hello, world!".to_string())
    }
}

impl<'a> LuaUserData for Blam<'a> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "get_line_blame",
            |lua, this, (file, line): (String, usize)| this.get_line_blame((file, line)),
        );
    }
}

#[lua_module]
fn register<'a>(lua: &'static Lua) -> LuaResult<LuaTable<'a>> {
    let d = Blam::new(lua).to_lua(lua)?;
    let LuaValue::Table(t) = d else {
        return Err(mlua::Error::FromLuaConversionError {
            from: "Blam",
            to: "LuaTable",
            message: None,
        });
    };
    Ok(t)
}
