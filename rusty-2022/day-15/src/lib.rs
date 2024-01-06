use regex::Regex;

// *boom*

pub fn no_beacon_in_row(input: &str, y: isize) -> isize {
    let bs = parse_beacons_and_sensors(input);
    let mut ranges = bs
        .into_iter()
        .filter(|((sx, sy), (bx, by))| sy.abs_diff(y) <= sx.abs_diff(*bx) + sy.abs_diff(*by))
        .map(|((sx, sy), (bx, by))| {
            let rd = ((sx.abs_diff(bx) + sy.abs_diff(by)) - sy.abs_diff(y)) as isize;
            sx - rd..=sx + rd
        })
        .collect::<Vec<_>>();

    let (mut merged, mut merging) = (vec![], ranges.pop().unwrap());
    loop {
        merging = match ranges
            .iter()
            .position(|other| merging.contains(other.start()) || merging.contains(other.end()))
        {
            Some(pos) => {
                let other = ranges.remove(pos);
                *(merging.start().min(other.start()))..=*(merging.end().max(other.end()))
            }
            None => {
                merged.push(merging);
                match ranges.pop() {
                    Some(cur) => cur,
                    None => break,
                }
            }
        }
    }

    merged.into_iter().map(|r| *r.end() - *r.start()).sum()
}

pub fn find_tuning_freq(input: &str) -> isize {
    let bs = parse_beacons_and_sensors(input);
    let distress = bs
        .iter()
        .flat_map(|((sx, sy), (bx, by))| {
            let d = (sx.abs_diff(*bx) + sy.abs_diff(*by)) as isize;
            (0..=d)
                .map(move |i| (sx + i, sy - d - 1 + i))
                .chain((0..=d).map(move |i| (sx - i, sy + d + 1 - i)))
                .chain((0..=d).map(move |i| (sx - d - 1 + i, sy - i)))
                .chain((0..=d).map(move |i| (sx + d + 1 - i, sy + i)))
        })
        .filter(|(x, y)| (0..4000000).contains(x) && (0..4000000).contains(y))
        .find(|(x, y)| {
            !bs.iter().any(|((sx, sy), (bx, by))| {
                sx.abs_diff(*x) + sy.abs_diff(*y) <= (sx.abs_diff(*bx) + sy.abs_diff(*by))
            })
        })
        .unwrap();

    distress.0 as isize * 4000000 + distress.1 as isize
}

pub fn parse_beacons_and_sensors(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    let re = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let matches: Vec<isize> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            ((matches[0], matches[1]), (matches[2], matches[3]))
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

        let res1_example = no_beacon_in_row(&example_input, 10);
        let res1 = no_beacon_in_row(&input, 2000000);
        assert_eq!(res1_example, 26);
        assert_eq!(res1, 5809294);

        let res2 = find_tuning_freq(&input);
        assert_eq!(res2, 10693731308112)
    }
}
