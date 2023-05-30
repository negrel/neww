use anyhow::Context;
use gtk::{traits::GtkWindowExt, Builder};
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

        // Execute lua scripts.
        let lua_vm = lua::new_vm(app).expect("Failed to initialize lua VM");
        if let Some(meta) = app_ui.meta {
            Self::exec_lua_scripts(lua_vm, meta.scripts);
        }
    }

    fn build_gtk_ui(app_iface: ui::Interface) -> gtk::Builder {
        log::debug!("building GTK UI...");
        let builder = Builder::new();
        builder
            .add_from_string(
                &ui::gtk::serialize(&app_iface.into()).expect("GTK UI serialization failed"),
            )
            .expect("Failed to build GTK UI");
        log::debug!("GTK UI built.");

        builder
    }

    fn exec_lua_scripts(lua_vm: &Lua, scripts: Vec<ui::Script>) {
        log::debug!("executing lua scripts...");
        if scripts
            .iter()
            .any(|s| s.source_path.is_none() && s.inline.is_none())
        {
            log::debug!("no lua script.");
            return;
        }

        // Load script files.
        let scripts: Result<Vec<String>, anyhow::Error> = scripts
            .into_iter()
            .map(|s| {
                if let Some(filepath) = s.source_path {
                    Ok(std::fs::read_to_string(filepath.clone())
                        .context(format!("Failed to read {filepath:?} lua script"))?)
                } else if let Some(source) = s.inline {
                    Ok(source)
                } else {
                    Ok("".to_owned())
                }
            })
            .collect();
        let scripts = scripts.unwrap();

        for script in scripts {
            let chunk = lua_vm.load(&script);
            chunk
                .exec()
                .context("Failed to execute lua script")
                .unwrap()
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
