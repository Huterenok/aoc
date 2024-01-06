const HEIGHT: usize = 200;
const WIDTH: usize = 700;

pub fn when_da_sand_out(input: &str) -> usize {
    let (mut cave, _) = parse_cave(input);
    let mut res = 0;

    while let Some((x, y)) = pour(&cave, u64::MAX as usize) {
        cave[y][x] = CaveElement::Sand;
        res += 1;
    }

    res
}

pub fn when_da_sand_on_da_floor(input: &str) -> usize {
    let (mut cave, lowest) = parse_cave(input);
    let mut res = 0;
    while let Some((x, y)) = pour(&cave, lowest + 2) {
        if y == 0 {
            break;
        }
        cave[y][x] = CaveElement::Sand;
        res += 1;
    }

    res + 1
}

fn pour(cave: &[[CaveElement; WIDTH]; HEIGHT], lowest: usize) -> Option<(usize, usize)> {
    let (mut x_start, y_start) = (500, 0);
    (y_start..HEIGHT - 1)
        .find(|y| {
            *y == lowest - 1
                || [x_start, x_start - 1, x_start + 1]
                    .into_iter()
                    .find(|x| cave[*y + 1][*x] == CaveElement::Air)
                    .map(|x| x_start = x)
                    .is_none()
        })
        .map(|y| (x_start, y))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaveElement {
    Air,
    Rock,
    Sand,
    Source,
}

impl std::fmt::Display for CaveElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaveElement::Air => write!(f, "."),
            CaveElement::Rock => write!(f, "#"),
            CaveElement::Sand => write!(f, "o"),
            CaveElement::Source => write!(f, "+"),
        }
    }
}

pub fn parse_cave(input: &str) -> ([[CaveElement; WIDTH]; HEIGHT], usize) {
    let mut cave = [[CaveElement::Air; WIDTH]; HEIGHT];
    let mut lowest = 0;
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|cords| {
                    let (x, y) = cords.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .for_each(|rock_line| {
            rock_line.windows(2).for_each(|pair| {
                let ((x1, y1), (x2, y2)) = (pair[0], pair[1]);
                lowest = lowest.max(y1.max(y2));
                if x1 == x2 {
                    (y1.min(y2)..=y1.max(y2)).for_each(|y| cave[y][x1] = CaveElement::Rock);
                } else {
                    (x1.min(x2)..=x1.max(x2)).for_each(|x| cave[y1][x] = CaveElement::Rock);
                }
            })
        });
    (cave, lowest)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = when_da_sand_out(&example_input);
        let res1 = when_da_sand_out(&input);
        assert_eq!(res1_example, 24);
        assert_eq!(res1, 1298);

        let res2_example = when_da_sand_on_da_floor(&example_input);
        let res2 = when_da_sand_on_da_floor(&input);
        assert_eq!(res2_example, 93);
        assert_eq!(res2, 25585);
    }
}
