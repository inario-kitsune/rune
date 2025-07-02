--[[
rune-meta
name: Lua Plugin
ext:
  - lua
]]
-- A basic Lua runner
local f = assert(loadfile(target))
return f(unpack(args))
