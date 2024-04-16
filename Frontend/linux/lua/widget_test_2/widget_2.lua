local cjson = require("cjson")
local http = require("socket.http")
local https = require("ssl.https")
local ltn12 = require("ltn12")

function ReadJsonFromFile(input)
    local json
    local file = io.open("lua/widget_test_2/text.json", "r")
    if file ~=nil then
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

function GetJsonFromUrl(input)
    local url = "https://mdn.github.io/learning-area/javascript/oojs/json/superheroes.json"
    local response, status_code, headers, status_line = https.request(url)

    print(response)
    print(status_code)
    print(headers)
    print(status_line)

    -- should actually be if status_code is in the 200s
    if response ~= nil then
        local data = cjson.decode(response)
        local content = data.formed
        return content.." "..input
    else
        local content = "Read error with Lua.";
        return content;
    end
end

function PostJsonFromUrl(input)
    local url = "https://mdn.github.io/learning-area/javascript/oojs/json/superheroes.json"
    local data = {
        key1 = "value1",
        key2 = "value2",
    }
    local json_data = cjson.encode(data)
    local request_headers = {
        ["Content-Type"] = "application/json",
        ["Content-Length"] = tostring(#json_data)
    }

    local response, status_code, response_headers, status_line = http.request{
        url = url,
        method = "POST",
        headers = request_headers,
        source = ltn12.source.string(json_data),
    }

    print(response)
    print(status_code.."\n")
    for key, value in pairs(response_headers) do
        print(tostring(key)..": "..tostring(value))
    end
    print("\n"..status_line)

    -- should actually be if status_code in the 200s
    if response ~= nil then
        local content = "success".." "..input
        return content;
    else
        local content = "failure".." "..tostring(status_code)
        return content
    end
end

