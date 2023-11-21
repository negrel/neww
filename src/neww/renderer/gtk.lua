local hooks = require("neww.hooks")

local M = {
	__app = nil,
}

local lgi = require("lgi")
local Gtk = lgi.require("Gtk")
local GLib = lgi.require("GLib")
local GObject = lgi.require("GObject")

-- Signals handlers lookup table.
local signal_handlers_id = {}

local connect_signal = function(instance, instance_key, signal_name, handler)
	local signal_id = instance[signal_name]:connect(handler)
	if signal_handlers_id[instance_key] == nil then
		signal_handlers_id[instance_key] = { [signal_name] = signal_id }
	else
		signal_handlers_id[instance_key][signal_name] = signal_id
	end
end

local disconnect_signal = function(instance, instance_key, signal_name)
	if signal_handlers_id[instance_key] ~= nil and signal_handlers_id[instance_key][signal_name] ~= nil then
		GObject.signal_handler_disconnect(instance, signal_handlers_id[instance_key][signal_name])
	end
end

-- VNode lookup table.
local instances_vnode = {}

-- Create a native component instance for the given VNode recursively.
local create_instance
create_instance = function(vnode)
	local instance = Gtk[vnode.type](vnode.props)
	local instance_key = tostring(instance)

	for _, child in ipairs(vnode.children) do
		instance:append(create_instance(child))
	end

	for signal_name, handler in pairs(vnode.handlers) do
		connect_signal(instance, instance_key, signal_name, handler)
	end

	instances_vnode[instance_key] = vnode

	return instance
end

local destroy_instance
destroy_instance = function(instance)
	local instance_key = tostring(instance)

	-- Destroy children.
	local node_child = Gtk.Widget.get_first_child(instance)
	while true do
		if node_child == nil then break end
		destroy_instance(node_child)
		node_child = Gtk.Widget.get_next_sibling(node_child)
	end

	signal_handlers_id[instance_key] = nil
	instances_vnode[instance_key] = nil
end

-- Recursively expand functional component.
local expand_vtree
expand_vtree = function(vnode)
	-- Execute functional component.
	while type(vnode.type) == "function" do
		vnode = vnode.type(vnode.props)
	end

	-- Render children
	for i, child in ipairs(vnode.props) do
		vnode.props[i] = expand_vtree(child)
	end
	-- Single child widgets.
	if vnode.props.child then
		vnode.props.child = expand_vtree(vnode.props.child)
	end

	return vnode
end

function M.setup(app_props)
	M.__app = Gtk.Application(app_props)
end

local render_queued = false

function M.render(vnode, container)
	if render_queued then return end

	render_queued = true
	GLib.idle_add(GLib.PRIORITY_DEFAULT, function()
		hooks.reset_index()
		local vtree = expand_vtree(vnode)

		local container_child = Gtk.Widget.get_first_child(container)

		-- Render entire subtree.
		if container_child == nil then
			local instance = create_instance(vtree)
			container:append(instance)
		else
			M.reconciliate(container_child, vtree)
		end

		hooks.clean_leftovers()
		render_queued = false
		return false
	end)
end

-- Function to check deep equality between two tables.
local deep_equal
deep_equal = function(value1, value2)
	local valueType = type(value1)

	-- Check if the tables have the same type.
	if valueType ~= type(value2) then
		return false
	end

	-- Not tables.
	if valueType ~= "table" then
		return value1 == value2
	end

	-- Check if the tables have the same length.
	if #value1 ~= #value2 then
		return false
	end

	-- Check the equality of each element in the tables.
	for key, inner_value1 in pairs(value1) do
		local inner_value2 = value2[key]
		if not deep_equal(inner_value1, inner_value2) then
			return false
		end
	end

	return true
end

function M.reconciliate(instance, vnode)
	local instance_key = tostring(instance)
	local instance_vnode = instances_vnode[instance_key]

	local differentWidgetType = instance._name ~= "Gtk." .. vnode.type
	local propsDiffer = not deep_equal(instance_vnode.props, vnode.props) or
			not deep_equal(instance_vnode.handlers, vnode.handlers)

	if differentWidgetType or propsDiffer then
		-- Recreate instance.
		local parent = Gtk.Widget.get_parent(instance)
		Gtk.Widget.insert_before(
			create_instance(vnode),
			parent,
			Gtk.Widget.get_next_sibling(instance)
		)
		parent:remove(instance)
		destroy_instance(instance)
		return
	end

	-- Reconciliate vnode and instance.

	-- Sync children
	local node_child = Gtk.Widget.get_first_child(instance)
	for _, childVnode in pairs(vnode.children) do
		-- new child element
		if node_child == nil then
			instance:append(create_instance(childVnode))
		else
			local next_node_child = Gtk.Widget.get_next_sibling(node_child)
			M.reconciliate(node_child, childVnode)
			node_child = next_node_child
		end
	end

	-- Sync signals
	for signal_name, handler in pairs(vnode.handlers) do
		disconnect_signal(instance, instance_key, signal_name)
		connect_signal(instance, instance_key, signal_name, handler)
	end

	-- Sync remaining props
	for key, value in pairs(vnode.props) do
		if instance[key] ~= value then
			instance[key] = value
		end
	end

	-- Update vnode lookup table.
	instances_vnode[instance_key] = vnode
end

return M
