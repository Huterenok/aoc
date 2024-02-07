fn attack_ocean_with_fish_in_80days(input: &str) -> usize {
    let fish = parse_fish(input);
    fish.into_iter()
        .map(|fish| lets_multiply(fish, 80, 1))
        .sum()
}

fn attack_ocean_with_fish_in_256days(input: &str) -> usize {
    let fish = parse_fish(input);
    let mut count = [0; 9];
    fish.iter().for_each(|&day| count[day] += 1);

    for _ in 0..256 {
        count[7] += count[0];
        count.rotate_left(1);
    }

    count.into_iter().sum()
}

fn lets_multiply(fish_days: usize, days: usize, fish: usize) -> usize {
    if days == 0 {
        return fish;
    }
    match fish_days {
        0 => lets_multiply(6, days - 1, fish + 1) + lets_multiply(8, days - 1, 0),
        _ => lets_multiply(fish_days - 1, days - 1, fish),
    }
}

fn parse_fish(input: &str) -> Vec<usize> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = attack_ocean_with_fish_in_80days(&example_input);
        let res1 = attack_ocean_with_fish_in_80days(&input);
        assert_eq!(res1_example, 5934);
        assert_eq!(res1, 391888);

        let res2_example = attack_ocean_with_fish_in_256days(&example_input);
        let res2 = attack_ocean_with_fish_in_256days(&input);
        assert_eq!(res2_example, 26984457539);
        assert_eq!(res2, 1754597645339);
    }
}
