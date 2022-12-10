FS = {}
STACK = {}

function cwd()
	return STACK[#STACK]
end

function pushd(dir)
	table.insert(STACK, dir)
end

function popd()
	table.remove(STACK)
end

function total_size(dir)
	if dir.total ~= nil then
		return dir.total
	end

	local size = dir.size
	for _, v in ipairs(dir.children) do
		v.total = v.total or total_size(v)
		size = size + v.total
	end
	return size
end

function part1()
	local sum = 0
	local inner
	inner = function(dir)
		for _, v in pairs(dir.children) do
			local total = total_size(v)
			if total <= 100000 then
				sum = sum + total
			end
			inner(v)
		end
	end
	inner(FS[1])
	return sum
end

function part2()
	local grand_total = total_size(FS[1])
	local to_free = 30000000 - (70000000 - grand_total)
	local deletion = grand_total
	local inner
	inner = function(dir)
		for _, v in pairs(dir.children) do
			local total = total_size(v)
			if total > to_free and total < deletion then
				deletion = total
			end
			inner(v)
		end
	end
	inner(FS[1])
	return deletion
end

for line in io.lines("input.txt") do
	if string.sub(line, 1, #"$ cd") == "$ cd" then
		local dirname = string.sub(line, 6, -1)
		if dirname == "/" then
			table.insert(FS, { name = "/", size = 0, children = {} })
			pushd(FS[1])
		elseif dirname ~= ".." then
			local dir = { name = dirname, size = 0, children = {} }
			table.insert(cwd().children, dir)
			pushd(dir)
		else
			popd()
		end
	elseif string.sub(line, 1, #"$ ls") == "$ ls" then
		-- listing dir, do nothing
	elseif string.sub(line, 1, #"dir ") == "dir " then
		-- is a dir, do nothing
	else
		cwd().size = cwd().size + string.sub(line, string.find(line, "%d+"))
	end
end

print(part1())
print(part2())
