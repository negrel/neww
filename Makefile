LUA ?= luajit

.PHONY: virtualenv
virtualenv: .venv

.venv:
	luarocks --tree=.venv install inspect

.PHONY: test
test:
	LUA_PATH="?/init.lua;?.lua;?;.venv/share/lua/5.1/?.lua" $(LUA) tests/init.lua

.PHONY: lint
lint:
	luacheck src tests

example/%:
	$(LUA) examples/$*/init.lua
