def prio(char):
	ascii = ord(char)
	if ascii in range(ord("a"), ord("z") + 1):
		return ascii - 96
	elif ascii in range(ord("A"), ord("Z") + 1):
		return ascii - 38


part1 = 0
with open("input.txt") as file:
	for line in file:
		line = line.strip()
		c1 = {char for char in line[: len(line) // 2]}
		c2 = {char for char in line[len(line) // 2 :]}
		part1 += prio(c1.intersection(c2).pop())
print(part1)

part2 = 0
with open("input.txt") as file:
	buffer = []
	for line in file:
		buffer.append({char for char in line.strip()})
		if len(buffer) == 3:
			part2 += prio(buffer[0].intersection(buffer[1], buffer[2]).pop())
			buffer = []
print(part2)
