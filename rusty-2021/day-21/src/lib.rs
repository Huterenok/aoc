use std::collections::HashMap;
use std::mem::swap;

type Cache = HashMap<(usize, usize, usize, usize), (usize, usize)>;

fn lets_play(input: &str) -> usize {
    let (mut start1, mut start2) = parse_config(input);
    let (mut pos1, mut pos2, mut die, mut rolls) = (0, 0, 1, 0);

    while pos2 < 1000 && pos1 < 1000 {
        for _ in 0..3 {
            start1 += die;
            die = div(die + 1, 100);
        }
        start1 = div(start1, 10);
        pos1 += start1;
        rolls += 3;
        swap(&mut pos1, &mut pos2);
        swap(&mut start1, &mut start2);
    }
    rolls * pos1.min(pos2)
}

fn lets_play_quantumgame(input: &str) -> usize {
    let (mut start1, mut start2) = parse_config(input);
    let (s1, s2) = quantum_game(&mut HashMap::new(), 0, 0, start1, start2);
    s1.max(s2)
}

fn quantum_game(
    cache: &mut Cache,
    s1: usize,
    s2: usize,
    pos1: usize,
    pos2: usize,
) -> (usize, usize) {
    if s2 >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(s1, s2, pos1, pos2)) {
        return score;
    }

    let mut score = (0, 0);
    for (die, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let pos1 = 1 + (pos1 + die - 1) % 10;
        let (s1, s2) = quantum_game(cache, s2, s1 + pos1, pos2, pos1);
        score = (score.0 + s2 * times, score.1 + s1 * times);
    }

    cache.insert((s1, s2, pos1, pos2), score);
    score
}

fn div(num: usize, div: usize) -> usize {
    let res = num % div;
    if res == 0 {
        div
    } else {
        res
    }
}

fn parse_config(input: &str) -> (usize, usize) {
    let nums = input
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse().unwrap())
        .collect::<Vec<_>>();
    (nums[0], nums[1])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{lets_play, lets_play_quantumgame};

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("example_input.txt").unwrap();
        let input = fs::read_to_string("input.txt").unwrap();

        let res1_example = lets_play(&example_input);
        let res1 = lets_play(&input);
        assert_eq!(res1_example, 739785);
        assert_eq!(res1, 920079);

        let res2_example = lets_play_quantumgame(&example_input);
        let res2 = lets_play_quantumgame(&input);
        assert_eq!(res2_example, 444356092776315);
        assert_eq!(res2, 56852759190649)
    }
}
