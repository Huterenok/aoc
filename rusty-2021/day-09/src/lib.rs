use std::collections::HashSet;

fn find_lowest_points(input: &str) -> i32 {
    let heights = parse_heights(input);

    let mut res = 0;
    for y in 0..heights.len() as i32 {
        for x in 0..heights[0].len() as i32 {
            if [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter_map(|&(x, y)| heights.get(y as usize).and_then(|arr| arr.get(x as usize)))
                .all(|n| *n > heights[y as usize][x as usize])
            {
                res += heights[y as usize][x as usize] + 1;
            }
        }
    }
    res
}

fn find_basins(input: &str) -> usize {
    let heights = parse_heights(&input);
    let mut possible_basins = heights
        .into_iter()
        .enumerate()
        .flat_map(|(y, arr)| arr.into_iter().enumerate().map(move |(x, num)| (x, y, num)))
        .filter_map(|(x, y, num)| {
            if num != 9 {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let mut basins = Vec::new();
    while let Some(&possible_basin) = possible_basins.iter().next() {
        basins.push(measure_basin(possible_basin, &mut possible_basins))
    }

    basins.sort();
    basins.into_iter().rev().take(3).product()
}

fn measure_basin((x, y): (i32, i32), possible_basins: &mut HashSet<(i32, i32)>) -> usize {
    if !possible_basins.remove(&(x, y)) {
        return 0;
    }
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .map(|possible_basin| measure_basin(possible_basin, possible_basins))
        .sum::<usize>()
        + 1
}

fn parse_heights(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
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

        let res1_example = find_lowest_points(&example_input);
        let res1 = find_lowest_points(&input);
        assert_eq!(res1_example, 15);
        assert_eq!(res1, 506);

        let res2_example = find_basins(&example_input);
        let res2 = find_basins(&input);
        assert_eq!(res2_example, 1134);
        assert_eq!(res2, 931200);
    }
}
