use gtk::glib;
use mlua::UserData;

use crate::{
    add_mapped_field_getter, add_upcast_methods,
    lua::bindings::{glib::GString, gtk::Widget},
};

#[derive(Debug, Clone, Default)]
pub struct Image(pub gtk::Image);

impl UserData for Image {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, file, file, |file: Option<glib::GString>| file
            .map(GString));
        fields.add_field_method_set("file", |_vm, this, file: Option<String>| {
            this.0.set_file(file.as_deref());
            Ok(())
        })
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);
    }
}
