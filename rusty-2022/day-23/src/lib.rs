use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn move_da_elves(input: &str) -> usize {
    let mut elves = parse_elves(input);
    let mut moves = HashMap::<_, Vec<_>>::with_capacity(10000);
    for t in 0..10 {
        for &(x, y) in &elves {
            let ns = [
                elves.contains(&(x - 1, y - 1)),
                elves.contains(&(x - 1, y)),
                elves.contains(&(x - 1, y + 1)),
                elves.contains(&(x, y - 1)),
                elves.contains(&(x, y + 1)),
                elves.contains(&(x + 1, y - 1)),
                elves.contains(&(x + 1, y)),
                elves.contains(&(x + 1, y + 1)),
            ];
            if ns.iter().all(|b| !b) {
                continue;
            }
            let props = [
                (!ns[0] && !ns[1] && !ns[2], (x - 1, y)),
                (!ns[5] && !ns[6] && !ns[7], (x + 1, y)),
                (!ns[0] && !ns[3] && !ns[5], (x, y - 1)),
                (!ns[2] && !ns[4] && !ns[7], (x, y + 1)),
            ];
            for i in 0..4 {
                let (free, pos) = props[(t + i) % 4];
                if free {
                    moves.entry(pos).or_default().push((x, y));
                    break;
                }
            }
        }
        for (pos, props) in moves.drain() {
            if props.len() == 1 {
                elves.remove(&props[0]);
                elves.insert(pos);
            }
        }
    }

    let (&minx, &maxx) = elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (&miny, &maxy) = elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    (minx..=maxx)
        .cartesian_product(miny..=maxy)
        .filter(|p| !elves.contains(p))
        .count()
}

pub fn maybe_break(input: &str) -> usize {
    let mut elves = parse_elves(input);
    let mut moves = HashMap::<_, Vec<_>>::with_capacity(10000);

    for t in 0.. {
        for &(x, y) in &elves {
            let ns = [
                elves.contains(&(x - 1, y - 1)),
                elves.contains(&(x - 1, y)),
                elves.contains(&(x - 1, y + 1)),
                elves.contains(&(x, y - 1)),
                elves.contains(&(x, y + 1)),
                elves.contains(&(x + 1, y - 1)),
                elves.contains(&(x + 1, y)),
                elves.contains(&(x + 1, y + 1)),
            ];
            if ns.iter().all(|b| !b) {
                continue;
            }
            let props = [
                (!ns[0] && !ns[1] && !ns[2], (x - 1, y)),
                (!ns[5] && !ns[6] && !ns[7], (x + 1, y)),
                (!ns[0] && !ns[3] && !ns[5], (x, y - 1)),
                (!ns[2] && !ns[4] && !ns[7], (x, y + 1)),
            ];
            for i in 0..4 {
                let (free, pos) = props[(t + i) % 4];
                if free {
                    moves.entry(pos).or_default().push((x, y));
                    break;
                }
            }
        }
        let mut moved = false;
        for (pos, props) in moves.drain() {
            if props.len() == 1 {
                moved = true;
                elves.remove(&props[0]);
                elves.insert(pos);
            }
        }
        if !moved {
            return t + 1;
        }
    }
    unreachable!()
}

pub fn parse_elves(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(y, _)| (x as i32, y as i32))
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

        let res1_example = move_da_elves(&example_input);
        let res1 = move_da_elves(&input);
        assert_eq!(res1_example, 110);
        assert_eq!(res1, 4025);

				let res2_example = maybe_break(&example_input);
        let res2 = maybe_break(&input);
        assert_eq!(res2_example, 20);
        assert_eq!(res2, 935);
    }
}
