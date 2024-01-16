fn decrypt(input: &str) -> i64 {
    let nums = parse_nums(input);

    let mut res = (0..nums.len()).collect::<Vec<_>>();
    for (i, &x) in nums.iter().enumerate() {
        let pos = res.iter().position(|&y| y == i).unwrap();
        res.remove(pos);
        let new_i = if pos as i64 + x < 0 {
            res.len() - ((pos as i64 + x).abs() as usize % res.len())
        } else {
            (pos as i64 + x).abs() as usize % res.len()
        } as usize;
        res.insert(new_i, i);
    }

    let orig_zero_i = nums.iter().position(|&i| i == 0).unwrap();
    let zero_i = res.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[res[(zero_i + i) % res.len()]])
        .sum()
}

fn full_decypt(input: &str) -> i64 {
    let nums: Vec<i64> = parse_nums(input)
        .into_iter()
        .map(|n| n * 811589153)
        .collect();

    let mut res = (0..nums.len()).collect::<Vec<_>>();
    for _ in 0..10 {
        for (i, &x) in nums.iter().enumerate() {
            let pos = res.iter().position(|&y| y == i).unwrap();
            res.remove(pos);
            let new_i = if pos as i64 + x < 0 {
                res.len() - ((pos as i64 + x).abs() as usize % res.len())
            } else {
                (pos as i64 + x).abs() as usize % res.len()
            } as usize;
            res.insert(new_i, i);
        }
    }

    let orig_zero_i = nums.iter().position(|&i| i == 0).unwrap();
    let zero_i = res.iter().position(|&i| i == orig_zero_i).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[res[(zero_i + i) % res.len()]])
        .sum()
}

fn parse_nums(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = decrypt(&example_input);
        let res1 = decrypt(&input);
        assert_eq!(res1_example, 3);
        assert_eq!(res1, 6712);

        let res2_example = full_decypt(&example_input);
        let res2 = full_decypt(&input);
        assert_eq!(res2_example, 1623178306);
        assert_eq!(res2, 1595584274798);
    }
}
