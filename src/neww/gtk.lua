local M = {}

-- For GTK4 Layer Shell to get linked before libwayland-client we must explicitly load it before importing with lgi
local ffi = require("ffi")
ffi.cdef [[
		void *dlopen(const char *filename, int flags);
	]]
ffi.C.dlopen("libgtk4-layer-shell.so", 0x101)

local lgi = require("lgi")
M.layer_shell = lgi.require("Gtk4LayerShell")
M.Gtk = lgi.require("Gtk")
M.Gdk = lgi.require("Gdk")
M.GLib = lgi.require("GLib")
M.Gio = lgi.require("Gio")

function M.create_app(app_props, window_props)
	local caller_app_on_activate = app_props.on_activate
	app_props.on_activate = nil

	-- Create application.
	local gtk_app = M.Gtk.Application(app_props)
	gtk_app.resource_base_path = os.getenv("NEWW_RESOURCE_PATH") or os.getenv("PWD")

	return function(vnode)
		gtk_app.on_activate = function()
			window_props.application = gtk_app
			local window = M.Gtk.Window(window_props)

			if type(caller_app_on_activate) == "function" then
				caller_app_on_activate(gtk_app, window)
			end

			local box = M.Gtk.Box {
				homogeneous = true,
				overflow = "HIDDEN"
			}

			window.child = box
			window:show()

			require("neww"):render(vnode, box)
		end


		gtk_app:run(arg) -- CLI args (global variable)
	end
end

return M
