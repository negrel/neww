use gtk::{gdk, traits::GtkWindowExt};
use mlua::{FromLua, Lua, UserData};

use crate::{
    lua::{
        self,
        bindings::gtk::{Widget, Window},
    },
    ui,
};

pub struct Application {
    builder: gtk::Builder,
    window: Window,
}

impl Application {
    pub fn run(app: &gtk::Application, app_ui: ui::Neww) {
        // Build UI.
        let builder = Self::build_gtk_ui(app_ui.interface);

        let app = Self::new(app, builder);

        if let Some(meta) = app_ui.meta {
            // Load CSS styles.
            let css_provider = gtk::CssProvider::new();
            Self::load_styles(&css_provider, meta.styles());
            if let Some(display) = gdk::Display::default() {
                gtk::style_context_add_provider_for_display(
                    &display,
                    &css_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }

            // Execute lua scripts.
            let lua_vm = lua::new_vm(app).expect("Failed to initialize lua VM");
            Self::exec_lua_scripts(lua_vm, meta.scripts());
        }
    }

    fn build_gtk_ui(app_iface: ui::Interface) -> gtk::Builder {
        log::debug!("building GTK UI...");
        let builder = gtk::Builder::new();
        builder
            .add_from_string(
                &ui::gtk::serialize(&app_iface.into()).expect("GTK UI serialization failed"),
            )
            .expect("Failed to build GTK UI");
        log::debug!("GTK UI built.");

        builder
    }

    fn load_styles<'a, I: Iterator<Item = &'a ui::Style>>(
        css_provider: &gtk::CssProvider,
        styles: I,
    ) {
        for style in styles {
            if let Some(source_path) = &style.source_path {
                css_provider.load_from_path(source_path)
            }
            if let Some(inline_css) = &style.inline {
                css_provider.load_from_data(inline_css)
            }
        }
    }

    fn exec_lua_scripts<'a, I: Iterator<Item = &'a ui::Script>>(lua_vm: &Lua, scripts: I) {
        log::debug!("executing lua scripts...");

        for script in scripts {
            if let Some(source_path) = &script.source_path {
                lua_vm
                    .load(source_path)
                    .exec()
                    .expect("Failed to execute lua script");
            }
            if let Some(inline_lua) = &script.inline {
                lua_vm
                    .load(inline_lua)
                    .exec()
                    .expect("Failed to execute inline lua script");
            }
        }

        log::debug!("lua scripts successfully executed.");
    }

    fn new(app: &gtk::Application, builder: gtk::Builder) -> Self {
        // Create application window.
        let app_window = gtk::ApplicationWindow::new(app);

        // Define body if there is one.
        if let Some(body) = builder.object::<gtk::Widget>("body") {
            log::debug!("body widget found, attaching it to application window.");
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
            let widget_id = String::from_lua(id, vm)?;
            let widget = this
                .builder
                .object::<gtk::Widget>(widget_id.clone())
                .unwrap_or_else(|| panic!("widget with id={widget_id:?} not found"));
            Ok(Widget(widget))
        })
    }
}
