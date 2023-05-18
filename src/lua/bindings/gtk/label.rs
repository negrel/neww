use gtk::prelude::Cast;
use mlua::{chunk, MetaMethod, UserData};

use crate::{
    add_field_getter, add_field_setter, add_mapped_field_getter, add_upcast_methods,
    lua::bindings::gtk::{Justification, Widget},
};

#[derive(Debug, Clone)]
pub struct Label(pub gtk::Label);

impl UserData for Label {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_field_getter!(fields, use_markup, uses_markup);
        add_field_setter!(fields, use_markup, set_use_markup);

        add_field_getter!(fields, use_underline, uses_underline);
        add_field_setter!(fields, use_underline, set_use_underline);

        add_field_getter!(fields, lines, lines);
        add_field_setter!(fields, lines, set_lines);

        add_field_getter!(fields, max_width_chars, max_width_chars);
        add_field_setter!(fields, max_width_chars, set_max_width_chars);

        add_field_getter!(fields, selectable, is_selectable);
        add_field_setter!(fields, selectable, set_selectable);

        add_mapped_field_getter!(fields, justify, justify, Justification);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);
    }
}
