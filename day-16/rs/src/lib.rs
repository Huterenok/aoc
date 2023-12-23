use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn energize(grid: &Vec<Vec<Tile>>, start: (i32, i32, i32, i32)) -> usize {
    let mut queue: VecDeque<(i32, i32, i32, i32)> = VecDeque::from([start]);
    let mut set: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    while !queue.is_empty() {
        let (mut x, mut y, dx, dy) = queue.pop_front().unwrap();

        x += dx;
        y += dy;
        if x < 0 || x as usize >= grid[0].len() || y < 0 || y as usize >= grid.len() {
            continue;
        }
        let tile = grid[y as usize][x as usize];

        if tile == Tile::Dot || (tile == Tile::Vert && dy != 0) || (tile == Tile::Horiz && dx != 0)
        {
            if set.insert((x, y, dx, dy)) {
                queue.push_back((x, y, dx, dy));
            }
        } else if tile == Tile::UpRight {
            let (dx, dy) = (-dy, -dx);
            if set.insert((x, y, dx, dy)) {
                queue.push_back((x, y, dx, dy));
            }
        } else if tile == Tile::UpLeft {
            let (dx, dy) = (dy, dx);
            if set.insert((x, y, dx, dy)) {
                queue.push_back((x, y, dx, dy));
            }
        } else {
            if tile == Tile::Vert {
                [(0, 1), (0, -1)].into_iter().for_each(|(dx, dy)| {
                    if set.insert((x, y, dx, dy)) {
                        queue.push_back((x, y, dx, dy));
                    }
                })
            } else {
                [(1, 0), (-1, 0)].into_iter().for_each(|(dx, dy)| {
                    if set.insert((x, y, dx, dy)) {
                        queue.push_back((x, y, dx, dy));
                    }
                })
            }
        }
    }

    set.into_iter().map(|(x, y, _, _)| (x, y)).unique().count()
}

fn find_max_energized(grid: &Vec<Vec<Tile>>) -> usize {
    (0..grid.len())
        .flat_map(|y| {
            vec![
                energize(grid, (-1, y as i32, 1, 0)),
                energize(grid, (grid[0].len() as i32, y as i32, -1, 0)),
            ]
        })
        .chain((0..grid[0].len()).flat_map(|x| {
            vec![
                energize(grid, (x as i32, -1, 0, 1)),
                energize(grid, (x as i32, grid.len() as i32, 0, 1)),
            ]
        }))
        .max()
        .unwrap()
}

pub fn parse_grid(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .into_iter()
        .map(|x| x.chars().map(Tile::from).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yays() {
        let example_input = std::fs::read_to_string("../example_input.txt").unwrap();
        let input = std::fs::read_to_string("../input.txt").unwrap();

        let grid_example = parse_grid(&example_input);
        let grid = parse_grid(&input);

        let res1_example = energize(&grid_example, (-1, 0, 1, 0));
        let res1 = energize(&grid, (-1, 0, 1, 0));

        println!("Result 1: example - {}, real - {}", res1_example, res1);
        assert_eq!(46, res1_example);
        assert_eq!(7543, res1);

        let res2_example = find_max_energized(&grid_example);
        let res2 = find_max_energized(&grid);

        println!("Result 2: example - {}, real - {}", res2_example, res2);
        assert_eq!(51, res2_example);
        assert_eq!(8231, res2);
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Tile {
    Dot,
    Horiz,
    Vert,
    UpRight,
    UpLeft,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Dot,
            '-' => Self::Horiz,
            '|' => Self::Vert,
            '/' => Self::UpRight,
            '\\' => Self::UpLeft,
            _ => panic!("RETARDED"),
        }
    }
}
