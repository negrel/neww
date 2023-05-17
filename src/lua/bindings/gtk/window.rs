use gtk::{prelude::Cast, traits::GtkWindowExt};
use mlua::UserData;

use crate::{
    add_field_getter, add_field_setter, add_mapped_field_getter, add_method_no_args_no_return,
    add_upcast_method,
    lua::bindings::{
        gtk::{Root, Widget},
        gtk_layer_shell::LayerShell,
    },
};

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

        add_mapped_field_getter!(fields, child, child, |widget: Option<gtk::Widget>| {
            widget.map(Widget)
        });
        fields.add_field_method_set("child", |_vm, this, widget: Option<Widget>| {
            match widget {
                Some(w) => this.0.set_child(Some(&w.0)),
                None => this.0.set_child(None::<&gtk::Widget>),
            };
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
        add_method_no_args_no_return!(methods, present);

        add_upcast_method!(methods, Widget);
        add_upcast_method!(methods, Root);

        methods.add_method("shell", |_vm, this, ()| Ok(LayerShell::new(this.0.clone())));
    }
}
