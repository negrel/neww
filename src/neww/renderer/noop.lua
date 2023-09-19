local M = {}

function M.render(vnode, parentDom)
	print(string.format("rendering vnode <%s> to parentDom (%s)", vnode.type or "#error", parentDom))
end

return M
