local hooks = require("neww.hooks")

local M = {}

function M.render(vnode, parentDom)
	hooks.reset_index()
	print(string.format("rendering vnode <%s> to parentDom (%s)", vnode.type or "#error", parentDom))
	hooks.clean_leftovers()
end

return M
