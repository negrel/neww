local M = {}

local neww = require("neww")

setmetatable(M, {
	__index = function(_t, k)
		-- User defined component.
		if type(_G[k]) == "function" then
			return function(props)
				return neww.create_element(_G[k], props or {})
			end
		end

		-- Native components.
		return function(props)
			return neww.create_element(k, props or {})
		end
	end
})

return M
