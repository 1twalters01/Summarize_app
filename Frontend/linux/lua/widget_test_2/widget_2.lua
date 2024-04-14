local cjson = require "cjson"

function GenerateContent(input)
    local json
    local file = io.open("lua/widget_test_2/text.json", "r")
    if file~=nil then
        json = file:read("*a")
        file:close()
        local data = cjson.decode(json)
        local content = data.content
        return content.." "..input
    else
        local content = "Read error with Lua.";
        return content;
    end

end

