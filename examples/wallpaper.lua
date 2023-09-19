-- For GTK4 Layer Shell to get linked before libwayland-client we must explicitly load it before importing with lgi
local ffi = require("ffi")
ffi.cdef [[
    void *dlopen(const char *filename, int flags);
]]
ffi.C.dlopen("libgtk4-layer-shell.so", 0x101)

local lgi = require("lgi")
local layer_shell = lgi.require("Gtk4LayerShell")

local neww_gtk = require("neww-gtk")
local neww = require("neww")
local luax = require("neww.luax")

function App()
	return luax.Image {
		file = "examples/image.jpg"
	}
end

local render = neww_gtk.create_app({
	application_id = 'dev.negrel.neww.example.wallpaper',
	on_activate = function(self, window)
		layer_shell.init_for_window(window)
		layer_shell.set_layer(window, layer_shell.Layer.BOTTOM)
		layer_shell.set_anchor(window, layer_shell.Edge.BOTTOM, true)
		layer_shell.set_anchor(window, layer_shell.Edge.LEFT, true)
		layer_shell.set_anchor(window, layer_shell.Edge.RIGHT, true)
		layer_shell.set_anchor(window, layer_shell.Edge.TOP, true)
	end
}, {
	title = "Wallpaper",
	hexpand = true,
	vexpand = true
})

render(luax.App {})
