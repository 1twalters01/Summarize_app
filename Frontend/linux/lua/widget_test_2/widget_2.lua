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
    -- local url = "https://mdn.github.io/learning-area/javascript/oojs/json/superheroes.json"
    local url = "http://127.0.0.1:8000/ping/any_auth"
    local response, status_code, headers, status_line = http.request(url)

    print(response)
    print(status_code)
    for key, value in pairs(headers) do
        print(tostring(key)..": "..tostring(value))
    end
    print(status_line)

    -- should actually be if status_code is in the 200s
    if response ~= nil then
        local data = cjson.decode(response)
        local content = data.message
        return content.." "..input
    else
        local content = "Read error with Lua.";
        return content;
    end
end

function PostJsonFromUrl(input)
    -- local url = "https://mdn.github.io/learning-area/javascript/oojs/json/superheroes.json"
    local url = "http://127.0.0.1:8000/ping/any_auth"

    local data = {
        message = "ping from linux desktop"
    }
    local json_data = cjson.encode(data)

    local request_headers = {
        ["Content-Type"] = "application/json",
        ["Content-Length"] = tostring(#json_data)
    }
    local response_body = {}

    local response, status_code, response_headers, status_line = http.request{
        url = url,
        method = "POST",
        headers = request_headers,
        source = ltn12.source.string(json_data),
        sink = ltn12.sink.table(response_body),
    }

    print(response)
    print(response_body)
    print(status_code)
    print(status_line)

    for key, value in pairs(response_body) do
        print(tostring(key) .. ": " .. tostring(value))
    end

    for key, value in pairs(response_headers) do
        print(tostring(key)..": "..tostring(value))
    end

    -- should actually be if status_code in the 200s
    if response ~= nil then
        print(response_body[1])
        local res = cjson.decode(response_body[1])
        print("\n"..res.message_1)
        local content = res.message_1
        return content.." "..input
    else
        local content = "failure".." "..tostring(status_code)
        return content
    end
end

