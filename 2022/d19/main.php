<?php
ini_set('memory_limit', '-1');

function build($blueprint, $cur, $robot, $max_time) {
	$time = [];
	foreach (["ore", "clay", "obsidian"] as $x) {
		if ($cur["prod"][$x] > 0) {
			if ($cur["has"][$x] >= $blueprint[$robot][$x]) {
				array_push($time, 0);
			} else {
				array_push($time, ceil(($blueprint[$robot][$x] - $cur["has"][$x]) / $cur["prod"][$x]));
			}
		}
	}
	$time = max($time) + 1;

	if ($cur["time"] + $time > $max_time) {
		$has = [];
		foreach (["ore", "clay", "obsidian", "geode"] as $x) {
			$has[$x] = $cur["has"][$x] + $cur["prod"][$x] * ($max_time - $cur["time"]);
		}
		return [
			"prod" => $cur["prod"],
			"time" => $max_time,
			"has" => $has,
		];
	}

	$has = [];
	foreach (["ore", "clay", "obsidian"] as $x) {
		$has[$x] = $cur["has"][$x] + $cur["prod"][$x] * $time - $blueprint[$robot][$x];
	}
	$has["geode"] = $cur["has"]["geode"] + $cur["prod"]["geode"] * $time;
	$prod = $cur["prod"];
	$prod[$robot] += 1;

	return [
		"time" => $cur["time"] + $time,
		"has" => $has,
		"prod" => $prod,
	];
}

function max_geodes($blueprint, $max_time) {
	global $maxcost;

	$max = 0;
	$s = new \Ds\Stack();
	$v = new \Ds\Set();
	$s->push([
		"time" => 0,
		"has" => ["ore" => 0, "clay" => 0, "obsidian" => 0, "geode" => 0],
		"prod" => ["ore" => 1, "clay" => 0, "obsidian" => 0, "geode" => 0],
	]);
	while (!$s->isEmpty()) {
		$cur = $s->pop();
		if (!$cur)
			continue;
		if ($cur["time"] == $max_time) {
			if ($cur["has"]["geode"] > $max)
				$max = $cur["has"]["geode"];
			continue;
		}
		if (!$v->contains($cur)) {
			$v->add($cur);
			$time_left = $max_time - $cur["time"];
			if ($max > $cur["has"]["geode"] + $time_left * ($cur["prod"]["geode"] + $time_left/2))
				continue;
			if ($cur["prod"]["ore"] < $maxcost["ore"])
				$s->push(build($blueprint, $cur, "ore", $max_time));
			if ($cur["prod"]["clay"] < $maxcost["clay"])
				$s->push(build($blueprint, $cur, "clay", $max_time));
			if ($cur["prod"]["clay"] > 0 && $cur["prod"]["obsidian"] < $maxcost["obsidian"])
				$s->push(build($blueprint, $cur, "obsidian", $max_time));
			if ($cur["prod"]["obsidian"] > 0)
				$s->push(build($blueprint, $cur, "geode", $max_time));
		}
	}
	return $max;
}


$input = fopen("input.txt", "r");
$blueprints = [];
$maxcost = ["ore" => 0, "clay" => 0, "obsidian" => 0];

while (!feof($input)) {
	if (preg_match_all("/\d+/", fgets($input), $matches)) {
		$matches = $matches[0];
		$blueprint = [
			"ore" => ["ore" => $matches[1], "clay" => 0, "obsidian" => 0],
			"clay" => ["ore" => $matches[2], "clay" => 0, "obsidian" => 0],
			"obsidian" => ["ore" => $matches[3], "clay" => $matches[4], "obsidian" => 0],
			"geode" => ["ore" => $matches[5], "clay" => 0, "obsidian" => $matches[6]]
		];
		$blueprints[$matches[0]] = $blueprint;

		foreach ($blueprint as $k => $v) {
			foreach (["ore", "clay", "obsidian"] as $x) {
				if ($v[$x] > $maxcost[$x]) {
					$maxcost[$x] = $v[$x];
				}
			}
		}
	}
}

$p1 = 0;
$p2 = 1;
foreach ($blueprints as $id => $blueprint) {
	$p1 += $id * max_geodes($blueprint, 24);
	if ($id <= 3)
		$p2 *= max_geodes($blueprint, 32);
}
echo $p1."\n";
echo $p2."\n";
