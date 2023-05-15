use gtk::glib;
use mlua::{FromLua, ToLua};

pub struct GString(pub glib::GString);

impl<'lua> ToLua<'lua> for GString {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::String(lua.create_string(&self.0.to_string())?))
    }
}

impl<'lua> FromLua<'lua> for GString {
    fn from_lua(lua_value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let utf8 = match lua_value {
            mlua::Value::String(ref str) => str.as_bytes().to_vec(),
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: lua_value.type_name(),
                    to: "GString",
                    message: None,
                })
            }
        };

        let gstring =
            glib::GString::from_utf8(utf8).map_err(|err| mlua::Error::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "GString",
                message: Some(err.to_string()),
            })?;

        Ok(GString(gstring))
    }
}
