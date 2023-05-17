use gtk::{
    gdk::{self, prelude::MonitorExt},
    glib,
};
use mlua::UserData;

use crate::{
    add_field_getter, add_mapped_field_getter,
    lua::bindings::{gdk::Rectangle, glib::GString},
};

pub struct Monitor(pub gdk::Monitor);

impl UserData for Monitor {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, connector, connector, |c: Option<glib::GString>| c
            .map(GString));

        add_field_getter!(fields, scale_factor, scale_factor);

        add_mapped_field_getter!(fields, geometry, geometry, Rectangle);

        fields.add_field_method_get("scaled_geometry", |_vm, this| {
            let rect = this.0.geometry();
            let scale_factor = this.0.scale_factor();
            let scaled_rect = gdk::Rectangle::new(
                rect.x(),
                rect.y(),
                rect.width() * scale_factor,
                rect.height() * scale_factor,
            );

            Ok(Rectangle(scaled_rect))
        })
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
