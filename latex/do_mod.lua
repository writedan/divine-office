-- debug_module.lua
local function try_load()
    local status, result = pcall(function()
        return require("divine_office")
    end)
    
    if status then
        print("SUCCESS: Module loaded!")
        return result
    else
        print("FAILURE: " .. tostring(result))
        
        -- Try to get OS error information
        local dll_path = "./divine_office.dll"
        local f = io.open(dll_path, "rb")
        if not f then
            print("File cannot be opened: " .. dll_path)
        else
            print("File exists and can be opened")
            local header = f:read(4)
            f:close()
            
            -- Check MZ header for valid PE file
            if header:byte(1) == 77 and header:byte(2) == 90 then
                print("File has valid MZ header (likely a PE file)")
            else
                print("File does NOT have valid PE header: " .. 
                      string.format("%02X %02X %02X %02X", 
                                   header:byte(1), header:byte(2), 
                                   header:byte(3), header:byte(4)))
            end
        end
    end
end

print("Lua version: " .. _VERSION)
print("Architecture: " .. (jit and jit.arch or (_SIZEOF_LONG == 8 and "64-bit" or "32-bit")))
print("Package path: " .. package.path)
print("Package cpath: " .. package.cpath)
print("Current directory: " .. (io.popen("cd"):read("*l") or "unknown"))

try_load()