use std::fs;

use anyhow::Context;
use env_logger::Env;
use lib::{parse_neww, Neww};
use mlua::{chunk, Lua};

pub fn main() -> Result<(), anyhow::Error> {
    // Initialize logger.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Retrieve .neww file from args.
    let files = &std::env::args().collect::<Vec<_>>();
    let fpath = files.get(1).context("UI file path is missing")?;

    // Read and parse .neww file.
    log::debug!("reading {fpath:?} file");
    let file_content = fs::read_to_string(fpath).context("failed to read file")?;
    log::debug!("parsing neww file");
    let neww: Neww = parse_neww(&file_content).context("failed to parse neww file")?;

    // Convert interface section of .neww file into GTK UI.
    log::debug!("converting neww UI to GTK UI XML");
    let gtk_ui = quick_xml::se::to_string(&neww.interface)
        .context("failed to convert neww UI to GTK UI XML")?;
    log::debug!("GTK UI: {gtk_ui}");

    let vm = unsafe { Lua::unsafe_new().into_static() };
    vm.load(chunk! {
        lgi = require("lgi")
        GLib = lgi.require("GLib")
        Gtk = lgi.require("Gtk")
        Builder = Gtk.Builder.new_from_string($gtk_ui, $gtk_ui:len())
    })
    .exec()
    .context("failed to initialize lua VM, please report a bug")?;

    log::debug!("loading lua scripts");

    vm.load(chunk! {
        local main = Builder:get_object("main")
        if main == nil then error("no window with id 'main' defined") end

        // Create the main event loop.
        local main_loop = GLib.MainLoop()

        // Stop main loop on close request.
        main.on_close_request = function()
            main_loop:quit()
        end
        // Show main window.
        main:present()

        // Start the main event loop.
        main_loop:run()
    })
    .exec()
    .context("failed to initialize lua VM, please report a bug")?;

    Ok(())
}
