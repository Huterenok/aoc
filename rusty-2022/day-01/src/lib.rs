pub fn find_the_fattest_elf(input: &str) -> usize {
    input.split("\n\n").fold(0, |acc, ration| {
        acc.max(
            ration
                .split("\n")
                .map(|x| x.parse::<usize>().unwrap())
                .sum(),
        )
    })
}

pub fn find_three_fattest_elves(input: &str) -> usize {
    let mut sorted_elfs: Vec<usize> = input
        .split("\n\n")
        .map(|ration| {
            ration
                .split("\n")
                .map(|x| x.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    sorted_elfs.sort_unstable();
    sorted_elfs.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = find_the_fattest_elf(&example_input);
        let res1 = find_the_fattest_elf(&input);
        assert_eq!(res1_example, 24000);
        assert_eq!(res1, 71934);

        let res2_example = find_three_fattest_elves(&example_input);
        let res2 = find_three_fattest_elves(&input);
        assert_eq!(res2_example, 45000);
        assert_eq!(res2, 211447);
    }
}
