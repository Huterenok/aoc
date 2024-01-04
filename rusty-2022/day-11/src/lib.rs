pub fn find_monkey_business_level(input: &str, rounds: usize) -> usize {
    let mut monkeys = parse_monkeys(input);
    let mut monkey_items = vec![vec![]; monkeys.len()];
    let maybe_lcm: usize = monkeys.iter().map(|monkey| monkey.test[0]).product();

    (0..rounds).for_each(|_| {
        monkeys.iter_mut().enumerate().for_each(|(i, monkey)| {
            monkey.items.append(&mut monkey_items[i]);

            monkey.items.drain(0..).for_each(|mut item| {
                item = (monkey.operation)(item) / ((rounds / 20 == 1).then_some(3).unwrap_or(1))
                    % maybe_lcm;

                monkey_items[if item % monkey.test[0] == 0 {
                    monkey.test[1]
                } else {
                    monkey.test[2]
                }]
                .push(item);

                monkey.seen += 1;
            });
        });
    });

    monkeys.sort_unstable_by(|a, b| b.seen.cmp(&a.seen));
    monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.seen)
        .product()
}

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut parts = monkey.split("  ").map(|p| p.replace("\n", ""));
            let items = parts
                .nth(1)
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();
            let operation = gen_operation(parts.next().unwrap().split_once("new = ").unwrap().1);
            let test = [
                parts
                    .next()
                    .unwrap()
                    .split_once("by ")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                parts.nth(1).unwrap().chars().last().unwrap() as u8 as usize - 48,
                parts.nth(1).unwrap().chars().last().unwrap() as u8 as usize - 48,
            ];

            Monkey {
                items,
                operation,
                test,
                seen: 0,
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

        let res1_example = find_monkey_business_level(&example_input, 20);
        let res1 = find_monkey_business_level(&input, 20);
        assert_eq!(res1_example, 10605);
        assert_eq!(res1, 117640);

        let res2_example = find_monkey_business_level(&example_input, 10000);
        let res2 = find_monkey_business_level(&input, 10000);
        assert_eq!(res2_example, 2713310158);
        assert_eq!(res2, 30616425600);
    }
}

pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub test: [usize; 3],
    pub seen: usize,
}

pub fn gen_operation(s: &str) -> Box<dyn Fn(usize) -> usize> {
    let parts = s.split(" ").collect::<Vec<&str>>();
    match parts[2] {
        "old" => Box::new(|old| old * old),
        b => match (parts[1], b.parse::<usize>().unwrap()) {
            ("+", n) => Box::new(move |old| old + n),
            ("*", n) => Box::new(move |old| old * n),
            _ => panic!("RETARDED"),
        },
    }
}
