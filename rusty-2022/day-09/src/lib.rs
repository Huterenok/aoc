use std::collections::HashSet;

pub fn tour_of_tail(input: &str) -> usize {
    let moves = parse_moves(input);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let (mut head, mut tail) = ((0, 0), (0, 0));

    moves.into_iter().for_each(|((dx, dy), s)| {
        (0..s).for_each(|_| {
            head = (head.0 + dx, head.1 + dy);
            if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                tail = (head.0 - dx, head.1 - dy);
                visited.insert(tail);
            }
        });
    });

    visited.len()
}

pub fn tour_of_longer_tail(input: &str) -> usize {
    let moves = parse_moves(input);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut parts = vec![(0, 0); 10];

    moves.into_iter().for_each(|((dx, dy), s)| {
        (0..s).for_each(|_| {
            parts[0] = (parts[0].0 + dx, parts[0].1 + dy);

            for i in 1..10 {
                let (head, tail) = (parts[i - 1], &mut parts[i]);
                if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                    let d = (tail.0 - head.0, tail.1 - head.1);
                    let l = d.0.abs().max(d.1.abs());
                    let m = (d.0 / l, d.1 / l);
                    *tail = (head.0 + m.0, head.1 + m.1);
                    if i == 9 {
                        visited.insert(*tail);
                    }
                } else {
                    break;
                }
            }
        });
    });

    visited.len()
}

pub fn parse_moves(input: &str) -> Vec<((i32, i32), i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_once(" ").unwrap();
            let steps: i32 = steps.parse().unwrap();
            match dir {
                "R" => ((1, 0), steps),
                "L" => ((-1, 0), steps),
                "U" => ((0, 1), steps),
                _ => ((0, -1), steps),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = tour_of_tail(&example_input);
        let res1 = tour_of_tail(&input);
        assert_eq!(res1_example, 13);
        assert_eq!(res1, 6030);

        let res2_example = tour_of_longer_tail(&example_input);
        let res2 = tour_of_longer_tail(&input);
        assert_eq!(res2_example, 1);
        assert_eq!(res2, 2545);
    }
}
