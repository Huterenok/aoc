use std::collections::HashMap;

use itertools::Itertools;

fn polymer_grow(input: &str, steps: usize) -> usize {
    let (template, map) = parse_polymers(input);

    let init_counter = template.chars().tuple_windows().counts();
    let counter = (0..steps).fold(init_counter, |curr_counter, _| {
        let mut next_counter = HashMap::new();
        for (&(a, b), count) in &curr_counter {
            let c = map[&(a, b)];
            *next_counter.entry((a, c)).or_insert(0) += count;
            *next_counter.entry((c, b)).or_insert(0) += count;
        }
        next_counter
    });

    let mut res_counter = HashMap::new();
    counter
        .iter()
        .for_each(|((a, _), c)| *res_counter.entry(a).or_insert(0) += c);
    let last_char = template.chars().last().unwrap();
    *res_counter.entry(&last_char).or_insert(0) += 1;
    let (min, max) = res_counter.values().minmax().into_option().unwrap();
    max - min
}

// fn polymer_grow(input: &str, steps: usize) -> usize {
// 	let (template, map) = parse_polymers(input);

// 	// XD
// 	let res = (0..steps).fold(template, |curr_template, _| {
// 			let res1 = (0..curr_template.len() - 1).fold(String::new(), |acc, curr_i| {
// 					acc + &curr_template[curr_i..=curr_i]
// 							+ map.get(&curr_template[curr_i..=curr_i + 1]).unwrap()
// 			});
// 			res1 + &curr_template.chars().last().unwrap().to_string()
// 	});

// 	let mut counter = HashMap::new();
// 	res.chars()
// 			.for_each(|c| *counter.entry(c).or_insert(0) += 1);

// 	counter.values().max().unwrap() - counter.values().min().unwrap()
// }

fn parse_polymers(input: &str) -> (String, HashMap<(char, char), char>) {
    let (template, map) = input.split_once("\n\n").unwrap();
    let map = map
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            (
                (from.chars().next().unwrap(), from.chars().nth(1).unwrap()),
                to.chars().next().unwrap(),
            )
        })
        .collect();
    (template.to_string(), map)
}

// fn parse_polymers(input: &str) -> (String, HashMap<&str, &str>) {
// 	let (template, map) = input.split_once("\n\n").unwrap();
// 	let map = map
// 			.lines()
// 			.map(|line| line.split_once(" -> ").unwrap())
// 			.collect();
// 	(template.to_string(), map)
// }

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = polymer_grow(&example_input, 10);
        let res1 = polymer_grow(&input, 10);
        assert_eq!(res1_example, 1588);
        assert_eq!(res1, 3906);

        let res2_example = polymer_grow(&example_input, 40);
        let res2 = polymer_grow(&input, 40);
        assert_eq!(res2_example, 2188189693529);
        assert_eq!(res2, 4441317262452);
    }
}
