local M = {}

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
	if type(vnode.type) == "function" then
		vnode = vnode.type(vnode.props)
	end

	-- Render children
	for i, child in ipairs(vnode.props) do
		vnode.props[i] = expand_vtree(child)
	end

	-- Create native component.
	return create_instance(vnode)
end

function M.render(vnode, container)
	container.child = expand_vtree(vnode)
	container:show()
end

return M
