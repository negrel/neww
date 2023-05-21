use gtk::{glib, traits::WidgetExt};
use mlua::{FromLua, UserData};

use crate::{
    add_downcast_method, add_field_getter, add_field_setter, add_mapped_field_getter,
    add_mapped_field_setter, add_method_no_args_no_return,
    lua::bindings::{
        glib::GString,
        gtk::{Box, Button, Label, Window},
    },
};

#[derive(Debug, Clone)]
pub struct Widget(pub gtk::Widget);

impl UserData for Widget {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_mapped_field_getter!(fields, css_name, css_name, GString);

        add_mapped_field_getter!(fields, css_classes, css_classes, |classes: Vec<
            glib::GString,
        >| classes
            .into_iter()
            .map(GString)
            .collect::<Vec<GString>>());
        fields.add_field_method_set("css_classes", |vm, this, value| {
            let css_classes: Vec<mlua::String> = Vec::from_lua(value, vm)?;
            let css_classes: Result<Vec<_>, _> =
                css_classes.iter().map(|class| class.to_str()).collect();
            let css_classes = css_classes?;

            this.0.set_css_classes(&css_classes);
            Ok(())
        });

        add_field_getter!(fields, activate, activate);
        add_field_getter!(fields, can_focus, can_focus);
        add_field_getter!(fields, can_target, can_target);
        add_field_getter!(fields, is_child_visible, is_child_visible);
        add_field_getter!(fields, gets_focus_on_click, gets_focus_on_click);
        add_field_getter!(fields, is_focusable, is_focusable);
        add_field_getter!(fields, has_tooltip, has_tooltip);
        add_field_getter!(fields, is_hexpand_set, is_hexpand_set);
        add_field_getter!(fields, is_mapped, is_mapped);

        // Getter and setter for hexpand and vexpand.
        add_field_getter!(fields, hexpand, hexpands);
        add_field_setter!(fields, hexpand, set_hexpand);
        add_field_getter!(fields, vexpand, vexpands);
        add_field_setter!(fields, vexpand, set_vexpand);

        // Margins getter and setter.
        add_field_getter!(fields, margin_bottom, margin_bottom);
        add_field_getter!(fields, margin_end, margin_end);
        add_field_getter!(fields, margin_start, margin_start);
        add_field_getter!(fields, margin_top, margin_top);
        add_field_setter!(fields, margin_bottom, set_margin_bottom);
        add_field_setter!(fields, margin_end, set_margin_end);
        add_field_setter!(fields, margin_start, set_margin_start);
        add_field_setter!(fields, margin_top, set_margin_top);

        // Getter and setter for size properties.
        add_field_getter!(fields, width, width);
        add_field_getter!(fields, height, height);
        add_field_getter!(fields, height_request, height_request);
        add_field_getter!(fields, width_request, width_request);
        add_field_setter!(fields, height_request, set_height_request);
        add_field_setter!(fields, width_request, set_width_request);

        add_field_getter!(fields, opacity, opacity);
        add_field_getter!(fields, is_realized, is_realized);
        add_field_getter!(fields, receives_default, receives_default);
        add_field_getter!(fields, scale_factor, scale_factor);
        add_field_getter!(fields, get_sensitive, get_sensitive);
        add_field_getter!(fields, vexpands, vexpands);
        add_field_getter!(fields, is_vexpand_set, is_vexpand_set);
        add_field_getter!(fields, get_visible, get_visible);
        add_field_getter!(fields, grab_focus, grab_focus);
        add_field_getter!(fields, has_default, has_default);
        add_field_getter!(fields, has_focus, has_focus);
        add_field_getter!(fields, has_visible_focus, has_visible_focus);
        add_field_getter!(fields, in_destruction, in_destruction);
        add_field_getter!(fields, is_drawable, is_drawable);
        add_field_getter!(fields, is_focus, is_focus);
        add_field_getter!(fields, is_sensitive, is_sensitive);
        add_field_getter!(fields, should_layout, should_layout);

        // Getter and setter for widget_name.
        add_mapped_field_getter!(fields, widget_name, widget_name, GString);
        add_mapped_field_setter!(fields, widget_name, set_widget_name, GString);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_method_no_args_no_return!(methods, show);

        add_downcast_method!(methods, Window);
        add_downcast_method!(methods, Button);
        add_downcast_method!(methods, Label);
        add_downcast_method!(methods, Box);
    }
}
