use gtk::gdk::{self, prelude::DragExt};
use mlua::UserData;

use crate::{
    add_mapped_field_getter,
    lua::bindings::gdk::{surface::Surface, Display},
};

pub struct Drag(gdk::Drag);

impl UserData for Drag {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, display, display, Display);
        add_mapped_field_getter!(fields, surface, surface, Surface);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
