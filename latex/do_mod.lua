local mod = require("divine_office")
local processor = {}

function processor.resolvePath(path)
	return string.format("%s/%s", "../backend", path)
end

function processor.compilePath(path)
	local path = processor.resolvePath(path)
	return path
end

return processor