use std::collections::HashMap;

use regex::Regex;

fn can_elephants_speak_with_monkeys(input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    exec("root", &monkeys)
}

fn no_they_cant(input: &str) {
    let monkeys = parse_monkeys(input);
    let Monkey::Operation(first, second, _) = monkeys.get("root").unwrap() else {
        panic!()
    };
    println!(
        "{} = {}",
        exec(second, &monkeys),
        print_equation(first, &monkeys)
    )
}

fn exec(monkey_name: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Operation(first, second, op) => op(exec(first, monkeys), exec(second, monkeys)),
        Monkey::Num(num) => *num,
        _ => panic!("RETARDED"),
    }
}

fn print_equation(monkey_name: &str, monkeys: &HashMap<&str, Monkey>) -> String {
    match monkeys.get(monkey_name).unwrap() {
        _ if monkey_name == "humn" => "x".into(),
        Monkey::Num(num) => num.to_string(),
        Monkey::Operation(first, second, op) => {
            format!(
                "({} {} {})",
                print_equation(first, monkeys),
                op_to_str(op),
                print_equation(second, monkeys)
            )
        }
        _ => panic!("RETARDED"),
    }
}

enum Monkey<'a> {
    Num(i64),
    Operation(&'a str, &'a str, Box<dyn Fn(i64, i64) -> i64>),
}

fn op_to_str(op: &Box<dyn Fn(i64, i64) -> i64>) -> String {
    (match op(5, 5) {
        25 => "*",
        1 => "/",
        10 => "+",
        0 => "-",
        _ => panic!("RETARDED"),
    })
    .into()
}

fn parse_monkeys(input: &str) -> HashMap<&str, Monkey> {
    let mut monkeys = HashMap::with_capacity(input.lines().count());

    let matcher = Regex::new(r"-?\d+").unwrap();
    input.lines().for_each(|line| {
        let (monkey_name, mov) = line.split_once(": ").unwrap();
        if let Some(num) = matcher.find(mov) {
            monkeys.insert(monkey_name, Monkey::Num(num.as_str().parse().unwrap()));
        } else {
            let mut op_parts = mov.split(" ");
            let first = op_parts.next().unwrap();
            let op = match op_parts.next().unwrap() {
                "+" => |a, b| a + b,
                "-" => |a, b| a - b,
                "*" => |a, b| a * b,
                _ => |a, b| a / b,
            };
            let second = op_parts.next().unwrap();
            monkeys.insert(monkey_name, Monkey::Operation(first, second, Box::new(op)));
        }
    });

    monkeys
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = can_elephants_speak_with_monkeys(&example_input);
        let res1 = can_elephants_speak_with_monkeys(&input);
        assert_eq!(res1_example, 152);
        assert_eq!(res1, 158661812617812);

        let res2_example = no_they_cant(&example_input);
        let res2 = no_they_cant(&input);
    }
}

use std::collections::HashMap;

use regex::Regex;

fn can_elephants_speak_with_monkeys(input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    exec("root", &monkeys)
}

fn no_they_cant(input: &str) {
    let monkeys = parse_monkeys(input);
    let Monkey::Operation(first, second, _) = monkeys.get("root").unwrap() else {
        panic!()
    };
    println!(
        "{} = {}",
        exec(second, &monkeys),
        print_equation(first, &monkeys)
    )
}

fn exec(monkey_name: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match monkeys.get(monkey_name).unwrap() {
        Monkey::Operation(first, second, op) => op(exec(first, monkeys), exec(second, monkeys)),
        Monkey::Num(num) => *num,
        _ => panic!("RETARDED"),
    }
}

fn print_equation(monkey_name: &str, monkeys: &HashMap<&str, Monkey>) -> String {
    match monkeys.get(monkey_name).unwrap() {
        _ if monkey_name == "humn" => "x".into(),
        Monkey::Num(num) => num.to_string(),
        Monkey::Operation(first, second, op) => {
            format!(
                "({} {} {})",
                print_equation(first, monkeys),
                op_to_str(op),
                print_equation(second, monkeys)
            )
        }
        _ => panic!("RETARDED"),
    }
}

enum Op<'a> {
    Num(i64),
    Op(&'a str, char, &'a str),
}
use itertools::Itertools;
fn val(monkies: &HashMap<&str, Op>, name: &str) -> i64 {
    match &monkies[name] {
        Op::Num(i) => *i,
        Op::Op(m1, '+', m2) => val(monkies, m1) + val(monkies, m2),
        Op::Op(m1, '-', m2) => val(monkies, m1) - val(monkies, m2),
        Op::Op(m1, '*', m2) => val(monkies, m1) * val(monkies, m2),
        Op::Op(m1, '/', m2) => val(monkies, m1) / val(monkies, m2),
        _ => unreachable!(),
    }
}

fn get_eq(monkies: &HashMap<&str, Op>, name: &str) -> String {
    match &monkies[name] {
        _ if name == "humn" => "x".to_string(),
        Op::Num(i) => i.to_string(),
        Op::Op(m1, op, m2) => format!("({} {} {})", get_eq(monkies, m1), op, get_eq(monkies, m2)),
    }
}

fn test(input: &str) {
    let monkies = input
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once(": ").unwrap();
            let op = rest
                .split(' ')
                .collect_tuple()
                .map(|(m1, op, m2)| Op::Op(m1, op.as_bytes()[0] as char, m2))
                .unwrap_or_else(|| Op::Num(rest.parse().unwrap()));
            (name, op)
        })
        .collect::<HashMap<_, _>>();
    let Op::Op(a, _, b) = monkies["root"] else {
        panic!()
    };
    println!("{} = {}", val(&monkies, b), get_eq(&monkies, a));
}

enum Monkey<'a> {
    Num(i64),
    Operation(&'a str, &'a str, Box<dyn Fn(i64, i64) -> i64>),
}

fn op_to_str(op: &Box<dyn Fn(i64, i64) -> i64>) -> String {
    (match op(5, 5) {
        25 => "*",
        1 => "/",
        10 => "+",
        0 => "-",
        _ => panic!("RETARDED"),
    })
    .into()
}

fn parse_monkeys(input: &str) -> HashMap<&str, Monkey> {
    let mut monkeys = HashMap::with_capacity(input.lines().count());

    let matcher = Regex::new(r"-?\d+").unwrap();
    input.lines().for_each(|line| {
        let (monkey_name, mov) = line.split_once(": ").unwrap();
        if let Some(num) = matcher.find(mov) {
            monkeys.insert(monkey_name, Monkey::Num(num.as_str().parse().unwrap()));
        } else {
            let mut op_parts = mov.split(" ");
            let first = op_parts.next().unwrap();
            let op = match op_parts.next().unwrap() {
                "+" => |a, b| a + b,
                "-" => |a, b| a - b,
                "*" => |a, b| a * b,
                _ => |a, b| a / b,
            };
            let second = op_parts.next().unwrap();
            monkeys.insert(monkey_name, Monkey::Operation(first, second, Box::new(op)));
        }
    });

    monkeys
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = can_elephants_speak_with_monkeys(&example_input);
        let res1 = can_elephants_speak_with_monkeys(&input);
        assert_eq!(res1_example, 152);
        assert_eq!(res1, 158661812617812);

        let res2_example = no_they_cant(&example_input);
        let res2 = no_they_cant(&input);
    }
}
