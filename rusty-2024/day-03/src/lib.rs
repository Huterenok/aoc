const DONT_EXPR: &str = "don't()";
const DO_EXPR: &str = "do()";
const MUL_EXPR_LEFT_PART: &str = "mul(";

pub fn multiply_dat(input: &str) -> usize {
    (0..input.len())
        .filter(|&idx| {
            input
                .get(idx..=idx + 3)
                .map(|prefix| prefix.starts_with(MUL_EXPR_LEFT_PART))
                .unwrap_or(false)
        })
        .filter_map(|idx| {
            input
                .get(idx..)
                .and_then(|expr| parse_mul_expr(expr).map(|(num1, num2)| num1 * num2))
        })
        .sum()
}

pub fn multiply_enabled_dat(input: &str) -> usize {
    (0..input.len())
        .fold((true, 0), |(prev_enabled, sum), idx| {
            let is_mul_expr = input
                .get(idx..idx + MUL_EXPR_LEFT_PART.len())
                .map(|prefix| prefix.starts_with(MUL_EXPR_LEFT_PART))
                .unwrap_or(false);
            let is_next_disabled = input
                .get(idx..idx + DONT_EXPR.len())
                .and_then(|prefix| prefix.starts_with(DONT_EXPR).then_some(false));
            let is_next_enabled = input
                .get(idx..idx + DO_EXPR.len())
                .and_then(|prefix| prefix.starts_with(DO_EXPR).then_some(true));

            let mul = (is_mul_expr && prev_enabled)
                .then(|| {
                    input
                        .get(idx..)
                        .and_then(|expr| parse_mul_expr(expr).map(|(num1, num2)| num1 * num2))
                })
                .flatten()
                .unwrap_or(0);

            (
                is_next_disabled.or(is_next_enabled).unwrap_or(prev_enabled),
                sum + mul,
            )
        })
        .1
}

pub fn parse_mul_expr(expr: &str) -> Option<(usize, usize)> {
    let (mul1, mul2) = expr
        .get(MUL_EXPR_LEFT_PART.len()..)
        .and_then(|expr| expr.split_once(','))?;

    let first_num = mul1.parse::<usize>().ok()?;
    let (end_idx, second_num) = mul2
        .as_bytes()
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .map(|b| (b - b'0') as usize)
        .enumerate()
        .reduce(|(_, num), (idx, digit)| (idx, num * 10 + digit))?;

    mul2.chars()
        .nth(end_idx + 1)
        .map(|c| c == ')')
        .and_then(|is_valid| is_valid.then_some((first_num, second_num)))
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        let res = multiply_dat(TEST_INPUT);
        assert_eq!(161, res);

        let real_res = multiply_dat(REAL_INPUT);
        assert_eq!(167650499, real_res);
    }

    #[test]
    fn part_2() {
        let res = multiply_enabled_dat(TEST_INPUT);
        assert_eq!(48, res);

        let real_res = multiply_enabled_dat(REAL_INPUT);
        assert_eq!(95846796, real_res);
    }
}
