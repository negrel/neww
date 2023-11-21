local inspect = require("inspect")

local M = {
	__tests = {}
}

local real_print = print
local test_print = function(file, name)
	return function(...)
		real_print(file .. " " .. name .. ": ", ...)
	end
end

function M:test(name, test)
	local info = debug.getinfo(2, "S")
	local filename = info.short_src

	local test_file_table = self.__tests[filename] or {}
	table.insert(test_file_table, { name = name, test = test })

	self.__tests[filename] = test_file_table
end

function M:execute_suite()
	local passed = 0
	local failed = 0
	local test_suite_result = "ok"
	local start_time = os.clock()

	-- Run tests per source file.
	for test_file, tests in pairs(self.__tests) do
		print(string.format("running %d tests from %s", #tests, test_file))

		-- Run all tests from the same file
		for _, test in ipairs(tests) do
			local test_passed = self.__execute_test(test_file, test.name, test.test)
			if test_passed then
				passed = passed + 1
			else
				failed = failed + 1
				test_suite_result = "FAILED" -- one test failed
			end
		end
	end

	-- Sum up results.
	local end_time = os.clock()
	print(string.format("\n%s | %d passed | %d failed | %.2fms", test_suite_result, passed, failed,
		(end_time - start_time) * 1000))

	if failed > 0 then
		os.exit(1)
	end
end

function M.__execute_test(file, name, test)
	--luacheck: ignore setting read-only global variable print

	local start_time = os.clock()
	print = test_print(file, name) -- replace std print

	local success, error_msg = pcall(test)

	print = real_print -- restore std print

	local end_time = os.clock()

	-- Print result.
	if success then
		print(name .. " ... " .. "ok" .. string.format(" %.2fms", (end_time - start_time) * 1000))
	else
		print(name .. " ... " .. "FAILED" .. string.format(" %.2fms", (end_time - start_time) * 1000))
		print(debug.traceback(error_msg))
		print()
	end

	return success
end

function M.fail(msg)
	error(msg)
end

function M.assert(condition, msg)
	if not condition then M.fail(msg) end
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

function M.assert_eq(left, right, msg)
	if not deep_equal(left, right) then
		real_print("values are not equal")
		real_print("left  :", inspect(left))
		real_print("right :", inspect(right))
		M.fail(msg)
	end
end

return M
