inspect = require("inspect")
Testx = require("src.testx")


Testx:test("multiple call to use_state hook creates different state", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local counter, set_counter = hooks.use_state(1)

		set_counter(counter + 1)

		return counter
	end

	local first_render = {
		a = Component(),
		b = Component(),
	}

	if first_render.a ~= first_render.b then
		error("use_state didn't create multiple state")
	end

	-- Reset internal hook index.
	hooks.reset_index()
	-- Now this is another render.

	local second_render = {
		a = Component(),
		b = Component()
	}

	if second_render.a ~= first_render.a + 1 then
		error("second render produce new state for component a")
	end

	if second_render.b ~= first_render.b + 1 then
		error("second render produce new state for component b")
	end
end)

Testx:test("use_effect callback is called on first render (no dependency)", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local use_effect_called = false

		hooks.use_effect(function()
			use_effect_called = true
		end)

		return use_effect_called
	end

	local first_render = {
		a = Component(),
		b = Component()
	}

	if not first_render.a or not first_render.b then
		error("use_effect callback wasn't called for component A or/and B")
	end
end)

Testx:test("use_effect callback is called on first render", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local counter, _set_counter = hooks.use_state(1)
		local use_effect_called = false

		hooks.use_effect(function()
			use_effect_called = true
		end, { 0, counter })

		return use_effect_called
	end

	local first_render = {
		a = Component(),
		b = Component()
	}

	if not first_render.a or not first_render.b then
		error("use_effect callback wasn't called for component A or/and B")
	end
end)

Testx:test("use_effect callback is called on second render (no dependency)", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local use_effect_called = false

		hooks.use_effect(function()
			use_effect_called = true
		end)

		return use_effect_called
	end

	-- First render.
	Component()
	Component()

	-- Reset internal hook index.
	hooks.reset_index()
	-- Now this is another render.

	local second_render = {
		a = Component(),
		b = Component()
	}

	if not second_render.a or not second_render.b then
		error("use_effect callback wasn't called for component A or/and B")
	end
end)


Testx:test("use_effect callback is called on second render if a dependency changed", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local counter, set_counter = hooks.use_state(0)
		local use_effect_called = false

		hooks.use_effect(function()
			use_effect_called = true
		end, { 0, counter, 1 })

		-- Update counter so a dependency of use_effect changed.
		set_counter(counter + 1)

		return use_effect_called
	end

	-- First render.
	Component()
	Component()

	-- Reset internal hook index.
	hooks.reset_index()
	-- Now this is another render.

	local second_render = {
		a = Component(),
		b = Component()
	}

	if not second_render.a or not second_render.b then
		error("use_effect callback wasn't called for component A or/and B")
	end
end)

Testx:test("use_effect callback is NOT called on second render if NO dependency changed", function()
	local hooks = require("src.neww.__hooks")
	hooks.reset()

	local Component = function()
		local counter, _set_counter = hooks.use_state(0)
		local use_effect_called = false

		hooks.use_effect(function()
			use_effect_called = true
		end, { 0, counter, 1 })

		return use_effect_called
	end

	-- First render.
	Component()
	Component()

	-- Reset internal hook index.
	hooks.reset_index()
	-- Now this is another render.

	local second_render = {
		a = Component(),
		b = Component()
	}

	if second_render.a or second_render.b then
		error("use_effect callback was called for component A or/and B")
	end
end)

Testx:execute_suite()
