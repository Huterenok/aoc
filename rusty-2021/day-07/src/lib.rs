use itertools::Itertools;

fn crabik_move(input: &str) -> i32 {
    let (crabs, (min, max)) = parse_crabs(input);
    (min..=max)
        .map(|pos| crabs.iter().map(|crab_pos| (crab_pos - pos).abs()).sum())
        .min()
        .unwrap()
}

fn exponential_crabik_move(input: &str) -> i32 {
    let (crabs, (min, max)) = parse_crabs(input);
    (min..=max)
        .map(|pos| {
            crabs
                .iter()
                .map(|&crab_pos| progression_sum((pos - crab_pos).abs()))
                .sum()
        })
        .min()
        .unwrap()
}

fn progression_sum(num: i32) -> i32 {
    num * (num + 1) / 2
}

fn parse_crabs(input: &str) -> (Vec<i32>, (i32, i32)) {
    let crabs = input
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let minmax = crabs.iter().copied().minmax().into_option().unwrap();
    (crabs, minmax)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = crabik_move(&example_input);
        let res1 = crabik_move(&input);
        assert_eq!(res1_example, 37);
        assert_eq!(res1, 328318);

        let res2_example = exponential_crabik_move(&example_input);
        let res2 = exponential_crabik_move(&input);
        assert_eq!(res2_example, 168);
        assert_eq!(res2, 89791146);
    }
}
