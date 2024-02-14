use std::collections::{HashMap, HashSet};

fn find_paths_tuda_suda(input: &str) -> usize {
    let caves = parse_caves(input);
    move_through_caves("start", &mut Vec::new(), true, &caves)
}

fn find_paths_tuda_suda_tuda_suda(input: &str) -> usize {
    let caves = parse_caves(input);
    move_through_caves("start", &mut Vec::new(), false, &caves)
}

fn move_through_caves<'a>(
    prev_path: &'a str,
    visited: &mut Vec<&'a str>,
    mut visit_twice: bool,
    caves: &'a HashMap<&str, Vec<&str>>,
) -> usize {
    if prev_path == "end" {
        return 1;
    }

    if (prev_path.chars().next().unwrap() as u8) >= b'a' && visited.contains(&prev_path) {
        if visit_twice || prev_path == "start" {
            return 0;
        }
        visit_twice = true;
    }
    visited.push(prev_path);
    let res = caves
        .get(prev_path)
        .unwrap()
        .into_iter()
        .fold(0, |acc, next_path| {
            acc + move_through_caves(next_path, visited, visit_twice, caves)
        });
    visited.pop();
    res
}

fn parse_caves<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let (k, v) = line.split_once("-").unwrap();
        map.entry(k).or_default().push(v);
        map.entry(v).or_default().push(k);
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = find_paths_tuda_suda(&example_input);
        let res1 = find_paths_tuda_suda(&input);
        assert_eq!(res1_example, 226);
        assert_eq!(res1, 3708);

        let res2 = find_paths_tuda_suda_tuda_suda(&input);
        assert_eq!(res2, 93858);
    }
}
