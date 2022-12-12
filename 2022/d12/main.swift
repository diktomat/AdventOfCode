// swiftformat:options --indent tab

import Foundation

var map = [[(val: Int, visited: Bool, parent: (Int, Int))]]()
var start = (x: 0, y: 0)
var end = (x: 0, y: 0)

for line in try! String(contentsOfFile: "input.txt").components(separatedBy: .newlines) {
	guard line.count > 0 else { continue }

	let line = Array(line)
	map.append(line.map { (Int($0.asciiValue!), false, (0, 0)) })

	if let startIndex = line.firstIndex(where: { $0 == "S" }) {
		start = (x: startIndex, y: map.count - 1)
	}
	if let endIndex = line.firstIndex(where: { $0 == "E" }) {
		end = (x: endIndex, y: map.count - 1)
	}
}

map[start.y][start.x].val = Int("a".first!.asciiValue!)
map[end.y][end.x].val = Int("z".first!.asciiValue!)
let size = (x: map[0].count, y: map.count)

for round in [1, 2] {
	if round == 2 {
		resetMap()
		start = end
	}

	var queue = [start]
	var cur = start
	map[cur.y][cur.x].visited = true
	while !queue.isEmpty {
		cur = queue.removeFirst()
		if round == 1 && cur == end || round == 2 && map[cur.y][cur.x].val == "a".first!.asciiValue! {
			break
		}
		for edge in edges(cur, round) {
			map[edge.y][edge.x].visited = true
			map[edge.y][edge.x].parent = cur
			queue.append(edge)
		}
	}

	var count = 0
	while cur != start {
		cur = map[cur.y][cur.x].parent
		count += 1
	}
	print(count)
}

func edges(_ cur: (x: Int, y: Int), _ round: Int) -> [(x: Int, y: Int)] {
	var ret: [(Int, Int)] = []
	for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
		let (x, y) = (cur.x + x, cur.y + y)

		guard x < size.x, y < size.y, x >= 0, y >= 0 else { continue }
		if map[y][x].visited { continue }
		let curHeight = map[cur.y][cur.x].val
		let edgeHeight = map[y][x].val
		if (round == 1 ? edgeHeight : curHeight) - (round == 1 ? curHeight : edgeHeight) > 1 { continue }

		ret.append((x, y))
	}
	return ret
}

func resetMap() {
	for x in 0 ..< size.x {
		for y in 0 ..< size.y {
			map[y][x].visited = false
		}
	}
}
