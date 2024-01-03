pub fn cycle_cycles(input: &str) -> i32 {
    let commands = parse_commands(input);

    commands
        .into_iter()
        .fold((0, 1, 1), |(res, curr, cycle_count), command| {
            if let Command::Addx(c) = command {
                (
                    res + ((cycle_count + 20) % 40 == 0 || (cycle_count + 1 + 20) % 40 == 0) as i32
                        * curr
                        * (cycle_count + 1 * ((cycle_count + 1 + 20) % 40 == 0) as i32),
                    curr + c,
                    cycle_count + 2,
                )
            } else {
                (
                    res + ((cycle_count + 20) % 40 == 0) as i32 * curr * cycle_count,
                    curr,
                    cycle_count + 1,
                )
            }
        })
        .0
}

pub fn draw_cycles(input: &str, label: &str) {
    let mut image = Vec::with_capacity(40 * 6);
    let commands = parse_commands(input);

    commands
        .into_iter()
        .fold((1, 0), |(mut curr, mut cycle_count), command| {
            image.push(
                (curr - 1 <= cycle_count % 40 && curr + 1 >= cycle_count % 40) as u8 * 3 + 32,
            );
            cycle_count += 1;
            if let Command::Addx(c) = command {
                image.push(
                    (curr - 1 <= cycle_count % 40 && curr + 1 >= cycle_count % 40) as u8 * 3 + 32,
                );
                cycle_count += 1;
                curr += c;
            }
            (curr, cycle_count)
        });

    println!("{label}");
    image.chunks(40).for_each(|line| {
        println!(
            "{}",
            line.iter().map(|c| char::from(*c)).collect::<String>()
        );
    })
}

pub enum Command {
    Noop,
    Addx(i32),
}

pub fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            if line.contains("addx") {
                Command::Addx(line.split(" ").nth(1).unwrap().parse().unwrap())
            } else {
                Command::Noop
            }
        })
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

        let res1_example = cycle_cycles(&example_input);
        let res1 = cycle_cycles(&input);
        assert_eq!(res1_example, 13140);
        assert_eq!(res1, 14220);

        let res2_example = draw_cycles(&example_input, "EXAMPLE:");
        let res2 = draw_cycles(&input, "REAL:");
    }
}
