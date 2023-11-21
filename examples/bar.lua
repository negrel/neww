local neww = require("neww")
local hooks = neww.hooks
local neww = neww.enable_gtk()
local luax = require("neww.luax")

inspect = require("inspect")

-- Replace BAT1 with BAT0 if your computer starts with 0.
local battery_capacity_file = "/sys/class/power_supply/BAT1/capacity"

local switch_workspace = function(id)
	-- Edit to switch workspace
	os.execute("wmctl workspace focus " .. tostring(id))
end

-- Our application component.
function App()
	local datetime, set_datetime = hooks.use_state(os.date("%x %X"))
	local battery, set_battery = hooks.use_state("100%")

	hooks.use_effect(function()
		local timeout_id = neww.GLib.timeout_add_seconds(neww.GLib.PRIORITY_DEFAULT, 1,
			function()
				-- Datetime
				set_datetime(os.date("%x %X"))

				-- Battery
				local file = io.open(battery_capacity_file, "r")
				if file ~= nil then
					local percentage = file:read("*l")
					set_battery(percentage .. "%")
					file:close()
				end
				return true
			end)

		return function()
			neww.GLib.source_remove(timeout_id)
		end
	end, {})

	local left = luax.Box {
		homogeneous = true,
		orientation = "HORIZONTAL",
		halign = "START",
	}
	for i = 1, 10, 1 do
		table.insert(left.children, luax.Button {
			label = tostring(i),
			on_clicked = function()
				print("switching to workspace", i)
				switch_workspace(i)
			end
		})
	end

	return luax.Box {
		homogeneous = true,
		orientation = "HORIZONTAL",
		-- Left
		left,
		-- Center
		luax.Box {
			homogeneous = true,
			orientation = "HORIZONTAL",
			luax.Label {
				label = datetime
			},
		},
		-- Right
		luax.Box {
			homogeneous = true,
			orientation = "HORIZONTAL",
			halign = "END",
			luax.Label {
				label = battery
			}
		}
	}
end

local render = neww.create_app({
	application_id = 'dev.negrel.neww.example.bar',
	on_activate = function(_self, window)
		neww.layer_shell.init_for_window(window)
		neww.layer_shell.set_exclusive_zone(window, 5)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.LEFT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.RIGHT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.TOP, true)
	end
}, {
	title = "Bar",
	hexpand = true,
	vexpand = true
})

render(luax.App {})
