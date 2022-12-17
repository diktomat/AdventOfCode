import 'dart:io';

const boulderTypes = [
	[
		[false, false, false, false],
		[false, false, false, false],
		[false, false, false, false],
		[true,  true,  true,  true],
	],
	[
		[false, false, false, false],
		[false, true,  false, false],
		[true,  true,  true,  false],
		[false, true,  false, false],
	],
	[
		[false, false, false, false],
		[false, false, true,  false],
		[false, false, true,  false],
		[true,  true,  true,  false],
	],
	[
		[true, false, false, false],
		[true, false, false, false],
		[true, false, false, false],
		[true, false, false, false]
	],
	[
		[false, false, false, false],
		[false, false, false, false],
		[true,  true,  false, false],
		[true,  true,  false, false],
	]
];
const undo = {">": "<", "<": ">", "^": "v", "v": "^"};
Set<String> cave = {};

List<List<List<int>>> update(List<List<List<int>>> boulder, String direction, [int step = 1]) {
	late int idx;
	switch (direction) {
		case "<":
			idx = 1;
			step = -step;
			break;
		case ">":
			idx = 1;
			step = step;
			break;
		case "v":
			idx = 0;
			step = -step;
			break;
		case "^":
			idx = 0;
			step = step;
			break;
	}

	for (var i = 0; i < boulder.length; i++) {
		for (var j = 0; j < boulder[i].length; j++) {
			boulder[i][j][idx] += step;
		}
	}
	return boulder;
}

bool wall(List<List<List<int>>> boulder, int boulderID) {
	for (var i = 0; i < boulder.length; i++) {
		for (var j = 0; j < boulder[i].length; j++) {
			if (boulderTypes[boulderID][i][j] && (boulder[i][j][1] < 0 || boulder[i][j][1] > 6))
				return true;
		}
	}
	return false;
}

bool collision(List<List<List<int>>> boulder, int boulderID) {
	for (var i = 0; i < boulder.length; i++) {
		for (var j = 0; j < boulder[i].length; j++) {
			if (boulderTypes[boulderID][i][j] &&
					(cave.contains(posID(boulder[i][j])) || boulder[i][j][0] == 0))
				return true;
		}
	}
	return false;
}

void updateCave(List<List<List<int>>> boulder, int boulderID) {
	for (var i = 0; i < boulder.length; i++) {
		for (var j = 0; j < boulder[i].length; j++) {
			if (boulderTypes[boulderID][i][j])
				cave.add(posID(boulder[i][j]));
		}
	}
}

String posID(List<int> pos) {
	return "${pos[0]}|${pos[1]}";
}

int findMax(List<List<List<int>>> boulder, int boulderID) {
	var max = 0;
	for (var i = 0; i < boulder.length; i++) {
		for (var j = 0; j < boulder[i].length; j++) {
			if (boulderTypes[boulderID][i][j] && boulder[i][j][0] > max)
				max = boulder[i][j][0];
		}
	}
	return max;
}

void main() async {
	var jets = (await File("input.txt").readAsString()).trim();
	var jetidx = 0;
	var max = 0;

	for (var i = 0; i < 2022; i++) {
		var boulderID = i % boulderTypes.length;
		List<List<List<int>>> boulder = [
			[[4,2], [4,3], [4,4], [4,5]],
			[[3,2], [3,3], [3,4], [3,5]],
			[[2,2], [2,3], [2,4], [2,5]],
			[[1,2], [1,3], [1,4], [1,5]],
		];
		boulder = update(boulder, "^", max + 3);

		while (!collision(boulder, boulderID)) {
			var jet = jets[jetidx % jets.length];
			jetidx++;

			boulder = update(boulder, jet);
			if (wall(boulder, boulderID) || collision(boulder, boulderID))
				boulder = update(boulder, undo[jet]!);

			boulder = update(boulder, "v");
		}
		boulder = update(boulder, "^");

		int newmax = findMax(boulder, boulderID);
		if (newmax > max) max = newmax;
		updateCave(boulder, boulderID);
	}
	print(max);
}
