LUA ?= luajit

.PHONY: virtualenv
virtualenv: .venv

.venv:
	luarocks --tree=.venv install inspect

.PHONY: test
test:
	LUA_PATH="?/init.lua;?.lua;?;.venv/share/lua/5.1/?.lua" $(LUA) test/init.lua

example/%:
	$(LUA) examples/$*.lua
