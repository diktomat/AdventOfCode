using Match
using DataStructures

struct Blizzard
	x
	y
	dx
	dy
end

function neighbours(coord)
	x = coord[1]
	y = coord[2]
	filter(c -> (c[1] >= 0 && c[2] >= 0 && c[1] <= width && c[2] <= height) || c == goal || c == start,
		[(x + 1, y), (x, y + 1), (x, y), (x, y - 1), (x - 1, y)])
end

function create_blizzard(char, x, y)
	(dx, dy) = @match char begin
		'>' => (1, 0)
		'<' => (-1, 0)
		'^' => (0, -1)
		'v' => (0, 1)
	end
	Blizzard(x - 2, y - 2, dx, dy)
end

function advance_blizzard(blizzard, rounds)
	x = mod(blizzard.x + (rounds * blizzard.dx), width + 1)
	y = mod(blizzard.y + (rounds * blizzard.dy), height + 1)
	(x, y)
end

function get_blizzards(round)
	if !haskey(blizzards_cache, round)
		blizzards_cache[round] = [advance_blizzard(blizzard, round) for blizzard in input]
	end
	blizzards_cache[round]
end

function bfs(from, to, round=0)
	q = Queue{Any}()
	visited = Set()
	push!(visited, (from, round))
	enqueue!(q, (from, round))

	while !isempty(q)
		(v, r) = dequeue!(q)
		if v == to
			return r
		end
		for coord in filter(coord -> (coord, r + 1) ∉ visited, neighbours(v))
			if coord in get_blizzards(r + 1)
				continue
			end
			push!(visited, (coord, r + 1))
			enqueue!(q, (coord, r + 1))
		end
	end
end

infile = "input.txt"
width = length(readline(infile)) - 3
height = length(readlines(infile)) - 3
start = (0, -1)
goal = (width, height + 1)

input = [create_blizzard(char, x, y)
	for (y, line) = enumerate(eachline(infile))
	for (x, char) = enumerate(line)
	if char in ['>', '<', '^', 'v']]
blizzards_cache = Dict()

first = bfs(start, goal)
println(first)
second = bfs(goal, start, first)
third = bfs(start, goal, second)
println(third)
