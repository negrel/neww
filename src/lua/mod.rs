use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use mlua::{chunk, Lua, UserData};

use self::bindings::gtk::{Box, Button, Image, Label, Scale, Window};

// Bindings
pub mod bindings;

pub fn new_vm(app: impl UserData + 'static) -> Result<&'static Lua, anyhow::Error> {
    // Use a static lua VM as it is used to handle signals.
    let vm = Lua::new().into_static();

    vm.globals()
        .set("application", app)
        .context("Failed to define global application table")?;

    load_neww_ui_components_module(vm).context("Failed to load neww.ui.components module")?;
    load_neww_ui_module(vm).context("Failed to load neww.ui module")?;
    load_neww_timer_module(vm).context("Failed to load neww.timer module")?;

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
        component!(Image);
        component!(Box);
        component!(Scale);

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

fn load_neww_timer_module(vm: &'static Lua) -> Result<(), anyhow::Error> {
    let module = vm.create_function(|vm, ()| {
        let table = vm.create_table()?;

        let timers = Rc::new(RefCell::new(Vec::new()));

        {
            let timers = timers.clone();
            table.set(
                "set_interval",
                vm.create_function(move |_vm, (callback, ms): (mlua::Function, u64)| {
                    let source_id = gtk::glib::timeout_add_local(
                        std::time::Duration::from_millis(ms),
                        move || {
                            let _ = callback.call::<_, ()>(());
                            gtk::glib::Continue(true)
                        },
                    );

                    timers.borrow_mut().push(source_id);

                    Ok(timers.borrow().len() - 1)
                })?,
            )?;
        }

        {
            let timers = timers.clone();
            table.set(
                "set_timeout",
                vm.create_function(move |_vm, (callback, ms): (mlua::Function, u64)| {
                    let source_id = gtk::glib::timeout_add_local(
                        std::time::Duration::from_millis(ms),
                        move || {
                            let _ = callback.call::<_, ()>(());
                            gtk::glib::Continue(false)
                        },
                    );

                    timers.borrow_mut().push(source_id);

                    Ok(timers.borrow().len() - 1)
                })?,
            )?;
        }

        {
            table.set(
                "clear_timer",
                vm.create_function(move |_vm, index: mlua::Integer| {
                    let index: usize =
                        index
                            .try_into()
                            .map_err(|_err| mlua::Error::FromLuaConversionError {
                                from: "integer",
                                to: "usize",
                                message: None,
                            })?;
                    let source_id: gtk::glib::SourceId = timers.borrow_mut().remove(index);
                    source_id.remove();
                    Ok(())
                })?,
            )?;
        }

        Ok(table)
    })?;

    vm.load_from_function::<_, mlua::Table>("neww.timer", module)?;

    Ok(())
}
