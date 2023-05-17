use std::path::PathBuf;

use env_logger::Env;
use gtk::{prelude::*, Builder};

use crate::lua::application::Application;

mod lua;
mod ui;

fn activate(app: &gtk::Application) {
    let binding = std::env::args().collect::<Vec<_>>();
    let ui_filepath = binding.get(1).expect("UI file path is missing");
    let ui_filepath: PathBuf = ui_filepath.into();
    let ui_filepath = ui_filepath.canonicalize().expect("UI file doesn't exist");
    // Safe to unwrap because path is absolute.
    let ui_filepath_parent = ui_filepath.parent().unwrap();

    // Read UI file.
    log::debug!("reading {ui_filepath:?}...");
    let ui_file = std::fs::read_to_string(&ui_filepath).expect("Failed to read UI file");
    log::debug!("{ui_filepath:?} read.");

    // Setting CWD.
    log::debug!("changing working directory...");
    std::env::set_current_dir(ui_filepath_parent).expect("Failed to change working directory");
    log::debug!("working directory successfully changed.");

    // Parse UI file.
    log::debug!("parsing {ui_filepath:?}...");
    let ui = ui::parse(&ui_file).expect("Failed to parse UI file");
    log::debug!("{ui_filepath:?} parsed.");

    // Build UI.
    log::debug!("building GTK UI...");
    let builder = Builder::new();
    builder
        .add_from_string(&ui.gtk_ui)
        .expect("Failed to build GTK UI");
    log::debug!("GTK UI built.");

    let neww_app = Application::new(app, builder);

    // Load scripts.
    log::debug!("loading lua scripts...");
    let lua_vm = lua::new_vm(neww_app).expect("Failed to initialize lua VM");
    let mut chunks = Vec::new();
    for script in ui.scripts {
        chunks.push(lua_vm.load(Box::leak(Box::new(script.source))));
    }
    log::debug!("lua scripts loaded.");

    // Execute scripts.
    log::debug!("executing lua scripts...");
    for chunk in chunks {
        chunk.exec().expect("Failed to execute lua script");
    }
    log::debug!("lua scripts executed.");
}

fn main() {
    // Initialize logger.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = gtk::Application::builder()
        .application_id("dev.negrel.neww")
        .build();

    app.connect_activate(activate);

    app.run_with_args(&[""]);
}
