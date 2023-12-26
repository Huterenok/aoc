use itertools::Itertools;

fn find_galaxy_distances(input: String, expansion_speed: usize) -> usize {
    let (galaxies, empty_rows, empty_cols) = retrieve_grid(input);
    galaxies
        .into_iter()
        .tuple_combinations()
        .map(|((y0, x0), (y1, x1))| {
            let expansion = empty_rows[y0.min(y1)..y0.max(y1)]
                .iter()
                .chain(&empty_cols[x0.min(x1)..x0.max(x1)])
                .filter(|empty| **empty)
                .count();
            y1.abs_diff(y0) + x1.abs_diff(x0) + expansion * expansion_speed
        })
        .sum()
}

fn retrieve_grid(input: String) -> (Vec<(usize, usize)>, Vec<bool>, Vec<bool>) {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().positions(|c| c == '#').map(move |x| (y, x)))
        .collect::<Vec<_>>();
    let (max_row, max_col) = galaxies
        .iter()
        .fold((0, 0), |(y0, x0), (y, x)| (y0.max(*y), x0.max(*x)));
    let mut empty_rows = vec![true; max_row + 1];
    let mut empty_cols = vec![true; max_col + 1];
    galaxies.iter().for_each(|(y, x)| {
        empty_rows[*y] = false;
        empty_cols[*x] = false;
    });

    (galaxies, empty_rows, empty_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yays() {
        let example_input = std::fs::read_to_string("../example_input.txt").unwrap();
        let input = std::fs::read_to_string("../input.txt").unwrap();

        let res1_example = find_galaxy_distances(example_input.clone(), 1);
        let res2_example = find_galaxy_distances(example_input, 999999);

        let res1 = find_galaxy_distances(input.clone(), 1);
        let res2 = find_galaxy_distances(input, 999999);

        assert_eq!(res1_example, 374);
        //assert_eq!(res2_example, ?);

        assert_eq!(res1, 10289334);
        assert_eq!(res2, 649862989626);
    }
}
