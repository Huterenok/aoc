use std::cmp::Ordering;

pub fn epsilon_x_gamma(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();
    let num_len = input.lines().next().unwrap().len();

    let gamma = (0..num_len)
        .map(|i| match find_bit(&nums, i) {
            Ordering::Greater => 1 << i,
            _ => 0,
        })
        .sum::<usize>();

    gamma * (!gamma & ((1 << num_len) - 1))
}

pub fn with_oxygen(input: &str, oxygen: usize) -> usize {
    let mut nums = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();
    let num_len = input.lines().next().unwrap().len();

    for i in (0..num_len).rev() {
        let bit = match find_bit(&nums, i) {
            Ordering::Equal => oxygen,
            Ordering::Less => 1 ^ oxygen,
            Ordering::Greater => 0 ^ oxygen,
        };
        nums.retain(|x| (x >> i) & 1 == bit);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

fn find_bit(nums: &[usize], bit: usize) -> Ordering {
    let (x, y) = nums.iter().fold((0, 0), |(x, y), num| {
        if (num >> bit) & 1 == 1 {
            (x + 1, y)
        } else {
            (x, y + 1)
        }
    });
    x.cmp(&y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = epsilon_x_gamma(&example_input);
        let res1 = epsilon_x_gamma(&input);
        assert_eq!(res1_example, 198);
        assert_eq!(res1, 2498354);

        let res2_example = with_oxygen(&example_input, 0) * with_oxygen(&example_input, 1);
        let res2 = with_oxygen(&input, 0) * with_oxygen(&input, 1);
        assert_eq!(res2_example, 230);
        assert_eq!(res2, 3277956);
    }
}
