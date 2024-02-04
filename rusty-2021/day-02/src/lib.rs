pub fn move_da_submarine(input: &str) -> usize {
    let (x, y) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<usize>().unwrap()))
        .fold((0, 0), |(x, y), (dir, steps)| {
            match dir.bytes().next().unwrap() {
                b'f' => (x + steps, y),
                b'd' => (x, y + steps),
                b'u' => (x, y - steps),
                _ => unreachable!(),
            }
        });

    x * y
}

pub fn i_have_aim(input: &str) -> usize {
    let (x, y, _) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, steps)| (dir, steps.parse::<usize>().unwrap()))
        .fold((0, 0, 0), |(x, y, aim), (dir, steps)| {
            match dir.bytes().next().unwrap() {
                b'f' => (x + steps, y + steps * aim, aim),
                b'd' => (x, y, aim + steps),
                b'u' => (x, y, aim - steps),
                _ => unreachable!(),
            }
        });

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = move_da_submarine(&example_input);
        let res1 = move_da_submarine(&input);
        assert_eq!(res1_example, 150);
        assert_eq!(res1, 2070300);

        let res2_example = i_have_aim(&example_input);
        let res2 = i_have_aim(&input);
        assert_eq!(res2_example, 900);
        assert_eq!(res2, 2078985210);
    }
}
