use gtk::gdk;
use mlua::UserData;

pub struct Cursor(pub gdk::Cursor);

impl UserData for Cursor {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
