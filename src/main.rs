use env_logger::Env;
use gtk::{prelude::*, Builder};

mod lua;
mod ui;

fn activate(app: &gtk::Application) {
    let ui_filepath = "./examples/helloworld/neww.ui";

    // Read UI file.
    log::debug!("reading {ui_filepath:?}...");
    let ui_file = std::fs::read_to_string(ui_filepath).expect("Failed to read UI file");
    log::debug!("{ui_filepath:?} read.");

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
    let widget = builder
        .object::<gtk::Widget>("body")
        .expect("No body widget found");
    let app_window = gtk::ApplicationWindow::new(app);
    app_window.set_child(Some(&widget));
    log::debug!("GTK UI built.");

    // Load scripts.
    log::debug!("loading lua scripts...");
    let lua_vm = lua::new_vm(app_window).expect("Failed to initialize lua VM");
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

    app.run();
}
