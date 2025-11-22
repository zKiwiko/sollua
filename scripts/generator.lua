#!/usr/bin/env lua
-- GitHub Copilot
-- Generates many example Lua functions into a file.

local NUM = 1000
local OUT = "./generated.lua"
local PREFIX = "gen_func"
local START = 1

local function usage()
    io.write(string.format([[
Usage: %s [-n number] [-o outfile] [-p prefix] [-s start]
        -n number   Number of functions to generate (default: %d)
        -o outfile  Output file (default: %s)
        -p prefix   Function name prefix (default: %s)
        -s start    Start index (default: %d)
]], arg[0] or "generator.lua", NUM, OUT, PREFIX, START))
end

-- simple arg parsing
local i = 1
while i <= #arg do
    local a = arg[i]
    if a == "-n" then
        if not arg[i + 1] then
            io.stderr:write("Missing value for -n\n"); usage(); os.exit(1)
        end
        NUM = tonumber(arg[i + 1]) or (io.stderr:write("Invalid value for -n\n")); usage(); os.exit(1)
        i = i + 2
    elseif a == "-o" then
        if not arg[i + 1] then
            io.stderr:write("Missing value for -o\n"); usage(); os.exit(1)
        end
        OUT = arg[i + 1]
        i = i + 2
    elseif a == "-p" then
        if not arg[i + 1] then
            io.stderr:write("Missing value for -p\n"); usage(); os.exit(1)
        end
        PREFIX = arg[i + 1]
        i = i + 2
    elseif a == "-s" then
        if not arg[i + 1] then
            io.stderr:write("Missing value for -s\n"); usage(); os.exit(1)
        end
        START = tonumber(arg[i + 1]) or (io.stderr:write("Invalid value for -s\n")); usage(); os.exit(1)
        i = i + 2
    elseif a == "-h" or a == "--help" then
        usage(); os.exit(0)
    else
        io.stderr:write("Invalid option: ", a, "\n")
        usage()
        os.exit(1)
    end
end

-- validate numeric args
if type(NUM) ~= "number" or type(START) ~= "number" or NUM < 1 or START < 1 or NUM ~= math.floor(NUM) or START ~= math.floor(START) then
    io.stderr:write("Error: -n and -s must be positive integers\n")
    os.exit(1)
end

local f, err = io.open(OUT, "w")
if not f then
    io.stderr:write("Error opening output file: ", tostring(err), "\n")
    os.exit(1)
end

f:write(string.format('-- Generated Lua functions (%s)\n\n', os.date("!%Y-%m-%dT%H:%M:%SZ")))

local END = START + NUM - 1
for idx = START, END do
    local name = string.format("%s_%d", PREFIX, idx)
    local mod = idx % 3

    f:write(string.format('function %s(a, b)\n', name))

    if mod == 0 then
        f:write('  -- returns a scaled by index plus b (fallbacks to 0)\n')
        f:write('  local A = (a or 0)\n')
        f:write('  local B = (b or 0)\n')
        f:write(string.format('  return A * %d + B\n', idx))
    elseif mod == 1 then
        local k = (idx % 5) + 1
        f:write(string.format('  -- accumulates 1..%d and adds a\n', k))
        f:write('  local sum = 0\n')
        f:write(string.format('  for j = 1, %d do\n', k))
        f:write(string.format('    sum = sum + j * %d\n', idx))
        f:write('  end\n')
        f:write('  return sum + (a or 0)\n')
    else -- mod == 2
        f:write('  -- returns a table with computed fields\n')
        f:write('  local A = (a or 0)\n')
        f:write(string.format('  return { idx = %d, val = A + %d, ok = (b ~= nil) }\n', idx, idx % 10))
    end

    f:write('end\n\n')
end

f:close()

io.stderr:write(string.format('Wrote %d functions to %s\n', NUM, OUT))
