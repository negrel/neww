local M = {}

function M.use_state(initial_state)
	local index = M.__index
	M.__index = M.__index + 1

	-- Set initial state if needed.
	M.__state[index] = M.__state[index] or initial_state

	return M.__state[index], function(new_state)
		M.__state[index] = new_state
	end
end

function M.use_effect(fn, deps)
	local index = M.__index
	M.__index = M.__index + 1

	-- Swap deps.
	local old_deps = M.__state[index]
	M.__state[index] = deps

	local has_changed = true

	-- Check if deps has changed.
	if old_deps then
		has_changed = false
		for i, old_dep in ipairs(old_deps) do
			if old_dep ~= deps[i] then
				has_changed = true
				break
			end
		end
	end

	if has_changed then fn() end
end

function M.reset_index()
	M.__index = 1
end

function M.reset()
	M.reset_index()
	M.__state = {}
end

M.reset()

return M
