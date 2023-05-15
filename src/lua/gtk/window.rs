use gtk::traits::{GtkWindowExt, WidgetExt};
use mlua::{FromLua, UserData};

use crate::lua::glib::GString;

macro_rules! add_method_no_args_no_return {
    ($methods:ident, $method_name:ident) => {
        $methods.add_method(stringify!($method_name), |_vm, this, ()| {
            this.0.$method_name();
            Ok(())
        });
    };
}

macro_rules! add_field_getter {
    ($fields:ident, $field_name:ident, $getter_name: ident) => {
        $fields.add_field_method_get(stringify!($field_name), |_vm, this| {
            Ok(this.0.$getter_name())
        });
    };
}

macro_rules! add_field_setter {
    ($fields:ident, $field_name:ident, $setter_name: ident) => {
        $fields.add_field_method_set(stringify!($field_name), |_vm, this, value| {
            this.0.$setter_name(value);
            Ok(())
        })
    };
}

#[derive(Debug, Clone)]
pub struct Window(pub gtk::Window);

impl UserData for Window {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_field_getter!(fields, fullscreen, is_fullscreen);
        add_field_setter!(fields, fullscreen, set_fullscreened);

        add_field_getter!(fields, maximized, is_maximized);
        add_field_setter!(fields, maximized, set_maximized);

        add_field_getter!(fields, active, is_active);

        add_field_getter!(fields, deletable, is_deletable);
        add_field_setter!(fields, deletable, set_deletable);

        add_field_getter!(fields, decorated, is_decorated);
        add_field_setter!(fields, decorated, set_decorated);

        add_field_getter!(fields, hide_on_close, hides_on_close);
        add_field_setter!(fields, hide_on_close, set_hide_on_close);

        add_field_getter!(fields, resizable, is_resizable);
        add_field_setter!(fields, resizable, set_resizable);

        add_field_getter!(fields, default_width, default_width);
        add_field_setter!(fields, default_width, set_default_width);

        add_field_getter!(fields, default_height, default_height);
        add_field_setter!(fields, default_height, set_default_height);

        // Getter for css_name.
        fields.add_field_method_get("css_name", |_vm, this| Ok(GString(this.0.css_name())));

        // Getter & setter for css_classes.
        fields.add_field_method_get("css_classes", |_vm, this| {
            Ok(this
                .0
                .css_classes()
                .into_iter()
                .map(GString)
                .collect::<Vec<GString>>())
        });
        fields.add_field_method_set("css_classes", |vm, this, value| {
            let css_classes: Vec<mlua::String> = Vec::from_lua(value, vm)?;
            let css_classes: Result<Vec<_>, _> =
                css_classes.iter().map(|class| class.to_str()).collect();
            let css_classes = css_classes?;

            this.0.set_css_classes(&css_classes);
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_method_no_args_no_return!(methods, close);
        add_method_no_args_no_return!(methods, destroy);
        add_method_no_args_no_return!(methods, maximize);
        add_method_no_args_no_return!(methods, unmaximize);
        add_method_no_args_no_return!(methods, minimize);
        add_method_no_args_no_return!(methods, unminimize);
        add_method_no_args_no_return!(methods, show);
    }
}
