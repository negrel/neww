local lgi = require("lgi")
local Gtk = lgi.require("Gtk")

local neww = require("neww")
local hooks = neww.hooks

local luax = require("neww.luax")

neww.renderer = require("neww.renderer.gtk")

-- Our application component.
function App(props)
	local counter, set_counter = hooks.use_state(props.initial_count or 0)

	return luax.Box {
		homogeneous = true,
		orientation = Gtk.Orientation.HORIZONTAL,
		luax.Button {
			label = "-",
			on_clicked = function()
				set_counter(counter - 1)
			end
		},
		luax.Label {
			label = tostring(counter)
		},
		luax.Button {
			label = "+",
			on_clicked = function()
				set_counter(counter + 1)
			end
		}
	}
end

-- Create and start the application.
local gtk_app = Gtk.Application { application_id = 'dev.negrel.neww.example.counter' }

function gtk_app:on_activate()
	local window = Gtk.Window {
		application = self,
		title = 'Counter',
		default_width = 640,
		default_height = 480,
	}
	window:show()

	neww:render(luax.App { initial_count = 3 }, window)
end

gtk_app:run { arg[0], ... }
