#!/usr/bin/env fish

set clock 0
set X 1
set sigstren 0

function cycle
	set clock (math $clock + 1)

	if contains (math "($clock - 1) % 40") (math $X - 1) $X (math $X + 1)
		echo -n "█"
	else
		echo -n " "
	end

	if test (math $clock % 40) -eq 0
		echo
	end

	if contains $clock 20 60 100 140 180 220
		set sigstren (math $sigstren + $clock x $X)
	end
end

for line in (cat input.txt)
	cycle
	if test $line != "noop"
		cycle
		set X (math $X + (string split -f2 " " $line))
	end
end

echo
echo $sigstren
