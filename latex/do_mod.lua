-- debug_module.lua
function try_load()
    local status, result = pcall(function()
        return require("divine_office")
    end)
    
    if status then
        print("SUCCESS: Module loaded!")
        return result
    else
        error("FAILURE: " .. tostring(result))
        
        -- Try to get OS error information
        local dll_path = "./divine_office.dll"
        local f = io.open(dll_path, "rb")
        if not f then
            error("File cannot be opened: " .. dll_path)
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

function string:endswith(suffix)
    return self:sub(-#suffix) == suffix
end

mod = require("divine_office")
cjson = require("cjson")

function process(elements)
    return "\\textbf{Unable to parse" .. elements .. "}"
end

function domod(file_path)
    if file_path:endswith(".gabc") then
        return "\\textbf{Cannot support GABC files}"
    elseif file_path:endswith(".lit") then
        return "" .. mod
        --return "\\textbf{Cannot support LIT files}"
    else
        return "\\textbf{Cannot handle " .. file_path .. "}"
    end
end

return {
    domod = domod
}