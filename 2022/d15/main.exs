infile = "input.txt"
searchrow = 2_000_000

distance = fn x1, y1, x2, y2 -> abs(x2 - x1) + abs(y2 - y1) end

input =
	File.read!(infile)
	|> String.split("\n", trim: true)
	|> Enum.map(fn line ->
		Regex.scan(~r/-?\d+/, line)
		|> Enum.map(fn coord -> hd(coord) |> String.to_integer() end)
	end)
	|> Enum.map(fn coords -> List.to_tuple(coords) end)
	|> Enum.map(fn {sx, sy, bx, by} -> {sx, sy, bx, by, distance.(sx, sy, bx, by)} end)

blocked_on_searchrow =
	Enum.filter(input, fn coords -> elem(coords, 1) == searchrow || elem(coords, 3) == searchrow end)
	|> Enum.map(fn coords -> elem(coords, 2) end)
	|> Enum.uniq()

Enum.reduce(input, [], fn {sx, sy, _, _, dist}, acc ->
	dx = dist - abs(sy - searchrow)

	if dx > 0 do
		acc ++ Enum.to_list((sx - dx)..(sx + dx))
	else
		acc
	end
end)
|> Enum.uniq()
|> Enum.filter(fn x -> !Enum.any?(blocked_on_searchrow, fn bx -> x == bx end) end)
|> Enum.count()
|> IO.puts()

# For part 2 see part2.rs.
# I implemented the same algorithm in Elixir first, but when it was still running after
# 20h and after verifying on Reddit that my approach was sound, I just redid it in Rust,
# where it (same algorithm, as said) takes only 650ms to give the correct answer.
