use std::collections::hash_map::RandomState;
use std::collections::HashSet;

pub fn prioritize_items(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (left, right) = line.split_at(line.len() / 2);

        let left: HashSet<char, RandomState> = HashSet::from_iter(left.chars());
        let right: HashSet<char, RandomState> = HashSet::from_iter(right.chars());
        acc + char_to_product(*left.intersection(&right).next().unwrap())
    })
}

pub fn find_group_items(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |acc, group| {
            let first: HashSet<char, RandomState> = HashSet::from_iter(group[0].chars());
            let second: HashSet<char, RandomState> = HashSet::from_iter(group[1].chars());

            let matched = group[2]
                .chars()
                .find(|x| first.contains(x) && second.contains(x))
                .unwrap();
            acc + char_to_product(matched)
        })
}

pub fn char_to_product(c: char) -> usize {
    if c as u8 >= 97 {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = prioritize_items(&example_input);
        let res1 = prioritize_items(&input);
        assert_eq!(res1_example, 157);
        assert_eq!(res1, 8123);

        let res2_example = find_group_items(&example_input);
        let res2 = find_group_items(&input);
        assert_eq!(res2_example, 70);
        assert_eq!(res2, 2620);
    }
}
