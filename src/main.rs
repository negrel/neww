use gtk::prelude::*;
use mlua::prelude::*;

/// Application state.
struct State {
    lua: Lua,
}

impl State {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }
}

fn activate(app: &gtk::Application, state: &'static State) {
    println!("activated");
}

fn main() {
    let state = Box::leak(Box::new(State::new()));
    let app = gtk::Application::builder()
        .application_id("dev.negrel.neww")
        .build();

    app.connect_activate(|app| activate(app, state));

    app.run();
}
