use gtk::gdk;
use mlua::UserData;

use crate::add_field_getter;

pub struct Rectangle(pub gdk::Rectangle);

impl UserData for Rectangle {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_field_getter!(fields, x, x);
        add_field_getter!(fields, y, y);

        add_field_getter!(fields, width, width);
        add_field_getter!(fields, height, height);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
