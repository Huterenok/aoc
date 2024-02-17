use std::collections::{BinaryHeap, HashSet};

fn low_da_risk(input: &str, expand: bool) -> usize {
    let grid = parse_grid(input);
    let grid = if expand { expand_grid(grid) } else { grid };
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State {
        pos: (0, 0),
        total_risk: 0,
    });
    visited.insert((0, 0));

    while let Some(state) = heap.pop() {
        if state.pos == (grid.len() as i32 - 1, grid[0].len() as i32 - 1) {
            return state.total_risk;
        }

        let (x, y) = state.pos;
        for (new_x, new_y) in [(x, y + 1), (x + 1, y)] {
            if new_x < grid[0].len() as i32
                && new_y < grid.len() as i32
                && visited.insert((new_x, new_y))
            {
                heap.push(State {
                    pos: (new_x, new_y),
                    total_risk: state.total_risk + grid[new_y as usize][new_x as usize],
                })
            }
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    total_risk: usize,
    pos: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .total_risk
            .cmp(&self.total_risk)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn expand_grid(grid: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..5 * grid.len())
        .map(|y| {
            (0..5 * grid[0].len())
                .map(|x| {
                    let cost = grid[y % grid.len()][x % grid[0].len()]
                        + (y / grid.len())
                        + (x / grid[0].len());
                    if cost <= 9 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect()
        })
        .collect()
}

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = low_da_risk(&example_input, false);
        let res1 = low_da_risk(&input, false);
        assert_eq!(res1_example, 40);
        assert_eq!(res1, 811);

        let res2_example = low_da_risk(&example_input, true);
        let res2 = low_da_risk(&input, true);
        assert_eq!(res2_example, 315);
        assert_eq!(res2, 3019);
    }
}
