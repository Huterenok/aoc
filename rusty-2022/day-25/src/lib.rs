fn find_supply(input: &str) -> String {
    let total_sum = input
        .lines()
        .fold(0, |acc, line| acc + parse_fckin_SNAFU(line));
    traverse_to_fckin_SNAFU(total_sum)
}

#[allow(non_snake_case)]
fn parse_fckin_SNAFU(SNAFU_num: &str) -> isize {
    let SNAFU_len = SNAFU_num.len() as u32 - 1;
    SNAFU_num.chars().enumerate().fold(0, |acc, (i, c)| {
        let num = c
            .to_digit(10)
            .map(|n| n as isize)
            .unwrap_or_else(|| match c {
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            });
        acc + num * 5isize.pow(SNAFU_len - i as u32)
    })
}

#[allow(non_snake_case)]
fn traverse_to_fckin_SNAFU(mut sum: isize) -> String {
    std::iter::from_fn(move || {
        if sum == 0 {
            return None;
        }

        let other = sum % 5;
        sum /= 5;

        if (0..=2).contains(&other) {
            Some(other.to_string())
        } else {
            sum += 1;
            Some(match other {
                3 => "=".to_string(),
                4 => "-".to_string(),
                _ => unreachable!(),
            })
        }
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
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

        let res1_example = find_supply(&example_input);
        let res1 = find_supply(&input);
        assert_eq!(res1_example, String::from("2=-1=0"));
        assert_eq!(res1, String::from("2=001=-2=--0212-22-2"))
    }
}
