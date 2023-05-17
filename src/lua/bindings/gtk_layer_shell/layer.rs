use anyhow::anyhow;
use mlua::{FromLua, ToLua};

pub struct Layer(pub gtk4_layer_shell::Layer);

impl TryFrom<&str> for Layer {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "background" => Ok(Self(gtk4_layer_shell::Layer::Background)),
            "bottom" => Ok(Self(gtk4_layer_shell::Layer::Bottom)),
            "top" => Ok(Self(gtk4_layer_shell::Layer::Top)),
            "overlay" => Ok(Self(gtk4_layer_shell::Layer::Overlay)),
            "entry_number" => Ok(Self(gtk4_layer_shell::Layer::EntryNumber)),
            _ => Err(anyhow!(
                // We're not documenting entry_number as it is not used.
                r#"layer must be one of "background", "bottom", "top", "overlay", got {value:?}"#
            )),
        }
    }
}

impl ToString for Layer {
    fn to_string(&self) -> String {
        match self.0 {
            gtk4_layer_shell::Layer::Background => "background",
            gtk4_layer_shell::Layer::Bottom => "bottom",
            gtk4_layer_shell::Layer::Top => "top",
            gtk4_layer_shell::Layer::Overlay => "overlay",
            gtk4_layer_shell::Layer::EntryNumber => "entry_number",
            _ => unreachable!(),
        }
        .to_owned()
    }
}

impl<'lua> ToLua<'lua> for Layer {
    fn to_lua(self, vm: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::String(vm.create_string(&self.to_string())?))
    }
}

impl<'lua> FromLua<'lua> for Layer {
    fn from_lua(lua_value: mlua::Value<'lua>, _vm: &'lua mlua::Lua) -> mlua::Result<Self> {
        let layer_str = match lua_value {
            mlua::Value::String(str) => str,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: lua_value.type_name(),
                    to: "layer",
                    message: None,
                })
            }
        };

        let layer_string =
            layer_str
                .to_str()
                .map_err(|err| mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "layer",
                    message: Some(err.to_string()),
                })?;

        Layer::try_from(layer_string).map_err(|err| mlua::Error::FromLuaConversionError {
            from: "string",
            to: "layer",
            message: Some(err.to_string()),
        })
    }
}

#[cfg(test)]
mod test {
    use claims::assert_ok;

    use crate::lua::gtk_layer_shell::Layer;

    #[test]
    fn from_string_to_string() {
        for layer_str in ["background", "bottom", "top", "overlay"] {
            let result = Layer::try_from(layer_str);
            assert_ok!(&result);

            let layer = result.unwrap();
            assert_eq!(layer_str, layer.to_string());
        }
    }
}
