use std::collections::HashMap;

use itertools::Itertools;

fn find_twice_overlapping(vents: impl Iterator<Item = (i32, i32, i32, i32)>) -> usize {
    let mut grid: HashMap<_, usize> = HashMap::new();
    vents.for_each(|(mut x1, mut y1, x2, y2)| {
        let dx = find_differential(x1, x2);
        let dy = find_differential(y1, y2);
        while (x1, y1) != (x2 + dx, y2 + dy) {
            *grid.entry((x1, y1)).or_default() += 1;
            (x1, y1) = (x1 + dx, y1 + dy);
        }
    });
    grid.values().filter(|v| **v >= 2).count()
}

fn find_differential(x1: i32, x2: i32) -> i32 {
    match x2.cmp(&x1) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn parse_vents(input: &str) -> Vec<(i32, i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|part| part.split(",").map(|s| s.parse().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let example_vents = parse_vents(&example_input);
        let vents = parse_vents(&input);
        let res1_example = find_twice_overlapping(
            example_vents
                .iter()
                .copied()
                .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2),
        );
        let res1 = find_twice_overlapping(
            vents
                .iter()
                .copied()
                .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2),
        );
        assert_eq!(res1_example, 5);
        assert_eq!(res1, 5147);

        let res2_example = find_twice_overlapping(example_vents.into_iter());
        let res2 = find_twice_overlapping(vents.into_iter());
        assert_eq!(res2_example, 12);
        assert_eq!(res2, 16925)
    }
}
