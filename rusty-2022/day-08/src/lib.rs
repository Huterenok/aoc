pub fn count_visible_trees(input: &str) -> usize {
    let grid = parse_grid(input);
    grid.iter().enumerate().fold(0, |acc, (y, line)| {
        line.iter()
            .enumerate()
            .filter(|(x, tree)| {
                grid[y][0..*x].iter().all(|n_tree| n_tree < tree)
                    || grid[y][*x + 1..].iter().all(|n_tree| n_tree < tree)
                    || (0..y).all(|n_line_i| &grid[n_line_i][*x] < tree)
                    || (y + 1..grid[0].len()).all(|n_line_i| &grid[n_line_i][*x] < tree)
            })
            .count()
            + acc
    })
}

pub fn find_place_for_treehouse(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut res = 0;
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, tree)| {
            let (mut r, mut l, mut t, mut b) = (0, 0, 0, 0);

            for i in (0..x).rev() {
                l += 1;
                if grid[y][i] >= *tree {
                    break;
                }
            }
            for i in x + 1..grid.len() {
                r += 1;
                if grid[y][i] >= *tree {
                    break;
                }
            }
            for i in y + 1..grid.len() {
                b += 1;
                if grid[i][x] >= *tree {
                    break;
                }
            }
            for i in (0..y).rev() {
                t += 1;
                if grid[i][x] >= *tree {
                    break;
                }
            }

            res = res.max(r * l * t * b);
        });
    });

    res
}

pub fn parse_grid(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = count_visible_trees(&example_input);
        let res1 = count_visible_trees(&input);
        assert_eq!(res1_example, 21);
        assert_eq!(res1, 1695);

        let res2_example = find_place_for_treehouse(&example_input);
        let res2 = find_place_for_treehouse(&input);
        assert_eq!(res2_example, 8);
        assert_eq!(res2, 287040);
    }
}
