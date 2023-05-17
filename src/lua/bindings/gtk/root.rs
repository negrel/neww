use gtk::prelude::RootExt;
use mlua::UserData;

use crate::{add_mapped_field_getter, lua::bindings::gdk::Display};

pub struct Root(pub gtk::Root);

impl UserData for Root {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, display, display, Display);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
