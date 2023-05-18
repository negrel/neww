use anyhow::anyhow;
use mlua::{FromLua, ToLua};

#[derive(Debug)]
pub struct Edge(pub gtk4_layer_shell::Edge);

impl TryFrom<&str> for Edge {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "left" => Ok(Self(gtk4_layer_shell::Edge::Left)),
            "right" => Ok(Self(gtk4_layer_shell::Edge::Right)),
            "bottom" => Ok(Self(gtk4_layer_shell::Edge::Bottom)),
            "top" => Ok(Self(gtk4_layer_shell::Edge::Top)),
            _ => Err(anyhow!(
                r#"edge must be one of "left", "right", "bottom", "top", got {value:?}"#
            )),
        }
    }
}

impl ToString for Edge {
    fn to_string(&self) -> String {
        match self.0 {
            gtk4_layer_shell::Edge::Left => "left",
            gtk4_layer_shell::Edge::Right => "right",
            gtk4_layer_shell::Edge::Bottom => "bottom",
            gtk4_layer_shell::Edge::Top => "top",
            _ => unreachable!(),
        }
        .to_owned()
    }
}

impl<'lua> ToLua<'lua> for Edge {
    fn to_lua(self, vm: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::String(vm.create_string(&self.to_string())?))
    }
}

impl<'lua> FromLua<'lua> for Edge {
    fn from_lua(lua_value: mlua::Value<'lua>, _vm: &'lua mlua::Lua) -> mlua::Result<Self> {
        let edge_str = match lua_value {
            mlua::Value::String(str) => str,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: lua_value.type_name(),
                    to: "edge",
                    message: None,
                })
            }
        };

        let edge_string = edge_str
            .to_str()
            .map_err(|err| mlua::Error::FromLuaConversionError {
                from: "string",
                to: "edge",
                message: Some(err.to_string()),
            })?;

        Edge::try_from(edge_string).map_err(|err| mlua::Error::FromLuaConversionError {
            from: "string",
            to: "edge",
            message: Some(err.to_string()),
        })
    }
}

#[cfg(test)]
mod test {
    use claims::assert_ok;

    use crate::lua::bindings::gtk_layer_shell::Edge;

    #[test]
    fn from_string_to_string() {
        for edge_str in ["top", "right", "bottom", "left"] {
            let result = Edge::try_from(edge_str);
            assert_ok!(&result);

            let edge = result.unwrap();
            assert_eq!(edge_str, edge.to_string());
        }
    }
}
