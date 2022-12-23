type Pos = {
	x: number,
	y: number,
}

const North = [{ x: -1, y: -1 }, { x: 0, y: -1 }, { x: 1, y: -1 }];
const South = [{ x: -1, y: 1 }, { x: 0, y: 1 }, { x: 1, y: 1 }];
const East = [{ x: 1, y: -1 }, { x: 1, y: 0 }, { x: 1, y: 1 }];
const West = [{ x: -1, y: -1 }, { x: -1, y: 0 }, { x: -1, y: 1 }];
let moves = [North, South, West, East];

var garden = new Set<string>;
var elves: Pos[] = [];

function str(pos: Pos): string {
	return pos.x + ';' + pos.y;
}

function get_move(elve: Pos): Pos | void {
	let count = 0;
	let pos = elve;

	for (let move of moves) {
		if (move.every(e => { return !garden.has(str({ x: elve.x + e.x, y: elve.y + e.y })) })) {
			if (count == 0) {
				pos = { x: elve.x + move[1].x, y: elve.y + move[1].y }
			}
			count++;
		}
	}

	if (count < 4 && count != 0) {
		return pos;
	}
}

const input = await Bun.file("input.txt").text();
input.split("\n").forEach((line, y) => {
	line.split("").forEach((char, x) => {
		if (char == "#") {
			garden.add(str({ x, y }));
			elves.push({ x, y });
		}
	})
})

for (let i = 1; i < Number.MAX_SAFE_INTEGER; i++) {
	let proposed_moves: [number, Pos][] = [];

	elves.forEach((elve, id) => {
		let proposal = get_move(elve);
		if (proposal) {
			proposed_moves.push([id, proposal]);
		}
	})

	proposed_moves.forEach(move => {
		if (proposed_moves.filter(e => { return e[1].x == move[1].x && e[1].y == move[1].y }).length == 1) {
			garden.delete(str(elves[move[0]]));
			elves[move[0]] = move[1];
			garden.add(str(move[1]));
		}
	})

	moves.push(moves.shift()!);

	if (i == 10) { // part 1
		let maxx = elves[0].x;
		let minx = maxx;
		let maxy = elves[0].y;
		let miny = maxy;
		for (let elve of elves) {
			if (elve.x > maxx) { maxx = elve.x };
			if (elve.x < minx) { minx = elve.x };
			if (elve.y > maxy) { maxy = elve.y };
			if (elve.y < miny) { miny = elve.y };
		}
		let empty = (maxx - minx + 1) * (maxy - miny + 1) - elves.length;
		console.log(empty);
	}

	if (proposed_moves.length == 0) { // part 2
		console.log(i);
		break;
	}
}
