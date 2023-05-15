use gtk::prelude::Cast;
use mlua::UserData;

use super::{gtk::Window, gtk_layer_shell::LayerShell};

pub struct Application {
    window: Window,
    layer_shell: LayerShell,
}

impl Application {
    pub fn new(window: gtk::ApplicationWindow) -> Self {
        Self {
            window: Window(window.clone().into()),
            layer_shell: LayerShell::new(window.upcast()),
        }
    }
}

impl UserData for Application {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("window", |_vm, this| Ok(this.window.clone()));
        fields.add_field_method_get("shell", |_vm, this| Ok(this.layer_shell.clone()))
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
