use std::collections::HashSet;

const MOVES: [(isize, isize); 5] = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

pub fn move_through_blizards(input: &str) -> i32 {
    let mut blizzards = parse_blizzards(input);
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());

    let mut pos = HashSet::from_iter([(0, 1)]);

    for len in 1.. {
        move_blizzards(&mut blizzards, width, height);
        pos = move_yourself(pos, &blizzards, width, height);

        if pos.contains(&(height - 1, width - 2)) {
            return len;
        }
    }
    unreachable!()
}

pub fn move_through_back_through_blizards(input: &str) -> i32 {
    let mut blizzards = parse_blizzards(input);
    let (width, height) = (input.lines().next().unwrap().len(), input.lines().count());
    let mut stage = 0;

    let mut pos = HashSet::from_iter([(0, 1)]);

    for len in 1.. {
        move_blizzards(&mut blizzards, width, height);
        pos = move_yourself(pos, &blizzards, width, height);

        match stage {
            0 => {
                if pos.contains(&(height - 1, width - 2)) {
                    pos = HashSet::from_iter([(height - 1, width - 2)]);
                    stage += 1;
                }
            }
            1 => {
                if pos.contains(&(0, 1)) {
                    pos = HashSet::from_iter([(0, 1)]);
                    stage += 1;
                }
            }
            2 => {
                if pos.contains(&(height - 1, width - 2)) {
                    return len;
                }
            }
            _ => unreachable!(),
        }
    }
    unreachable!()
}

pub fn move_blizzards(blizzards: &mut Vec<(usize, usize, u8)>, width: usize, height: usize) {
    for b in blizzards {
        match b.2 {
            b'>' => b.1 = if b.1 == width - 2 { 1 } else { b.1 + 1 },
            b'<' => b.1 = if b.1 == 1 { width - 2 } else { b.1 - 1 },
            b'v' => b.0 = if b.0 == height - 2 { 1 } else { b.0 + 1 },
            b'^' => b.0 = if b.0 == 1 { height - 2 } else { b.0 - 1 },
            _ => unreachable!(),
        }
    }
}

pub fn move_yourself(
    cur_pos: HashSet<(usize, usize)>,
    blizzards: &Vec<(usize, usize, u8)>,
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let blizzard_pos = blizzards
        .iter()
        .map(|&(x, y, _)| (x, y))
        .collect::<HashSet<_>>();
    let mut next_pos = HashSet::with_capacity(cur_pos.len());
    for &(x, y) in &cur_pos {
        for (dx, dy) in MOVES {
            if (x == 0 && dx == -1) || (x == height - 1 && dx == 1) {
                continue;
            }
            let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if (x != 0 || y == 1)
                && (x != height - 1 || y == width - 2)
                && y != 0
                && y != width - 1
                && !blizzard_pos.contains(&(x, y))
            {
                next_pos.insert((x, y));
            }
        }
    }
    next_pos
}

pub fn parse_blizzards(input: &str) -> Vec<(usize, usize, u8)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(y, b)| b">v<^".contains(&b).then_some((x, y, b)))
        })
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

        let res1_example = move_through_blizards(&example_input);
        let res1 = move_through_blizards(&input);
        assert_eq!(res1_example, 18);
        assert_eq!(res1, 299);

        let res2_example = move_through_back_through_blizards(&example_input);
        let res2 = move_through_back_through_blizards(&input);
        assert_eq!(res2_example, 54);
        assert_eq!(res2, 899)
    }
}
