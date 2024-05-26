local package = require("neww")
package.renderer = require("neww.renderer.gtk")

local M = {}
setmetatable(M, {
	---@diagnostic disable-next-line: unused-local
	__index = function(t, k)
		return package[k] -- fallback to package level value.
	end
})

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

M.stacking = {
	BACKGROUND = M.layer_shell.Layer.BACKGROUND,
	BOTTOM = M.layer_shell.Layer.BOTTOM,
	TOP = M.layer_shell.Layer.TOP,
	OVERLAY = M.layer_shell.Layer.OVERLAY,
}

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

_G.App = function(props)
	local render = M.create_app({
		application_id = props.application_id,
		---@diagnostic disable-next-line: unused-local
		on_activate = function(self, window)
			-- GTK layer shell
			if props.stacking ~= nil or props.exclusive == true or props.anchor then
				M.layer_shell.init_for_window(window)
				if props.exclusive == true then
					M.layer_shell.auto_exclusive_zone_enable(window)
				end
				M.layer_shell.set_layer(window, props.stacking)

				local anchor2LayerShellEdge = {
					top = M.layer_shell.Edge.TOP,
					right = M.layer_shell.Edge.RIGHT,
					bottom = M.layer_shell.Edge.BOTTOM,
					left = M.layer_shell.Edge.LEFT,
				}
				for _, anchor in ipairs(props.anchors) do
					M.layer_shell.set_anchor(window, anchor2LayerShellEdge[anchor], true)
				end
			end

			-- CSS files.
			if type(props.css_files) == "table" then
				-- Load css.
				local provider = M.Gtk.CssProvider()
				for _, css_fpath in ipairs(props.css_files) do
					provider:load_from_path(css_fpath)
				end
				local display = M.Gdk.Display.get_default()
				M.Gtk.StyleContext.add_provider_for_display(
					display, provider, 600 -- Priority
				)
			end

			if type(props.on_activate) == "function" then
				props.on_activate()
			end
		end
	}, props.window)

	render(props.children)
end

return M
