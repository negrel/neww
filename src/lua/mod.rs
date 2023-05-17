use anyhow::Context;
use mlua::Lua;

use self::{application::Application, bindings::gtk::Window};

// Global application user data.
pub mod application;

// Bindings
pub mod bindings;

pub fn new_vm(app: Application) -> Result<Lua, anyhow::Error> {
    let vm = Lua::new();

    vm.globals()
        .set("application", app)
        .context("Failed to define global application table")?;

    load_neww_ui_module(&vm).context("Failed to load neww.ui module")?;

    Ok(vm)
}

fn load_neww_ui_module(vm: &Lua) -> Result<(), anyhow::Error> {
    let module = vm.create_function(|vm, ()| {
        let table = vm.create_table()?;

        // Window constructor
        table.set(
            "window",
            vm.create_function(|_vm, ()| Ok(Window(gtk::Window::new())))?,
        )?;

        Ok(table)
    })?;

    vm.load_from_function::<_, mlua::Table>("neww.ui", module)?;

    Ok(())
}
