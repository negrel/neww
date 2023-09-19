local M = {}

-- For GTK4 Layer Shell to get linked before libwayland-client we must explicitly load it before importing with lgi
local ffi = require("ffi")
ffi.cdef [[
    void *dlopen(const char *filename, int flags);
]]
ffi.C.dlopen("libgtk4-layer-shell.so", 0x101)

local lgi = require("lgi")
local Gtk = lgi.require("Gtk")
local Gdk = lgi.require("Gdk")
local layer_shell = lgi.require("Gtk4LayerShell")

local renderer = require("neww-gtk.renderer")

-- Define renderer.
local neww = require("neww")
local luax = require("neww.luax")
neww.renderer = renderer

function M.create_app(app_props, window_props)
	local caller_app_on_activate = app_props.on_activate
	app_props.on_activate = nil

	-- Create application.
	local gtk_app = Gtk.Application(app_props)

	return function(vnode)
		gtk_app.on_activate = function()
			window_props.application = gtk_app
			local window = Gtk.Window(window_props)

			if type(caller_app_on_activate) == "function" then
				caller_app_on_activate(gtk_app, window)
			end

			window:show()
			neww:render(vnode, window)
		end


		gtk_app:run(arg) -- CLI args (global variable)
	end
end

return M
