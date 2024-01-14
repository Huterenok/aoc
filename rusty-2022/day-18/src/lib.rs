use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn find_outer_surface(input: &str) -> usize {
    let cubes = parse_cubes(input);
    cubes
        .iter()
        .flat_map(|cubic| gen_sides(*cubic))
        .filter(|side| !cubes.contains(side))
        .count()
}

fn find_exterior_surface(input: &str) -> usize {
    let cubes = parse_cubes(input);
    let max = *cubes.iter().flat_map(|(x, y, z)| [x, y, z]).max().unwrap() + 1;
    let mut visited = HashSet::new();
    let mut stack = vec![(0, 0, 0)];

    while let Some(s) = stack.pop() {
        gen_sides(s).into_iter().for_each(|(x, y, z)| {
            if !cubes.contains(&(x, y, z))
                && !visited.contains(&(x, y, z))
                && [x, y, z].iter().all(|&i| -1 <= i && i <= max)
            {
                visited.insert((x, y, z));
                stack.push((x, y, z));
            }
        });
    }

    cubes
        .iter()
        .flat_map(|&p| gen_sides(p))
        .filter(|s| visited.contains(s))
        .count()
}

fn gen_sides((x, y, z): (i8, i8, i8)) -> [(i8, i8, i8); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn parse_cubes(input: &str) -> HashSet<(i8, i8, i8)> {
    input
        .lines()
        .filter_map(|l| l.split(",").map(|s| s.parse().unwrap()).collect_tuple())
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

        let res1_example = find_outer_surface(&example_input);
        let res1 = find_outer_surface(&input);
        assert_eq!(res1_example, 64);
        assert_eq!(res1, 4608);

        let res2_example = find_exterior_surface(&example_input);
        let res2 = find_exterior_surface(&input);
        assert_eq!(res2_example, 58);
        assert_eq!(res2, 2652);
    }
}
