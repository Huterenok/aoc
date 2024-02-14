use std::collections::HashSet;

fn lets_lit(input: &str) -> u64 {
    let mut dumbos = parse_dumbos(input);

    (0..100).fold(0, |acc, _| {
        dumbos.iter_mut().flatten().for_each(|n| *n += 1);
        let mut litted = HashSet::new();
        for y in 0..dumbos.len() {
            for x in 0..dumbos[0].len() {
                lit(&mut dumbos, (x as u64, y as u64), &mut litted);
            }
        }
        dumbos.iter_mut().flatten().for_each(|n| {
            if *n > 9 {
                *n = 0;
            }
        });

        acc + litted.len() as u64
    })
}

fn lets_big_lit(input: &str) -> usize {
    let mut dumbos = parse_dumbos(input);

    (0..)
        .find(|_| {
            dumbos.iter_mut().flatten().for_each(|n| *n += 1);
            let mut litted = HashSet::new();
            for y in 0..dumbos.len() {
                for x in 0..dumbos[0].len() {
                    lit(&mut dumbos, (x as u64, y as u64), &mut litted);
                }
            }
            dumbos.iter_mut().flatten().for_each(|n| {
                if *n > 9 {
                    *n = 0;
                }
            });

            litted.len() == dumbos.iter().flatten().count()
        })
        .unwrap()
        + 1
}

fn lit(dumbos: &mut [Vec<u32>], (x, y): (u64, u64), litted: &mut HashSet<(u64, u64)>) {
    if dumbos[y as usize][x as usize] < 10 || !litted.insert((x, y)) {
        return;
    }

    for (new_x, new_y) in [
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y.wrapping_sub(1)),
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x.wrapping_sub(1), y + 1),
    ] {
        if new_x < dumbos[0].len() as u64 && new_y < dumbos.len() as u64 {
            dumbos[new_y as usize][new_x as usize] += 1;
            lit(dumbos, (new_x, new_y), litted);
        }
    }
}

fn parse_dumbos(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
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

        let res1_example = lets_lit(&example_input);
        let res1 = lets_lit(&input);
        assert_eq!(res1_example, 1656);
        assert_eq!(res1, 1655);

        let res2_example = lets_big_lit(&example_input);
        let res2 = lets_big_lit(&input);
        assert_eq!(res2_example, 195);
        assert_eq!(res2, 337);
    }
}
