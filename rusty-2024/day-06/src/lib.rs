use std::collections::HashSet;
use std::ops::Add;

type Point = (i32, i32);

struct Lab {
    obstructions: HashSet<Point>,
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct GuardState {
    pos: Point,
    flow: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn when_will_she_leave_lab(input: &str) -> usize {
    let (lab, guard) = parse_lab(input);
    let visited = visited_places(&lab, guard);

    visited.len()
}

pub fn loop_it_till_she_die(input: &str) -> usize {
    let (lab, guard) = parse_lab(input);
    let visited = visited_places(&lab, guard);

    visited
        .iter()
        .filter(|&&new_obs_pos| {
            let mut guard = guard.clone();
            let mut revisited_states = HashSet::new();

            while guard.check_bounds(&lab) {
                if !revisited_states.insert(guard) {
                    return true;
                }

                let maybe_next_pos = guard.pos + guard.flow;
                if lab.obstructions.get(&maybe_next_pos).is_some() || maybe_next_pos == new_obs_pos
                {
                    guard.flow = guard.flow.brbrbr();
                } else {
                    guard.pos = maybe_next_pos;
                }
            }

            false
        })
        .count()
}

fn visited_places(lab: &Lab, mut guard: GuardState) -> HashSet<Point> {
    let mut visited = HashSet::new();

    while guard.check_bounds(&lab) {
        visited.insert(guard.pos);

        let maybe_next_pos = guard.pos + guard.flow;
        if lab.obstructions.get(&maybe_next_pos).is_some() {
            guard.flow = guard.flow.brbrbr();
        } else {
            guard.pos = maybe_next_pos;
        }
    }

    visited
}

fn parse_lab(input: &str) -> (Lab, GuardState) {
    let (mut obstructions, mut guard) = (HashSet::new(), None);

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .for_each(|(pos, c)| match c {
            '#' => {
                obstructions.insert(pos);
            }
            '^' | '>' | '<' | 'v' => {
                let flow = match c {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    '<' => Direction::West,
                    'v' => Direction::South,
                    _ => unreachable!("unreachable"),
                };
                guard = Some(GuardState { flow, pos })
            }
            _ => {}
        });

    let (height, width) = (input.lines().count(), input.lines().next().unwrap().len());

    (
        Lab {
            obstructions,
            height,
            width,
        },
        guard.expect(":("),
    )
}

impl Direction {
    fn brbrbr(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn as_point_flow(&self) -> Point {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl GuardState {
    pub fn check_bounds(&self, lab: &Lab) -> bool {
        self.pos.0 >= 0
            && self.pos.1 >= 0
            && self.pos.0 < lab.width as i32
            && self.pos.1 < lab.height as i32
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        let (dx, dy) = rhs.as_point_flow();
        (self.0 + dx, self.1 + dy)
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        let res = when_will_she_leave_lab(TEST_INPUT);
        assert_eq!(res, 41);

        let real_res = when_will_she_leave_lab(REAL_INPUT);
        assert_eq!(real_res, 4663);
    }

    #[test]
    fn part_2() {
        let res = loop_it_till_she_die(TEST_INPUT);
        assert_eq!(res, 6);

        let real_res = loop_it_till_she_die(REAL_INPUT);
        assert_eq!(real_res, 1530);
    }
}
