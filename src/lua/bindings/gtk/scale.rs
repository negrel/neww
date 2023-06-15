use mlua::UserData;

use crate::{
    add_upcast_methods,
    lua::bindings::gtk::{Range, Widget},
};

#[derive(Debug, Clone)]
pub struct Scale(pub gtk::Scale);

impl UserData for Scale {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget, Range);
    }
}
