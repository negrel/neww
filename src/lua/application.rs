use gtk::traits::GtkWindowExt;
use mlua::{FromLua, UserData};

use super::bindings::gtk::{Widget, Window};

pub struct Application {
    builder: gtk::Builder,
    window: Window,
}

impl Application {
    pub fn new(app: &gtk::Application, builder: gtk::Builder) -> Self {
        let app_window = gtk::ApplicationWindow::new(app);

        // Define body if there is one.
        if let Some(body) = builder.object::<gtk::Widget>("body") {
            // Initialize layer.
            gtk4_layer_shell::init_for_window(&app_window);

            app_window.set_child(Some(&body));
            app_window.present();
        }

        Self {
            builder,
            window: Window(app_window.into()),
        }
    }
}

impl UserData for Application {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("window", |_vm, this| Ok(this.window.clone()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("find_widget_by_id", |vm, this, id| {
            let widget = this
                .builder
                .object::<gtk::Widget>(String::from_lua(id, vm)?)
                .expect("Widget not found");
            Ok(Widget(widget))
        })
    }
}
