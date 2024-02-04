pub fn find_gamma_and_epsilon(input: &str) -> usize {
    let bits = input
        .lines()
        .flat_map(|line| line.chars().map(|s| s.to_digit(10).unwrap()))
        .collect::<Vec<_>>();
    let rate_len = input.lines().next().unwrap().len();
    let rate_count = input.lines().count();

    let (gamma, epsilon) = (0..rate_len).fold((0, 0), |(g, e), col| {
        let bit = find_bit(&bits, rate_count, rate_len, col);
        if bit == 1 {
            (g + (1 << (rate_len - col - 1)) as usize, e)
        } else {
            (g, e + (1 << (rate_len - col - 1)) as usize)
        }
    });
    gamma * epsilon
}

//TODO
pub fn find_life_support_rating(input: &str, oxygen: u32) -> usize {
    let mut bits = input
        .lines()
        .flat_map(|line| line.chars().map(|s| s.to_digit(10).unwrap()))
        .collect::<Vec<_>>();
    let rate_len = input.lines().next().unwrap().len();

    for i in 0..rate_len {
        let bit = find_bit(&bits, bits.len() / rate_len, rate_len, i) ^ oxygen;
        bits = bits
            .chunks(rate_len)
            .filter(|b| b[i] == bit)
            .flatten()
            .copied()
            .collect();
        if bits.len() == rate_len {
            break;
        }
    }

    bits.into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + e as usize * (1 << i))
}

pub fn find_bit(bits: &Vec<u32>, rate_count: usize, rate_len: usize, col: usize) -> u32 {
    let power =
        (0..rate_count).fold(0, |stage_acc, idx| stage_acc + bits[col + rate_len * idx]) as usize;

    if power > rate_count / 2 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = find_gamma_and_epsilon(&example_input);
        let res1 = find_gamma_and_epsilon(&input);
        assert_eq!(res1_example, 198);
        assert_eq!(res1, 2498354);

        let res2_example = find_life_support_rating(&example_input, 1)
            * find_life_support_rating(&example_input, 0);
        let res2 = find_life_support_rating(&input, 0) * find_life_support_rating(&input, 1);
        assert_eq!(res2_example, 230);
        assert_eq!(res2, 3277956);
    }
}
