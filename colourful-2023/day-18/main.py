# https://en.wikipedia.org/wiki/Shoelace_formula
# https://en.wikipedia.org/wiki/Pick%27s_theorem

import os
from typing import List, Tuple

dirs = {
    "U": (0, 1),
    "D": (0, -1),
    "L": (-1, 0),
    "R": (1, 0) 
}

def findInnerSquare(input: str, withHex: bool = False) -> int:
    records = parseRecords(input)
    cords = [(0, 0)]
    p = 0

    for direction, num, hex_value in records:
        if(withHex):
            num = int(hex_value[2:-2], 16)
            direction = convertDir(hex_value)
            
        num = int(num)
        p += num
        dx, dy = dirs[direction]
        lx, ly = cords[-1]
        cords.append([lx + dx * num, ly + dy * num])
        
    square = abs(sum(cords[i][1] * (cords[i - 1][0] - cords[(i + 1) % len(cords)][0]) for i in range(len(cords)))) // 2
    return square - p // 2 + p + 1

def convertDir(hex_value: str) -> str:
    match hex_value[-2]:
        case "0":
            return "R"
        case "1":
            return "D"
        case "2":
            return "L"
        case "3":
            return "U"

def parseRecords(input: str) -> List[List[str]]:
	return list(map(lambda line: line.split(" "), input.splitlines()))

input = open("input.txt").read()
example_input = open("example_input.txt").read()

res1_example = findInnerSquare(example_input)
res1 = findInnerSquare(input)

print(f"Result 1: example - {res1_example}, real - {res1}")

res2_example = findInnerSquare(example_input, True)
res2 = findInnerSquare(input, True)

print(f"Result 2: example - {res2_example}, real - {res2}")