use gtk::{
    gdk::{self, prelude::DisplayExt},
    prelude::Cast,
};
use mlua::UserData;

use crate::{add_field_getter, add_method_no_args_no_return, lua::bindings::gdk::Monitor};

pub struct Display(pub gdk::Display);

impl UserData for Display {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_field_getter!(fields, composited, is_composited);
        add_field_getter!(fields, input_shapes, is_input_shapes);
        add_field_getter!(fields, rgba, is_rgba);

        fields.add_field_method_get("monitors", |_vm, this| {
            let monitors: Result<Vec<_>, _> = this.0.monitors().into_iter().collect();
            let monitors = monitors.expect("Monitors ListModel modified during iteration.");

            let monitors: Result<Vec<_>, _> =
                monitors.into_iter().map(|g_obj| g_obj.downcast()).collect();
            let monitors = monitors.map_err(|_| mlua::Error::ToLuaConversionError {
                from: "GObject",
                to: "Monitor",
                message: None,
            })?;
            let monitors: Vec<_> = monitors.into_iter().map(Monitor).collect();

            Ok(monitors)
        })
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_method_no_args_no_return!(methods, beep);
    }
}
