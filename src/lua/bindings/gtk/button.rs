use gtk::{glib, prelude::ButtonExt};
use mlua::UserData;

use crate::{
    add_child_accessors, add_field_getter, add_field_setter, add_mapped_field_getter,
    add_mapped_field_setter, add_upcast_methods,
    lua::bindings::{glib::GString, gtk::Widget},
};

#[derive(Debug, Clone, Default)]
pub struct Button(pub gtk::Button);

impl UserData for Button {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_child_accessors!(fields);

        add_mapped_field_getter!(fields, label, label, |label: Option<glib::GString>| label
            .map(GString));
        add_mapped_field_setter!(fields, label, set_label, GString);

        add_field_getter!(fields, frame, has_frame);
        add_field_setter!(fields, frame, set_has_frame);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);
    }
}
