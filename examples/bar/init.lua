local neww = require("neww.gtk")
local hooks = neww.hooks
local luax = require("neww.luax")

_G.inspect = require("inspect")

local display = neww.Gdk.Display.get_default()
local monitors = display:get_monitors()

local i = 0
while true do
	local monitor = neww.Gio.ListModel.get_item(monitors, i)
	if monitor == nil then break end
	print(i, monitor)
	i = i + 1
end

local switch_workspace = function(id)
	-- Edit to switch workspace.
	os.execute("wmctl workspace focus " .. tostring(id))
end

local battery_percentage = function()
	local battery_capacity_file = "/sys/class/power_supply/BAT1/capacity"

	local result = -1
	local file = io.open(battery_capacity_file, "r")
	if file ~= nil then
		result = file:read("*n")
		file:close()
	end

	return result
end

function Bar()
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

-- Create app.
luax.App {
	application_id = "dev.negrel.neww.example.bar",
	stacking = neww.stacking.OVERLAY,
	exclusive = true,
	anchors = { "top", "left", "right" },
	css_files = { "examples/bar/style.css" },
	window = {
		title = "Bar",
		hexpand = true,
		vexpand = true
	},
	children = luax.Bar {},
}
