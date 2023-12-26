const assert = require("assert");
const fs = require("fs");

const input = fs.readFileSync("input.txt").toString().replace(/\r/g, "");

const parseMap = (input) => input.split("\n").map((line) => line.split(""));

const isSymbol = (map, y, x) =>
  y >= 0 &&
  y < map.length &&
  x >= 0 &&
  x < map[0].length &&
  map[y][x] !== "." &&
  !/\d/.test(map[y][x]);

const isAdjacentToSymbols = (map, y, startX, endX) => {
  for (let x = startX - 1; x <= endX; x++) {
    if (isSymbol(map, y - 1, x) || isSymbol(map, y + 1, x)) return true;
  }
  return isSymbol(map, y, startX - 1) || isSymbol(map, y, endX);
};

const getNumbersAdjacentToSymbols = (map) =>
  map.flatMap((line, y) => {
    return [...line.join("").matchAll(/\d+/g)].map((match) => {
      const numBuffer = match[0];
      const startX = match.index;
      const endX = startX + numBuffer.length;
      return isAdjacentToSymbols(map, y, startX, endX) ? Number(numBuffer) : 0;
    });
  });

const getAdjacentGears = (map, y, startX, endX) => {
  const isGear = (y, x) => map[y]?.[x] === "*";
  const serialize = (y, x) => `${y}:${x}`;
  let result = [];

  for (let x = startX - 1; x <= endX; x++) {
    if (isGear(y - 1, x)) result.push(serialize(y - 1, x));
    if (isGear(y + 1, x)) result.push(serialize(y + 1, x));
  }
  if (isGear(y, startX - 1)) result.push(serialize(y, startX - 1));
  if (isGear(y, endX)) result.push(serialize(y, endX));

  return result;
};

const getAllGears = (map) =>
  map.reduce((acc, line, y) => {
    [...line.join("").matchAll(/\d+/g)].forEach((match) => {
      const numBuffer = match[0];
      const startX = match.index;
      const endX = startX + numBuffer.length;
      const gears = getAdjacentGears(map, y, startX, endX);

      gears.forEach((gear) => {
        if (!acc[gear]) acc[gear] = [];
        acc[gear].push(Number(numBuffer));
      });
    });
    return acc;
  }, {});

const filterProperGears = (gears) =>
  Object.keys(gears)
    .filter((gear) => gears[gear].length === 2)
    .reduce((acc, gear) => ({ ...acc, [gear]: gears[gear] }), {});

const sumOfGearRatios = (gears) =>
  Object.values(gears)
    .map(([a, b]) => a * b)
    .reduce((a, c) => a + c, 0);

const map = parseMap(input);

assert.equal(
  getNumbersAdjacentToSymbols(map).reduce((a, c) => a + c, 0),
  538046
);
assert.equal(sumOfGearRatios(filterProperGears(getAllGears(map))), 81709807);
