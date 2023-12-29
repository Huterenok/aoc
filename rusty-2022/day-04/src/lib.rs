use std::ops::RangeInclusive;

pub fn find_absorptions(input: &str) -> usize {
    let data = parse_data(input);
    data.into_iter()
        .filter(|(l, r)| {
            (l.start() >= r.start() && l.end() <= r.end())
                || (l.start() <= r.start() && l.end() >= r.end())
        })
        .count()
}

pub fn find_intersections(input: &str) -> usize {
    let data = parse_data(input);
    data.into_iter()
        .filter(|(l, r)| std::cmp::max(l.start(), r.start()) <= std::cmp::min(l.end(), r.end()))
        .count()
}

pub fn parse_data(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            let mut left_range = left.split("-").map(|x| x.parse::<usize>().unwrap());
            let mut right_range = right.split("-").map(|x| x.parse::<usize>().unwrap());

            (
                left_range.next().unwrap()..=left_range.next().unwrap(),
                right_range.next().unwrap()..=right_range.next().unwrap(),
            )
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

        let res1_example = find_absorptions(&example_input);
        let res1 = find_absorptions(&input);
        assert_eq!(res1_example, 2);
        assert_eq!(res1, 450);

        let res2_example = find_intersections(&example_input);
        let res2 = find_intersections(&input);
        assert_eq!(res2_example, 4);
        assert_eq!(res2, 837);
    }
}
