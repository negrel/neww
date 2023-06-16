use mlua::UserData;

use crate::{
    add_field_getter, add_field_setter, add_mapped_field_getter, add_mapped_field_setter,
    add_upcast_methods,
    lua::bindings::{
        glib::GString,
        gtk::{Justification, Widget},
        pango::WrapMode,
    },
};

#[derive(Debug, Clone, Default)]
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

        add_field_getter!(fields, xalign, xalign);
        add_field_getter!(fields, yalign, yalign);
        add_field_setter!(fields, yalign, set_yalign);
        add_field_setter!(fields, xalign, set_xalign);

        add_mapped_field_getter!(fields, justify, justify, Justification);
        fields.add_field_method_set(
            "justification",
            |_vm, this, justification: Justification| {
                this.0.set_justify(justification.0);
                Ok(())
            },
        );

        add_field_getter!(fields, wraps, wraps);
        add_field_setter!(fields, wraps, set_wrap);
        add_mapped_field_getter!(fields, wrap_mode, wrap_mode, WrapMode);
        fields.add_field_method_set("wrap_mode", |_vm, this, wrap_mode: WrapMode| {
            this.0.set_wrap_mode(wrap_mode.0);
            Ok(())
        });

        add_mapped_field_getter!(fields, label, label, GString);
        add_mapped_field_setter!(fields, label, set_label, GString);

        add_mapped_field_getter!(fields, text, text, GString);
        add_mapped_field_setter!(fields, text, set_text, GString);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);
    }
}
