import assert from "assert";
import fs from "fs";

interface GameRecord {
  green: number;
  red: number;
  blue: number;
}

function part1(input: String): number {
  const lines = input.split("\n");

  const redRegexp = /(\d+)\s+red/g;
  const greenRegexp = /(\d+)\s+green/g;
  const blueRegexp = /(\d+)\s+blue/g;

  let res = 0;

  for (let i = 0; i < lines.length; i++) {
    const red = Math.max(
      ...lines[i].match(redRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );
    const green = Math.max(
      ...lines[i].match(greenRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );
    const blue = Math.max(
      ...lines[i].match(blueRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );

    res += red <= 12 && green <= 13 && blue <= 14 ? i + 1 : 0;
  }

  return res;
}

function part2(input: String): number {
  const lines = input.split("\n");

  const redRegexp = /(\d+)\s+red/g;
  const greenRegexp = /(\d+)\s+green/g;
  const blueRegexp = /(\d+)\s+blue/g;

  let res = 0;

  for (let i = 0; i < lines.length; i++) {
    const red = Math.max(
      ...lines[i].match(redRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );
    const green = Math.max(
      ...lines[i].match(greenRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );
    const blue = Math.max(
      ...lines[i].match(blueRegexp)?.map((x) => Number(x.match(/\d+/)![0]))!
    );

    res += red * green * blue;
  }

  return res;
}

function test() {
  let input = fs.readFileSync("./input.txt").toString();

  assert.equal(2810, part1(input));
  assert.equal(69110, part2(input));
}

test();
