use env_logger::Env;
use gtk::{prelude::*, Builder};

mod ui;

use mlua::Lua;

fn activate(app: &gtk::Application) {
    let lua = Lua::new();

    let ui_filepath = "./examples/helloworld/neww.ui";

    log::debug!("reading {ui_filepath:?}...");
    let ui_file = std::fs::read_to_string(ui_filepath).expect("Failed to read UI file");
    log::debug!("{ui_filepath:?} read.");

    log::debug!("parsing {ui_filepath:?}...");
    let ui = ui::parse(&ui_file).expect("Failed to parse UI file");
    log::debug!("{ui_filepath:?} parsed.");

    log::debug!("building GTK UI...");
    let builder = Builder::new();
    builder
        .add_from_string(&ui.gtk_ui)
        .expect("Failed to build GTK UI");
    log::debug!("GTK UI built.");

    let app_window = gtk::ApplicationWindow::builder()
        .application(app)
        .child(&builder.object::<gtk::Widget>("body").unwrap())
        .width_request(600)
        .height_request(400)
        .build();

    app_window.show()
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
