use std::ops::RangeInclusive;

use itertools::Itertools;
use regex::Regex;

fn find_highest_position(input: &str) -> Vec<i32> {
    let (x_range, y_range) = parse_target(input);

    (0..=*x_range.end())
        .cartesian_product(*y_range.start()..1000)
        .filter_map(|(x, y)| simulate(&x_range, &y_range, x, y))
        .collect()
}

fn simulate(
    x_range: &RangeInclusive<i32>,
    y_range: &RangeInclusive<i32>,
    mut dx: i32,
    mut dy: i32,
) -> Option<i32> {
    let (mut cur_x, mut cur_y, mut max_y) = (0, 0, 0);
    loop {
        (cur_x, cur_y, max_y) = (cur_x + dx, cur_y + dy, max_y.max(cur_y + dy));
        (dx, dy) = (dx - dx.signum(), dy - 1);

        match (x_range.contains(&cur_x), y_range.contains(&cur_y)) {
            (true, true) => {
                return Some(max_y);
            }
            (false, _) if dx == 0 => return None,
            (_, false) if dy < 0 && cur_y < *y_range.start() => return None,
            _ => {}
        }
    }
}

fn parse_target(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let matcher = Regex::new(r"-?\d+").unwrap();
    let matches = matcher
        .find_iter(input)
        .map(|n| i32::from_str_radix(n.as_str(), 10).unwrap())
        .collect::<Vec<_>>();

    (matches[0]..=matches[1], matches[2]..=matches[3])
}

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = find_highest_position(&example_input)
            .into_iter()
            .max()
            .unwrap();
        let res1 = find_highest_position(&input).into_iter().max().unwrap();
        assert_eq!(res1_example, 45);
        assert_eq!(res1, 11175);

        let res2_example = find_highest_position(&example_input).len();
        let res2 = find_highest_position(&input).len();
        assert_eq!(res2_example, 112);
        assert_eq!(res2, 3540);
    }
}
