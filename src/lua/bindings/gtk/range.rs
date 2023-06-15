use gtk::{
    glib,
    prelude::{ObjectExt, RangeExt},
};
use mlua::UserData;

use crate::{
    add_connect_methods, add_field_getter, add_field_setter, add_mapped_field_getter,
    add_mapped_field_setter, add_upcast_methods,
    lua::bindings::gtk::{Adjustment, Widget},
};

#[derive(Debug, Clone)]
pub struct Range(pub gtk::Range);

impl UserData for Range {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, adjustment, adjustment, Adjustment);
        add_mapped_field_setter!(fields, adjustment, set_adjustment, Adjustment);

        add_field_getter!(fields, fill_level, fill_level);
        add_field_setter!(fields, fill_level, set_fill_level);

        add_field_getter!(fields, flippable, is_flippable);
        add_field_setter!(fields, flippable, set_flippable);

        add_field_getter!(fields, inverted, is_inverted);
        add_field_setter!(fields, inverted, set_inverted);

        add_field_getter!(fields, restricts_to_fill_level, restricts_to_fill_level);

        add_field_getter!(fields, round_digits, round_digits);

        fields.add_field_method_get("slider_range", |_vm, this| {
            let range = this.0.slider_range();
            Ok(SliderRange(range.0, range.1))
        });

        add_field_getter!(fields, is_slider_size_fixed, is_slider_size_fixed);

        add_field_getter!(fields, value, value);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);

        add_connect_methods!(
            methods,
            "value_changed" as fn(Self) -> (),
            "change_value" as fn(Self) -> ()
        );
    }
}

#[derive(Debug, Clone)]
struct SliderRange(pub i32, pub i32);

impl UserData for SliderRange {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |_vm, this| Ok(this.0));
        fields.add_field_method_get("end", |_vm, this| Ok(this.1));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
