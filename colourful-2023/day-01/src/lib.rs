struct Solution;

impl Solution {
    pub fn part1(input: &str) -> u32 {
        input
            .lines()
            .map(|line| line.chars().filter(|c| c.is_digit(10)))
            .fold(0, |acc, mut digits| {
                let first = digits.next().unwrap();
                let second = digits.last().unwrap_or(first);
                acc + first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap()
            })
    }

    pub fn part2(input: &str) -> u32 {
        let str_digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        input
            .lines()
            .map(|line| {
                let mut digits = (0..line.len()).filter_map(|i| {
                    let rline = &line[i..];

                    let d = str_digits
                        .iter()
                        .enumerate()
                        .find(|(_, str_digit)| rline.starts_with(*str_digit))
                        .map(|(num, _)| char::from_digit(num as u32 + 1, 10).unwrap())
                        .unwrap_or(rline.chars().next().unwrap());

                    d.to_digit(10)
                });

                let first = digits.next().unwrap();
                let second = digits.last().unwrap_or(first);

                first * 10 + second
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    fn setup_test() -> (String) {
        std::fs::read_to_string("./input.txt").unwrap()
    }

    #[test]
    fn yays() {
        let input = std::fs::read_to_string("./input.txt").unwrap();
        let example_input = std::fs::read_to_string("./example_input.txt").unwrap();

        assert_eq!(Solution::part1(&input), 53194);
        assert_eq!(Solution::part2(&input), 54249);
    }
}
