const DIRECTIONS: [(usize, usize); 4] = [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];

pub fn find_shortest_path(input: &str) -> usize {
    let (grid, width, height, start, end) = parse_grid(input);
    bfs_shortest_path(
        &(end % width, end / width),
        &grid,
        width,
        height,
        &(start % width, start / width),
    )
    .unwrap()
    .len()
        - 1
}

pub fn find_shortest_path_from_a(input: &str) -> usize {
    let (grid, width, height, _, end) = parse_grid(input);
    grid.iter()
        .enumerate()
        .filter(|(_, &val)| val == 0)
        .filter_map(|(idx, _)| {
            bfs_shortest_path(
                &(end % width, end / width),
                &grid,
                width,
                height,
                &(idx % width, idx / width),
            )
            .map(|path| path.len() - 1)
        })
        .min()
        .unwrap()
}

fn bfs_shortest_path(
    start: &(usize, usize),
    grid: &[u8],
    width: usize,
    height: usize,
    goal: &(usize, usize),
) -> Option<Vec<(usize, usize)>> {
    pathfinding::directed::bfs::bfs(
        start,
        |&(x, y)| {
            let cur = grid[y * width + x];
            DIRECTIONS
                .iter()
                .map(|&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
                .filter(|&(nx, ny)| {
                    nx < width && ny < height && grid[ny * width + nx] >= cur.saturating_sub(1)
                })
                .collect::<Vec<_>>()
        },
        |&p| p == *goal,
    )
}

pub fn parse_grid(input: &str) -> (Vec<u8>, usize, usize, usize, usize) {
    let mut grid: Vec<u8> = input
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let height = grid.len() / width;
    let mut start = input.bytes().position(|b| b == b'S').unwrap();
    let mut end = input.bytes().position(|b| b == b'E').unwrap();
    (start, end, grid[start], grid[end]) =
        (start - start / (width + 1), end - end / (width + 1), 0, 25);

    (grid, width, height, start, end)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = find_shortest_path(&example_input);
        let res1 = find_shortest_path(&input);
        assert_eq!(res1_example, 31);
        assert_eq!(res1, 447);

        let res2_example = find_shortest_path_from_a(&example_input);
        let res2 = find_shortest_path_from_a(&input);
        assert_eq!(res2_example, 29);
        assert_eq!(res2, 446);
    }
}
