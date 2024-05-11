local http = require('socket.http')
local parser = require('htmlparser')
local headers = {"id", "author", "title", "publisher", "year", "pages", "language", "size", "filetype", "link 1"}

function Search (term)
	local link = "https://libgen.is/search.php?req=".. string.gsub(term, " ", "+")
	print(link)
	local res = http.request(link)
	local root = parser.parse(res, 5000)
	local elements = root:select("table")

	local subs = elements[2]("tr")

	local entries = {}
	for i, sub in ipairs(subs) do
		if i == 1 then goto continue end
		local entry = {}
		for j, row in ipairs(sub("td")) do
			if j > #headers then break end
			if headers[j] == "author" then
				entry[headers[j]] = row("a")[1]:getcontent()
			elseif headers[j] == "title" then
				local titleElement = row('a[title]')[1]
				entry[headers[j]] = titleElement:getcontent()
				for _, child in ipairs(titleElement.nodes) do
					entry[headers[j]] = string.gsub(entry[headers[j]], "<.*/%a+>", "")
				end
			elseif headers[j] == "link 1" then
				entry[headers[j]] = row("a")[1].attributes["href"]
			else
				entry[headers[j]] = row:getcontent()
			end
		end
		table.insert(entries, entry)
		::continue::
	end
	return entries
end
