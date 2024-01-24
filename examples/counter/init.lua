local neww = require("neww")
local hooks = neww.hooks
neww = neww.enable_gtk()

local luax = require("neww.luax")

-- Our application component.
function App(props)
	local counter, set_counter = hooks.use_state(props.initial_count or 0)

	return luax.Box {
		homogeneous = true,
		orientation = "HORIZONTAL",
		luax.Button {
			label = "-3",
			on_clicked = function()
				set_counter(counter - 3)
			end
		},
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
		},
		luax.Button {
			label = "+3",
			on_clicked = function()
				set_counter(counter + 3)
			end
		}
	}
end

-- Create and start the application.
local render = neww.create_app({
	application_id = 'dev.negrel.neww.example.counter',
}, {
	title = 'Counter',
	default_width = 640,
	default_height = 480,
})

render(luax.App { initial_count = 3 })
