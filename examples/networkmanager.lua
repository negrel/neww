local neww = require("neww")
local hooks = neww.hooks
neww = neww.enable_gtk()
local luax = require("neww.luax")

inspect = require("inspect")

local switch_workspace = function(id)
	-- Edit to switch workspace
	os.execute("wmctl workspace focus " .. tostring(id))
end

local battery_percentage = function()
	-- Replace BAT1 with BAT0 if your computer starts with 0.
	local battery_capacity_file = "/sys/class/power_supply/BAT1/capacity"

	local result = -1
	local file = io.open(battery_capacity_file, "r")
	if file ~= nil then
		result = file:read("*n")
		file:close()
	end

	return result
end

-- Our application component.
function App()
	return luax.Box {
		homogeneous = true,
		orientation = "HORIZONTAL",
		overflow = "HIDDEN",
		-- Left
		luax.Left {},
		-- Center
		luax.Center {},
		-- Right
		luax.Right {}
	}
end

-- Left module
function Left()
	local left = luax.Box {
		css_classes = { "left" },
		orientation = "HORIZONTAL",
		halign = "START",
	}
	for i = 1, 10, 1 do
		-- Note: unlike luax, we're inserting button in children property.
		table.insert(left.children, luax.Button {
			label = tostring(i),
			width = 8,
			on_clicked = function()
				print("switching to workspace", i)
				switch_workspace(i)
			end
		})
	end

	return left
end

-- Center
function Center()
	local datetime, set_datetime = hooks.use_state(os.date("%x %X"))

	hooks.use_effect(function()
		local timeout_id = neww.GLib.timeout_add_seconds(neww.GLib.PRIORITY_DEFAULT, 1,
			function()
				-- Datetime
				set_datetime(os.date("%x %X"))

				return true
			end)

		return function()
			neww.GLib.source_remove(timeout_id)
		end
	end, {})

	return luax.Box {
		css_classes = { "center" },
		homogeneous = true,
		orientation = "HORIZONTAL",
		luax.Label {
			label = datetime
		},
	}
end

-- Right
function Right()
	local battery, set_battery = hooks.use_state(battery_percentage())

	hooks.use_effect(function()
		local timeout_id = neww.GLib.timeout_add_seconds(neww.GLib.PRIORITY_DEFAULT, 30,
			function()
				set_battery(battery_percentage())
				return true
			end)

		return function()
			neww.GLib.source_remove(timeout_id)
		end
	end, {})

	local icons = { "", "", "", "", "" }
	local icon = icons[5]
	if battery <= 90 then
		icon = icons[4]
	elseif battery <= 70 then
		icon = icons[3]
	elseif battery <= 50 then
		icon = icons[2]
	elseif battery <= 20 then
		icon = icons[1]
	end

	return luax.Box {
		css_classes = { "right" },
		homogeneous = true,
		orientation = "HORIZONTAL",
		halign = "END",
		luax.Label {
			class = "battery",
			label = tostring(battery) .. "%"
		},
		luax.Label {
			css_classes = { "text-icon", "battery-icon" },
			label = icon
		}
	}
end

local render = neww.create_app({
	application_id = 'dev.negrel.neww.example.bar',
	on_activate = function(_self, window)
		neww.layer_shell.init_for_window(window)
		neww.layer_shell.auto_exclusive_zone_enable(window)
		neww.layer_shell.set_layer(window, neww.layer_shell.Layer.OVERLAY)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.LEFT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.RIGHT, true)
		neww.layer_shell.set_anchor(window, neww.layer_shell.Edge.TOP, true)

		-- Load css.
		local provider = neww.Gtk.CssProvider()
		-- Relative to working directory from which this script is executed.
		provider:load_from_path("examples/bar.css")
		local display = neww.Gdk.Display.get_default()
		neww.Gtk.StyleContext.add_provider_for_display(
			display, provider, 600 -- Priority
		)
	end
}, {
	title = "Bar",
	hexpand = true,
	vexpand = true
})

render(luax.App {})
