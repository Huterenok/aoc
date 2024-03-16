use std::cmp::{max, min, Ordering};
use std::ops::Range;

use regex::Regex;

type Cube = (bool, [Range<i64>; 3]);

fn turn_cuboid_slightly(input: &str) -> usize {
    let instructions = parse_instructions(input);
    let mut cuboid = vec![vec![vec![false; 101]; 101]; 101];

    for (is_on, [xr, yr, zr]) in instructions {
        for x in xr.start.max(-50)..=xr.end.min(50) {
            for y in yr.start.max(-50)..=yr.end.min(50) {
                for z in zr.start.max(-50)..=zr.end.min(50) {
                    let (x, y, z) = (x + 50, y + 50, z + 50);
                    cuboid[x as usize][y as usize][z as usize] = is_on;
                }
            }
        }
    }

    cuboid
        .into_iter()
        .flatten()
        .flatten()
        .filter(|n| *n)
        .count()
}

fn turn_cuboid_not_slightly(input: &str) -> i64 {
    let instructions = parse_instructions(input);

    (0..instructions.len())
        .filter(|&i| instructions[i].0)
        .map(|i| corrected_volume(&instructions[i], &instructions[i + 1..]))
        .sum()
}

fn volume((_, rngs): &Cube) -> i64 {
    (rngs[0].end - rngs[0].start + 1)
        * (rngs[1].end - rngs[1].start + 1)
        * (rngs[2].end - rngs[2].start + 1)
}

fn subaxis(r1: &Range<i64>, r2: &Range<i64>) -> Option<Range<i64>> {
    if r1.end < r2.start {
        return None;
    }
    if r1.start > r2.end {
        return None;
    }
    let a = min(max(r1.start, r2.start), r2.end);
    let b = min(max(r1.end, r2.start), r2.end);
    Some(a..b)
}

fn subcube(
    r1: &Cube,
    r2: &Cube,
) -> Option<Cube> {
    let xr = subaxis(&r1.1[0], &r2.1[0])?;
    let yr = subaxis(&r1.1[1], &r2.1[1])?;
    let zr = subaxis(&r1.1[2], &r2.1[2])?;
    Some((r1.0, [xr, yr, zr]))
}

fn corrected_volume(c: &Cube, rest: &[Cube]) -> i64 {
    let subcubes = rest
        .iter()
        .filter_map(|c2| subcube(&c2, &c))
        .collect::<Vec<_>>();
    let vsubcubes = (0..subcubes.len())
        .map(|i| corrected_volume(&subcubes[i], &subcubes[i + 1..]))
        .sum::<i64>();
    volume(&c) - vsubcubes
}

fn parse_instructions(input: &str) -> Vec<Cube> {
    let matcher = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let turn = line.starts_with("on");
            let nums = matcher
                .find_iter(line)
                .map(|n| n.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (turn, [nums[0]..nums[1], nums[2]..nums[3], nums[4]..nums[5]])
        })
        .collect()
}

mod tests {
    use std::fs;

    use crate::{turn_cuboid_not_slightly, turn_cuboid_slightly};

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("example_input.txt").unwrap();
        let input = fs::read_to_string("input.txt").unwrap();

        let res1_example = turn_cuboid_slightly(&example_input);
        let res1 = turn_cuboid_slightly(&input);
        assert_eq!(590784, res1_example);
        assert_eq!(589411, res1);

				let res2 = turn_cuboid_not_slightly(&input);
        assert_eq!(res2, 1130514303649907);
    }
}
