local M = {
	__app = nil,
}

local lgi = require("lgi")
local Gtk = lgi.require("Gtk")

-- Create a native component instance for the given VNode..
local create_instance = function(vnode)
	local instance = Gtk[vnode.type](vnode.props)

	for _, child in ipairs(vnode.props) do
		instance:append(child)
	end

	return instance
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

	-- Create native component.
	return create_instance(vnode)
end

function M.setup(app_props)
	M.__app = Gtk.Application(app_props)
end

function M.render(vnode, container)
	local tree = expand_vtree(vnode)

	container.child = tree
end

return M
