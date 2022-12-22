package d22

import "core:fmt"
import "core:os"
import "core:strings"

Point :: struct{x, y: int}
pos: Point
maze: []string
cubelen: int
right :: 0
down  :: 1
left  :: 2
up    :: 3
direction: int

turn :: proc(side: rune) {
	switch side {
		case 'R': direction = (direction + 1) %% 4
		case 'L': direction = (direction - 1) %% 4
	}
}

move :: proc(count, part: int) {
	for _ in 0..<count {
		dx := (direction %% 2 == 0) ?  direction * -1  +  1 : 0
		dy := (direction %% 2 != 0) ? (direction -  2) * -1 : 0

		if part == 1 {
			y := (pos.y + dy) %% len(maze)
			x := (pos.x + dx) %% len(maze[y])

			switch maze[y][x] {
				case '.': pos = Point{x, y}; continue
				case '#': return
			}

			for maze[y][x] != '.' {
				y = (y + dy) %% len(maze)
				x = (x + dx) %% len(maze[y])
				if maze[y][x] == '#' {
					return
				}
			}
			pos = Point {x,y}
		} else {
			y := pos.y + dy
			x := pos.x + dx
			tmpdir := direction

			if y >= 0 && x >= 0 && y < len(maze) && x < len(maze[y]) && maze[y][x] != ' ' {
				// stay on same side of cube
			} else if y < 0 && x < 2*cubelen { // A upw
				tmpdir = right
				y = x + 2*cubelen
				x = 0
			} else if y < 0 && x >= 2*cubelen { // B upw
				y = 4*cubelen - 1
				x = x - 2*cubelen
			} else if x >= 3*cubelen { // B rightw
				tmpdir = left
				y = 2*cubelen + (cubelen - y - 1)
				x = 2*cubelen - 1
			} else if y >= cubelen && x >= 2*cubelen && direction == down { // B downw
				tmpdir = left
				y = x - cubelen
				x = 2*cubelen - 1
			} else if y >= cubelen && y < 2*cubelen && x >= 2*cubelen && direction == right { // C rightw
				tmpdir = up
				x = y + cubelen
				y = cubelen - 1
			} else if y >= 2*cubelen && x >= 2*cubelen { // E rightw
				tmpdir = left
				y = cubelen - (y - 2*cubelen) - 1
				x = 3*cubelen - 1
			} else if y >= 3*cubelen && x >= cubelen && direction == down { // E downw
				tmpdir = left
				y = x + 2*cubelen
				x = cubelen - 1
			} else if x >= cubelen && y >= 3*cubelen && direction == right { // F rightw
				tmpdir = up
				x = y - 2*cubelen
				y = 3*cubelen - 1
			} else if y >= 4*cubelen { // F downw
				y = 0
				x = 2*cubelen + x
			} else if x < 0 && y >= 3*cubelen { // F leftw
				tmpdir = down
				x = y - 2*cubelen
				y = 0
			} else if x < 0 { // D leftw
				tmpdir = right
				y = cubelen - (y - 2*cubelen + 1)
				x = cubelen
			} else if y < 2*cubelen && x < cubelen && direction == up { // D upw
				tmpdir = right
				y = cubelen + x
				x = cubelen
			} else if y < 2*cubelen && y >= cubelen && x < cubelen && direction == left { // C leftw
				tmpdir = down
				x = y - cubelen
				y = 2*cubelen
			} else if x < cubelen && y < cubelen { // A leftw
				tmpdir = right
				y = 3*cubelen - y - 1
				x = 0
			} else {
				fmt.println(pos, x, y, dx, dy, direction, tmpdir)
				fmt.println("WTF")
			}

			if maze[y][x] == '#' {
				return
			}

			pos = Point{x, y}
			direction = tmpdir
		}
	}
}

main :: proc() {
	inbytes, _   := os.read_entire_file_from_filename("input.txt")
	input        := strings.split(strings.clone_from_bytes(inbytes), "\n\n")
	maze          = strings.split_lines(input[0])
	instructions := strings.trim(input[1], "\n")
	cubelen       = len(maze[0]) / 3

	for part in 1..=2 {
		direction = right
		move_to_start: for line, y in input {
			for char, x in line {
				if char == '.' {
					pos = Point{x, y}
					break move_to_start
				}
			}
		}

		count := 0
		for c in instructions {
			switch c {
				case '0'..='9': count = count * 10 + int(c) - 48
				case: move(count, part); count = 0; turn(c)
			}
		}
		if count != 0 {
			move(count, part)
		}

		fmt.println(1000 * (pos.y+1) + 4 * (pos.x+1) + direction)
	}
}
