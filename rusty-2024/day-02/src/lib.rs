use itertools::Itertools;

pub fn count_safe_reports(input: &str) -> usize {
    let reports = parse_reports(input);

    reports
        .iter()
        .filter(|report| {
            let flow = (report[1] - report[0]).signum();
            report.windows(2).all(|levels| {
                (1..=3).contains(&(levels[1] - levels[0]).abs())
                    && (levels[1] - levels[0]).signum() == flow
            })
        })
        .count()
}

pub fn count_safe_reports_with_dampener(input: &str) -> usize {
    let reports = parse_reports(input);

    reports
        .iter()
        .filter(|report| {
            (-1..report.len() as i32).any(|skip_idx| {
                let flow = if skip_idx != 0 {
                    (report[1] - report[0]).signum()
                } else {
                    (report[2] - report[1]).signum()
                };

                report
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx as i32 != skip_idx)
                    .tuple_windows()
                    .all(|((_, level1), (_, level2))| {
                        (1..=3).contains(&(level2 - level1).abs())
                            && (level2 - level1).signum() == flow
                    })
            })
        })
        .count()
}

// Too lazy to use vec2
fn parse_reports(input: &str) -> Box<[Box<[i64]>]> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|id| id.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        let res = count_safe_reports(TEST_INPUT);
        assert_eq!(2, res);

        let real_res = count_safe_reports(REAL_INPUT);
        assert_eq!(224, real_res);
    }

    #[test]
    fn part_2() {
        let res = count_safe_reports_with_dampener(TEST_INPUT);
        assert_eq!(4, res);

        let real_res = count_safe_reports_with_dampener(REAL_INPUT);
        assert_eq!(293, real_res);
    }
}
