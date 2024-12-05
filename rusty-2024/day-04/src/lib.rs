pub fn count_xmasses(input: &str) -> usize {
    count_pattern_matches_v_bukavkax(
        input,
        'X',
        [
            // M        A       S
            [(0, 1), (0, 2), (0, 3)],
            [(1, 1), (2, 2), (3, 3)],
            [(1, 0), (2, 0), (3, 0)],
            [(1, -1), (2, -2), (3, -3)],
            [(0, -1), (0, -2), (0, -3)],
            [(-1, -1), (-2, -2), (-3, -3)],
            [(-1, 0), (-2, 0), (-3, 0)],
            [(-1, 1), (-2, 2), (-3, 3)],
        ],
        ['M', 'A', 'S'],
    )
}

pub fn count_x_masses(input: &str) -> usize {
    count_pattern_matches_v_bukavkax(
        input,
        'A',
        [
            // M         S         M        S
            [(-1, -1), (1, 1), (1, -1), (-1, 1)],
            [(1, -1), (-1, 1), (1, 1), (-1, -1)],
            [(1, 1), (-1, -1), (-1, 1), (1, -1)],
            [(-1, 1), (1, -1), (-1, -1), (1, 1)],
        ],
        ['M', 'S', 'M', 'S'],
    )
}

pub fn count_pattern_matches_v_bukavkax<const Y: usize, const X: usize>(
    input: &str,
    base_bukavka: char,
    offsets_from_base: [[(i32, i32); X]; Y],
    bukavki_to_match: [char; X],
) -> usize {
    // Too lazy to use vec2
    let bukavki = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| c.eq(&base_bukavka).then_some((x as i32, y as i32)))
        })
        .map(|(x, y)| {
            offsets_from_base
                .iter()
                .filter(|offsets| {
                    offsets.iter().all(|(offset_x, offset_y)| {
                        let (cx, cy) = (offset_x + x, offset_y + y);
                        cx >= 0
                            && cx < bukavki[0].len() as i32
                            && cy >= 0
                            && cy < bukavki.len() as i32
                    })
                })
                .filter(|offsets| {
                    bukavki_to_match
                        .iter()
                        .zip(offsets.iter())
                        .all(|(c, (offset_x, offset_y))| {
                            let (cy, cx) = (y + offset_y, x + offset_x);
                            bukavki[cy as usize][cx as usize] == *c
                        })
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    const REAL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        let res = count_xmasses(TEST_INPUT);
        assert_eq!(res, 18);

        let real_res = count_xmasses(REAL_INPUT);
        assert_eq!(real_res, 2414);
    }

    #[test]
    fn part_2() {
        let res = count_x_masses(TEST_INPUT);
        assert_eq!(res, 9);

        let real_res = count_x_masses(REAL_INPUT);
        assert_eq!(real_res, 1871);
    }
}
