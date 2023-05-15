use mlua::{FromLua, UserData};

use super::{Edge, Layer};

#[derive(Clone)]
pub struct LayerShell {
    pub window: gtk::Window,
}

impl LayerShell {
    pub fn new(window: gtk::Window) -> LayerShell {
        gtk4_layer_shell::init_for_window(&window);

        Self { window }
    }
}

impl UserData for LayerShell {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        // Getter & setter for layer.
        fields.add_field_method_get("layer", |_vm, this| {
            Ok(Layer(gtk4_layer_shell::layer(&this.window)))
        });
        fields.add_field_method_set("layer", |vm, this, layer| {
            gtk4_layer_shell::set_layer(&this.window, Layer::from_lua(layer, vm)?.0);
            Ok(())
        });

        // Getter & setter for auto_exclusive_zone
        fields.add_field_method_get("auto_exclusive_zone", |_vm, this| {
            Ok(gtk4_layer_shell::auto_exclusive_zone_is_enabled(
                &this.window,
            ))
        });
        fields.add_field_method_set("auto_exclusive_zone", |_vm, this, enable: bool| {
            if enable {
                gtk4_layer_shell::auto_exclusive_zone_enable(&this.window);
            } else {
                gtk4_layer_shell::set_exclusive_zone(&this.window, 0);
            }

            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Anchors
        methods.add_method("set_anchors", |vm, this, anchors: mlua::Table| {
            for pair in anchors.pairs() {
                let (key, value): (mlua::String, bool) = pair?;
                let edge = match Edge::from_lua(mlua::Value::String(key.clone()), vm) {
                    Ok(edge) => edge,
                    Err(err) => {
                        log::warn!(
                            "ignoring invalid edge {}: {err}",
                            key.to_str().unwrap_or("INVALID_UTF8_STRING")
                        );
                        continue;
                    }
                };

                gtk4_layer_shell::set_anchor(&this.window, edge.0, value)
            }
            Ok(())
        });
        methods.add_method("get_anchors", |vm, this, ()| {
            let anchors = vm.create_table()?;
            for edge in [
                gtk4_layer_shell::Edge::Top,
                gtk4_layer_shell::Edge::Bottom,
                gtk4_layer_shell::Edge::Left,
                gtk4_layer_shell::Edge::Right,
            ] {
                anchors.set(Edge(edge), gtk4_layer_shell::is_anchor(&this.window, edge))?;
            }

            Ok(anchors)
        });

        // Margins
        methods.add_method("set_margins", |vm, this, anchors: mlua::Table| {
            for pair in anchors.pairs() {
                let (key, value): (mlua::String, i32) = pair?;
                let edge = match Edge::from_lua(mlua::Value::String(key.clone()), vm) {
                    Ok(edge) => edge,
                    Err(err) => {
                        log::warn!(
                            "ignoring invalid edge {}: {err}",
                            key.to_str().unwrap_or("INVALID_UTF8_STRING")
                        );
                        continue;
                    }
                };

                gtk4_layer_shell::set_margin(&this.window, edge.0, value)
            }
            Ok(())
        });
        methods.add_method("get_margins", |vm, this, ()| {
            let anchors = vm.create_table()?;
            for edge in [
                gtk4_layer_shell::Edge::Top,
                gtk4_layer_shell::Edge::Bottom,
                gtk4_layer_shell::Edge::Left,
                gtk4_layer_shell::Edge::Right,
            ] {
                anchors.set(Edge(edge), gtk4_layer_shell::margin(&this.window, edge))?;
            }

            Ok(anchors)
        })
    }
}
