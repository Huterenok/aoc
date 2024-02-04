use itertools::Itertools;
pub fn count_increasing(input: &str) -> usize {
    input
        .lines()
        .map(|n| n.parse::<usize>().unwrap())
        .tuple_windows()
        .fold(0, |acc, (prev, x)| acc + if x > prev { 1 } else { 0 })
}

pub fn count_group_increasing(input: &str) -> usize {
    let nums: Vec<usize> = input.lines().map(|n| n.parse::<usize>().unwrap()).collect();

    nums.windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .fold(0, |acc, (prev_sum, sum)| {
            acc + if sum > prev_sum { 1 } else { 0 }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = count_increasing(&example_input);
        let res1 = count_increasing(&input);
        assert_eq!(res1_example, 7);
        assert_eq!(res1, 1139);

        let res2_example = count_group_increasing(&example_input);
        let res2 = count_group_increasing(&input);
        assert_eq!(res2_example, 5);
        assert_eq!(res2, 1103)
    }
}
