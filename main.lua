local http = require('socket.http')
local parser = require('htmlparser')
res = http.request("https://libgen.is/search.php?req=ap+world+history&lg_topic=libgen&open=0&view=simple&res=25&phrase=1&column=def")
local root = parser.parse(res, 5000)
local elements = root:select("tr td a")
print(#elements)

for _,e in ipairs(elements) do
	print(e:getcontent())
end