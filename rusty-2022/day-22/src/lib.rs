use regex::Regex;

fn lets_go(input: &str) -> i32 {
    let (grid, steps, moves) = parse_grid(input);
    let (mut steps, mut moves) = (steps.into_iter(), moves.into_iter());
    let (mut x, mut y, mut dx, mut dy) = (
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0 as i32,
        0,
        1,
        0,
    );

    while steps.len() != 0 && moves.len() != 0 {
        let (step, mov) = (steps.next().unwrap(), moves.next().unwrap());
        for _ in 0..step {
            let (mut curr_x, mut curr_y) = (
                (x + dx).rem_euclid(grid[0].len() as i32),
                (y + dy as i32).rem_euclid(grid.len() as i32),
            );

            while grid[curr_y as usize][curr_x as usize] == ' ' {
                curr_x = (curr_x + dx).rem_euclid(grid[0].len() as i32);
                curr_y = (curr_y + dy).rem_euclid(grid.len() as i32);
            }
            if grid[curr_y as usize][curr_x as usize] == '#' {
                break;
            }
            x = curr_x;
            y = curr_y;
        }

        if mov == 'R' {
            (dx, dy) = (-dy, dx);
        } else {
            (dx, dy) = (dy, -dx);
        }
    }

    let d = match (dx, dy) {
        (1, 0) => 0,
        (-1, 0) => 2,
        (0, 1) => 1,
        _ => 3,
    };
    1000 * (y + 1) + 4 * (x + 1) + d
}

fn i_dont_want_to_go_further(input: &str) -> i32 {
    let (grid, steps, moves) = parse_grid(input);
    let (mut steps, mut moves) = (steps.into_iter(), moves.into_iter());
    let (mut x, mut y, mut dx, mut dy) = (
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0 as i32,
        0,
        1,
        0,
    );

    while steps.len() != 0 && moves.len() != 0 {
        let (step, mov) = (steps.next().unwrap(), moves.next().unwrap());
        for _ in 0..step {
            let (curr_dx, curr_dy) = (dx, dy);
            let (mut curr_x, mut curr_y) = ((x + dx), (y + dy));

            //omg
            if curr_y < 0 && 50 <= curr_x && curr_x < 100 && dy == -1 {
                (dy, dx) = (0, 1);
                (curr_y, curr_x = curr_x + 100, 0);
            } else if curr_x < 0 && 150 <= curr_y && curr_y < 200 && dx == -1 {
                (dy, dx) = (1, 0);
                (curr_y, curr_x) = (0, curr_y - 100);
            } else if curr_y < 0 && 100 <= curr_x && curr_x < 150 && dy == -1 {
                (curr_y, curr_x) = (199, curr_x - 100);
            } else if curr_y >= 200 && 0 <= curr_x && curr_x < 50 && dy == 1 {
                (curr_y, curr_x) = (0, curr_x + 100);
            } else if curr_x >= 150 && 0 <= curr_y && curr_y < 50 && dx == 1 {
                dx = -1;
                (curr_y, curr_x) = (149 - curr_y, 99);
            } else if curr_x == 100 && 100 <= curr_y && curr_y < 150 && dx == 1 {
                dx = -1;
                (curr_y, curr_x) = (149 - curr_y, 149);
            } else if curr_y == 50 && 100 <= curr_x && curr_x < 150 && dy == 1 {
                (dy, dx) = (0, -1);
                (curr_y, curr_x) = (curr_x - 50, 99);
            } else if curr_x == 100 && 50 <= curr_y && curr_y < 100 && dx == 1 {
                (dy, dx) = (-1, 0);
                (curr_y, curr_x) = (49, curr_y + 50);
            } else if curr_y == 150 && 50 <= curr_x && curr_x < 100 && dy == 1 {
                (dy, dx) = (0, -1);
                (curr_y, curr_x) = (curr_x + 100, 49);
            } else if curr_x == 50 && 150 <= curr_y && curr_y < 200 && dx == 1 {
                (dy, dx) = (-1, 0);
                (curr_y, curr_x) = (149, curr_y - 100);
            } else if curr_y == 99 && 0 <= curr_x && curr_x < 50 && dy == -1 {
                (dy, dx) = (0, 1);
                (curr_y, curr_x) = (curr_x + 50, 50);
            } else if curr_x == 49 && 50 <= curr_y && curr_y < 100 && dx == -1 {
                (dy, dx) = (1, 0);
                (curr_y, curr_x) = (100, curr_y - 50);
            } else if curr_x == 49 && 0 <= curr_y && curr_y < 50 && dx == -1 {
                dx = 1;
                (curr_y, curr_x) = (149 - curr_y, 0);
            } else if curr_x < 0 && 100 <= curr_y && curr_y < 150 && dx == -1 {
                dx = 1;
                (curr_y, curr_x) = (149 - curr_y, 50);
            }

            if grid[curr_y as usize][curr_x as usize] == '#' {
                dy = curr_dy;
                dx = curr_dx;
                break;
            }

            (x, y) = (curr_x, curr_y);
        }

        if mov == 'R' {
            (dx, dy) = (-dy, dx);
        } else {
            (dx, dy) = (dy, -dx);
        }
    }

    let d = match (dx, dy) {
        (1, 0) => 0,
        (-1, 0) => 2,
        (0, 1) => 1,
        _ => 3,
    };
    1000 * (y + 1) + 4 * (x + 1) + d
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, Vec<i32>, Vec<char>) {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let move_matcher = Regex::new(r"[RL]").unwrap();
    let step_matcher = Regex::new(r"\d+").unwrap();

    let grid = grid
        .lines()
        .map(|line| {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.extend(vec![' '; width - line.len()]);
            chars
        })
        .collect();
    let steps = step_matcher
        .find_iter(instructions)
        .map(|c| c.as_str().parse().unwrap())
        .collect();
    let moves = move_matcher
        .find_iter(instructions)
        .map(|c| c.as_str().chars().next().unwrap())
        .collect();

    (grid, steps, moves)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = lets_go(&example_input);
        let res1 = lets_go(&input);
        assert_eq!(res1_example, 6032);
        assert_eq!(res1, 65368); //dunno

        let res2_example = i_dont_want_to_go_further(&example_input);
        let res2 = i_dont_want_to_go_further(&input);
        assert_eq!(res2_example, 5031);
        assert_eq!(res2, 156166);
    }
}
