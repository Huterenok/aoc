use std::collections::VecDeque;

use regex::Regex;

pub fn rearrange_crates(input: &str) -> String {
    let (mut crates, moves) = parse_data(input);

    moves.into_iter().for_each(|mv| {
        (0..mv[0]).for_each(|_| {
            if let Some(new_crat) = crates[mv[1] - 1].pop_front() {
                crates[mv[2] - 1].push_front(new_crat);
            }
        });
    });

    crates
        .into_iter()
        .filter(|crat| crat.len() > 0)
        .map(|mut crat| crat.pop_front().unwrap())
        .collect()
}

pub fn super_puper_rearranging_crates(input: &str) -> String {
    let (mut crates, moves) = parse_data(input);

    moves.into_iter().for_each(|mv| {
        let mut temp = Vec::with_capacity(mv[0]);
        (0..mv[0]).for_each(|_| {
            if let Some(new_crat) = crates[mv[1] - 1].pop_front() {
                temp.push(new_crat);
            }
        });
        temp.into_iter()
            .rev()
            .for_each(|crat| crates[mv[2] - 1].push_front(crat));
    });

    crates
        .into_iter()
        .filter(|crat| crat.len() > 0)
        .map(|mut crat| crat.pop_front().unwrap())
        .collect()
}

pub fn parse_data(input: &str) -> (Vec<VecDeque<char>>, Vec<Vec<usize>>) {
    let (crates_part, moves_part) = input.split_once("\n\n").unwrap();

    let mut crates = vec![VecDeque::new(); crates_part.len() / crates_part.lines().count() / 3];
    let crate_matcher = Regex::new(r"\p{L}").unwrap();
    crates_part.lines().for_each(|line| {
        crate_matcher.find_iter(line).for_each(|crat| {
            let char = crat.as_str().chars().next().unwrap();
            crates[(crat.start() + 2 - ((crat.start() + 2) / 4)) / 3 - 1].push_back(char);
        });
    });

    let move_matcher = Regex::new(r"\d+").unwrap();
    let moves = moves_part
        .lines()
        .map(|line| {
            move_matcher
                .find_iter(line)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (crates, moves)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = rearrange_crates(&example_input);
        let res1 = rearrange_crates(&input);
        assert_eq!(res1_example, String::from("CMZ"));
        assert_eq!(res1, String::from("LBLVVTVLP"));

        let res2_example = super_puper_rearranging_crates(&example_input);
        let res2 = super_puper_rearranging_crates(&input);
        assert_eq!(res2_example, String::from("MCD"));
        assert_eq!(res2, String::from("TPFFBDRJD"));
    }
}
