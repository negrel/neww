local M = {}

local get_state = function(index)
	return M.__state[index]
end

local swap_state = function(index, state)
	local old_state = get_state(index)

	if type(old_state) == "table" and type(old_state.cleanup) == "function" then
		old_state.cleanup()
	end

	M.__state[index] = state

	return old_state
end

local hook = function(fn)
	return function(...)
		local index = M.__index
		M.__index = M.__index + 1
		return fn(index, ...)
	end
end

M.use_state = hook(function(index, initial_state)
	local state = get_state(index) or { value = initial_state }
	swap_state(index, state)

	return state.value, function(new_state)
		if type(new_state) == "function" then
			swap_state(index, { value = new_state(get_state(index).value) })
		else
			swap_state(index, { value = new_state })
		end
	end
end)

M.use_effect = hook(function(index, fn, deps)
	local old_state = get_state(index)

	local has_changed = true

	-- Check if deps has changed.
	if old_state and old_state.deps then
		has_changed = false
		for i, old_dep in ipairs(old_state.deps) do
			if old_dep ~= deps[i] then
				has_changed = true
				break
			end
		end
	end

	if has_changed then
		local state = { deps = deps }
		state.cleanup = fn()
		swap_state(index, state)
	end
end)

function M.reset_index()
	M.__index = 1
end

function M.clean_leftovers()
	for i = M.__index, #M.__state do
		swap_state(i, nil)
	end
end

function M.reset()
	M.reset_index()
	M.__state = {}
end

M.reset()

return M
