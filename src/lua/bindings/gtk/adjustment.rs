use gtk::{
    glib,
    prelude::{AdjustmentExt, ObjectExt},
};
use mlua::UserData;

use crate::{add_connect_methods, add_field_getter, add_field_setter};

#[derive(Debug, Clone)]
pub struct Adjustment(pub gtk::Adjustment);

impl UserData for Adjustment {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_field_getter!(fields, lower, lower);
        add_field_setter!(fields, lower, set_lower);

        add_field_getter!(fields, upper, upper);
        add_field_setter!(fields, upper, set_upper);

        add_field_getter!(fields, value, value);
        add_field_setter!(fields, value, set_value);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_connect_methods!(
            methods,
            "changed" as fn(Self) -> (),
            "value_changed" as fn(Self) -> (),
            "lower_notify" as fn(Self) -> (),
            "upper_notify" as fn(Self) -> (),
            "value_notify" as fn(Self) -> ()
        );
    }
}
