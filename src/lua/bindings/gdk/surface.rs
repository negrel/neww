use gtk::gdk::{self, prelude::SurfaceExt};
use mlua::UserData;

use crate::lua::bindings::gdk::Cursor;

pub struct Surface(pub gdk::Surface);

impl UserData for Surface {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("cursor", |_vm, this| Ok(this.0.cursor().map(Cursor)));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
