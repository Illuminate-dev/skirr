local http = require('socket.http')
local parser = require('htmlparser')
local headers = {"id", "author", "title", "publisher", "year", "pages", "language", "size", "filetype", "link 1"}


res = http.request("https://libgen.is/search.php?req=ap+world+history&lg_topic=libgen&open=0&view=simple&res=25&phrase=1&column=def")
local root = parser.parse(res, 5000)
local elements = root:select("table")
print(#elements)

local subs = elements[2]("tr")

for i, sub in ipairs(subs) do
	if i == 1 then goto continue end
	for j, entry in ipairs(sub("td")) do
		if j > #headers then goto continue end
		print(headers[j] .. " " .. entry:getcontent())
	end
	
	::continue::
end