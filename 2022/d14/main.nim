import strutils
import sequtils

type
    State = enum # setup ascii debug drawing within type
        Air = "."
        Rock = "#"
        Sand = "o"

var
    cave: array[1000, array[1000, State]] # better safe than sorry
    maxy = 0

const
    source = (500, 0)

proc draw(x1: int, y1: int, x2: int, y2: int) =
    if x1 == x2:
        var (y1, y2) = (y1, y2)
        if y1 > y2: swap(y1, y2)
        for y in y1..y2: cave[y][x1] = Rock
    elif y1 == y2:
        var (x1, x2) = (x1, x2)
        if x1 > x2: swap(x1, x2)
        for x in x1..x2: cave[y1][x] = Rock

proc sand(round: int): bool =
    var (x, y) = source
    while true:
        y = y + 1
        if round == 0 and y >= len(cave): return false
        elif cave[0][500] == Sand: return false

        case cave[y][x]:
        of Air: continue
        of Rock, Sand:
            if cave[y][x-1] == Air:
                x = x - 1
                continue
            elif cave[y][x+1] == Air:
                x = x + 1
                continue
            else:
                cave[y-1][x] = Sand
                return true


for line in readFile("input.txt").strip().splitLines():
    let coords = line.split(" -> ")
    for i in 1..<len(coords):
        let start = coords[i-1].split(',').map(parseInt)
        let ende = coords[i].split(',').map(parseInt)

        if start[1] > maxy: maxy = start[1]
        if ende[1] > maxy: maxy = ende[1]

        draw(start[0], start[1], ende[0], ende[1])

for round in 0..1:
    var count = 0
    while sand(round): count += 1
    echo count

    # Round 2 setup
    for x, _ in cave[maxy+2]: cave[maxy+2][x] = Rock
    for y, _ in cave:
        for x, _ in cave[y]:
            if cave[y][x] == Sand: cave[y][x] = Air
