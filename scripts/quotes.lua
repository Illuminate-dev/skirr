-- helper functions
local char_to_hex = function(c)
	return string.format("%%%02X", string.byte(c))
end

local function urlencode(url)
	if url == nil then
		return
	end
	url = url:gsub("\n", "\r\n")
	url = url:gsub("([^%w ])", char_to_hex)
	url = url:gsub(" ", "+")
	return url
end

-- end of helper functions

BASE_URL = "https://quotes.toscrape.com"
function Get_URL(query)
	-- return BASE_URL .. urlencode(query)
	return BASE_URL
end

function Search(term)
	local link = Get_URL(term)
	local html = Get_HTML(link)

	local entries = {}

	for _, element in ipairs(html:select(".quote .text")) do
		local entry = {}
		entry["quote"] = element.text
		table.insert(entries, entry)
	end

	return entries
end

function Display(entry)
	local display_map = {}
	display_map["main_text"] = entry["quote"]
	return display_map
end

-- local headers = { "id", "author", "title", "publisher", "year", "pages", "language", "size", "filetype", "link 1" }

-- function Search(term)
-- 	local link = Get_URL(term)
-- 	local res = http.request(link)
-- 	local root = parser.parse(res, 5000)
-- 	local elements = root:select("table")
--
-- 	local subs = elements[2]("tr")
--
-- 	local entries = {}
-- 	for i, sub in ipairs(subs) do
-- 		if i == 1 then goto continue end
-- 		local entry = {}
-- 		for j, row in ipairs(sub("td")) do
-- 			if j > #headers then break end
-- 			if headers[j] == "author" then
-- 				entry[headers[j]] = row("a")[1]:getcontent()
-- 			elseif headers[j] == "title" then
-- 				local titleElement = row('a[title]')[1]
-- 				entry[headers[j]] = titleElement:getcontent()
-- 				for _, child in ipairs(titleElement.nodes) do
-- 					entry[headers[j]] = string.gsub(entry[headers[j]], "<.*/%a+>", "")
-- 				end
-- 			elseif headers[j] == "link 1" then
-- 				entry[headers[j]] = row("a")[1].attributes["href"]
-- 			else
-- 				entry[headers[j]] = row:getcontent()
-- 			end
-- 		end
-- 		table.insert(entries, entry)
-- 		::continue::
-- 	end
-- 	return entries
-- end
