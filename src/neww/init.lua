local hooks = require("neww.__hooks")

local M = {
	renderer = require("neww.renderer.noop"),
	hooks = {},
	full_render = nil,
}

function M.create_element(eltype, props)
	local handlers = {}
	local children = {}
	local normalizedProps = {}
	local key = nil
	local ref = nil

	for pkey, pvalue in pairs(props) do
		if pkey == "key" then
			key = pvalue
		elseif pkey == "ref" then
			ref = pvalue
		elseif type(pkey) == "number" then
			table.insert(children, pvalue)
		elseif type(pvalue) == "function" then
			handlers[pkey] = pvalue
		else
			normalizedProps[pkey] = pvalue
		end
	end

	return M.create_vnode(eltype, normalizedProps, handlers, children, key, ref)
end

function M.create_vnode(type, props, handlers, children, key, ref)
	local vnode = {
		type = type,
		props = props,
		handlers = handlers,
		children = children,
		key = key,
		ref = ref,
	};

	return vnode
end

function M.hooks.use_state(...)
	local state, state_setter = hooks.use_state(...)
	return state, function(...)
		state_setter(...)
		M.full_render()
	end
end

function M.render(self, vnode, container)
	hooks.reset_index()

	-- Set full render.
	if M.full_render == nil then
		M.full_render = function()
			M.render(self, vnode, container)
		end
	end

	self.renderer.render(vnode, container)
end

return M
