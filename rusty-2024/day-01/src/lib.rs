pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
use std::collections::HashMap;

use itertools::Itertools;

pub fn find_total_distance(input: &str) -> usize {
    let (office_line1, office_line2) = parse_offices(input);

    office_line1
        .into_iter()
        .sorted()
        .zip(office_line2.into_iter().sorted())
        .map(|(id1, id2)| id1.abs_diff(id2))
        .sum()
}

pub fn find_similarity_score(input: &str) -> usize {
    let (office_line1, office_line2) = parse_offices(input);

    let mut similarity_score_map = office_line1
        .iter()
        .map(|&id| (id, 0))
        .collect::<HashMap<_, _>>();
    office_line2.into_iter().for_each(|id| {
        similarity_score_map
            .entry(id)
            .and_modify(|count| *count += 1);
    });

    office_line1
        .into_iter()
        .map(|id| {
            similarity_score_map
                .get(&id)
                .expect("similarity_score_map is based on office_line1 values")
                * id
        })
        .sum()
}

fn parse_offices(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|id| id.parse::<usize>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1() {
        let res = find_total_distance(TEST_INPUT);
        assert_eq!(11, res);

        let real_input = include_str!("../input.txt");
        let real_res = find_total_distance(real_input);
        assert_eq!(1506483, real_res);
    }

    #[test]
    fn part_2() {
        let res = find_similarity_score(TEST_INPUT);
        assert_eq!(31, res);

        let real_input = include_str!("../input.txt");
        let real_res = find_similarity_score(real_input);
        assert_eq!(23126924, real_res);
    }
}
