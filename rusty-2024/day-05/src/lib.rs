use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn sum_correct_middles(input: &str) -> usize {
    let (rules, updates) = parse_updates(input);

    updates
        .into_iter()
        .filter(|nums| {
            let mut must_be_ahead: HashSet<usize> = HashSet::new();
            nums.iter().rev().all(|num| {
                if let Some(nums_ahead) = rules.get(num) {
                    must_be_ahead.extend(nums_ahead)
                }
                !must_be_ahead.contains(num)
            })
        })
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

pub fn sum_corrected_by_myself_frickin_elves_middles(input: &str) -> usize {
    let (rules, updates) = parse_updates(input);

    updates
        .into_iter()
        .filter(|nums| {
            let mut must_be_ahead: HashSet<usize> = HashSet::new();
            nums.iter().rev().any(|num| {
                if let Some(nums_ahead) = rules.get(num) {
                    must_be_ahead.extend(nums_ahead)
                }
                must_be_ahead.contains(num)
            })
        })
        .map(|mut nums| {
            nums.sort_by(|a, b| match rules.get(a) {
                Some(rules) if rules.contains(b) => Ordering::Less,
                _ => Ordering::Equal,
            });
            nums
        })
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

fn parse_updates(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let rules = rules_str
        .lines()
        .map(|rule| {
            let (before, after) = rule.split_once('|').unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .fold(
            HashMap::with_capacity(rules_str.lines().count()),
            |mut rules, (before, after)| {
                rules
                    .entry(before)
                    .and_modify(|a: &mut Vec<usize>| a.push(after))
                    .or_insert(vec![after]);
                rules
            },
        );
    let updates = updates_str
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        let res = sum_correct_middles(TEST_INPUT);
        assert_eq!(res, 143);

        let real_res = sum_correct_middles(REAL_INPUT);
        assert_eq!(real_res, 6267);
    }

    #[test]
    fn part_2() {
        let res = sum_corrected_by_myself_frickin_elves_middles(TEST_INPUT);
        assert_eq!(res, 123);

        let real_res = sum_corrected_by_myself_frickin_elves_middles(REAL_INPUT);
        assert_eq!(real_res, 5184);
    }
}
