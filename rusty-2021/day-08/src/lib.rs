pub fn count_simple_nums(input: &str) -> usize {
    let displays = parse_displays(input);
    displays
        .into_iter()
        .flat_map(|(_, display)| {
            display
                .into_iter()
                .filter(|s| [2, 4, 3, 7].contains(&s.len()))
        })
        .count()
}

pub fn mmmmm_deduction(input: &str) -> usize {
    let displays = parse_displays(input);
    displays
        .into_iter()
        .map(|(test, display)| find_out_nums(&test, &display))
        .sum()
}

fn find_out_nums(test: &[&str], display: &[&str]) -> usize {
    let mut nums_repr = [""; 10];
    test.iter().for_each(|s| match s.len() {
        2 => nums_repr[1] = s,
        3 => nums_repr[7] = s,
        4 => nums_repr[4] = s,
        7 => nums_repr[8] = s,
        _ => {}
    });

    test.iter().filter(|s| s.len() == 6).for_each(|s| {
        if !nums_repr[1].chars().all(|c| s.contains(c)) {
            nums_repr[6] = s;
        } else if nums_repr[4].chars().all(|c| s.contains(c)) {
            nums_repr[9] = s;
        } else {
            nums_repr[0] = s;
        }
    });
    test.iter().filter(|s| s.len() == 5).for_each(|s| {
        if nums_repr[1].chars().all(|c| s.contains(c)) {
            nums_repr[3] = s;
        } else if s.chars().all(|c| nums_repr[6].contains(c)) {
            nums_repr[5] = s;
        } else {
            nums_repr[2] = s;
        }
    });

    let res = display.iter().fold(0, |acc, num| {
        acc * 10
            + nums_repr
                .iter()
                .position(|s| num.chars().all(|c| s.contains(c)) && s.len() == num.len())
                .unwrap()
    });

    res
}

fn parse_displays(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let (test, display) = line.split_once(" | ").unwrap();
            (
                test.split_ascii_whitespace().collect(),
                display.split_ascii_whitespace().collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = count_simple_nums(&example_input);
        let res1 = count_simple_nums(&input);
        assert_eq!(res1_example, 26);
        assert_eq!(res1, 288);

        let res2_example = mmmmm_deduction(&example_input);
        let res2 = mmmmm_deduction(&input);
        assert_eq!(res2_example, 61229);
        assert_eq!(res2, 940724);
    }
}
