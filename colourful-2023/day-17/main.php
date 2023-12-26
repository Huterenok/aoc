<?php

const MOVES = [[0, -1], [1, 0], [0, 1], [-1, 0]];
function efficientlyMoveCrucible($input, $maxStepCount = 3, $minStepCount = 1)
{
	$grid = parseGrid($input);
	$map = [];
	$heap = new SplMinHeap();

	$heap->insert([0, [0, 0, -1, 0]]);

	while (!$heap->isEmpty()) {
		[$curr, [$x, $y, $dir, $dirCount]] = $heap->extract();

		if ($x == count($grid[0]) - 1 && $y == count($grid) - 1)
			return $curr;

		foreach (MOVES as $i => $move) {
			[$newX, $newY] = [$x + $move[0], $y + $move[1]];

			if (($i + 2) % 4 == $dir || $newX < 0 || $newY < 0 || $newX >= count($grid[0]) || $newY >= count($grid))
				continue;

			$newDirCount = $dir == $i ? $dirCount + 1 : 1;
			if ($newDirCount > $maxStepCount || ($curr && $i != $dir && $dirCount < $minStepCount))
				continue;

			$newCurr = $curr + $grid[$newY][$newX];

			$key = "$newX,$newY,$i,$newDirCount";
			if ($newCurr < ($map[$key] ?? INF)) {
				$map[$key] = $newCurr;
				$heap->insert([$newCurr, [$newX, $newY, $i, $newDirCount]]);
			}
		}
	}

	return -1;
}

function parseGrid(string $input)
{
	return array_map(function ($line) {
		return array_map("intval", str_split($line));
	}, explode("\n", $input));
}

$input = file_get_contents("input.txt");
$example_input = file_get_contents("example_input.txt");

$res1_example = efficientlyMoveCrucible($example_input);
$res1 = efficientlyMoveCrucible($input);

echo "Result 1: example - $res1_example, real - $res1\n";

$res2_example = efficientlyMoveCrucible($example_input, 10, 4);
$res2 = efficientlyMoveCrucible($input, 10, 4);

echo "Result 2: example - $res2_example, real - $res2\n";
?>