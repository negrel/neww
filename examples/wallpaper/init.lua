local neww = require("neww")
neww = neww.enable_gtk()
local luax = require("neww.luax")
inspect = require("inspect")

function App()
	return luax.Box {
		id = "wallpaper"
	}
end

local render = neww.create_app({
	application_id = 'dev.negrel.neww.example.wallpaper',
	on_activate = function(self, window)
		neww.layer_shell.init_for_window(window)
		neww.layer_shell.set_layer(window, neww.layer_shell.Layer.BOTTOM)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.BOTTOM, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.LEFT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.RIGHT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.TOP, true)

		-- Load css.
		local provider = neww.Gtk.CssProvider()
		-- Relative to working directory from which this script is executed.
		provider:load_from_path("examples/wallpaper/style.css")
		local display = neww.Gdk.Display.get_default()
		neww.Gtk.StyleContext.add_provider_for_display(
			display, provider, 600 -- Priority
		)
	end
}, {
	title = "Wallpaper",
	hexpand = true,
	vexpand = true
})

render(luax.App {})
