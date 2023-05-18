use anyhow::Context;
use mlua::{chunk, Lua, UserData};

use self::{
    application::Application,
    bindings::gtk::{Button, Label, Window},
};

// Global application user data.
pub mod application;

// Bindings
pub mod bindings;

pub fn new_vm(app: Application) -> Result<Lua, anyhow::Error> {
    let vm = Lua::new();

    vm.globals()
        .set("application", app)
        .context("Failed to define global application table")?;

    load_neww_ui_components_module(&vm).context("Failed to load neww.ui.components module")?;
    load_neww_ui_module(&vm).context("Failed to load neww.ui module")?;

    Ok(vm)
}

pub trait UIComponent: Default + UserData {}

impl<T: Default + UIComponent> UIComponent for T {}

fn load_neww_ui_components_module(vm: &Lua) -> Result<(), anyhow::Error> {
    let module = vm.create_function(|vm, ()| {
        let table = vm.create_table()?;

        macro_rules! component {
            ($component_name:ident) => {
                table.set(
                    stringify!($component_name).to_lowercase(),
                    vm.create_function(|_vm, ()| {
                        Ok($component_name(gtk::$component_name::default()))
                    })?,
                )?;
            };
        }

        component!(Window);
        component!(Label);
        component!(Button);

        Ok(table)
    })?;

    vm.load_from_function::<_, mlua::Table>("neww.ui.components", module)?;

    Ok(())
}

fn load_neww_ui_module(vm: &Lua) -> Result<(), anyhow::Error> {
    let module = chunk! {
        local components = require("neww.ui.components")
        local M = {}
        setmetatable(M, {
            __index = function(table, key)
                return function(attributes)
                    local component = components[key]()
                    // Set attributes
                    for name, value in pairs(attributes) do
                        component[name] = value
                    end
                    return component
                end
            end
        })
        return M
    };
    let module = vm.load(module).into_function()?;

    vm.load_from_function::<_, mlua::Table>("neww.ui", module)?;

    Ok(())
}
