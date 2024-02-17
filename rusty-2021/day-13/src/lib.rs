use std::collections::HashSet;

fn fold_paper(input: &str, peek_all: bool) -> usize {
    let (dots, folds) = parse_paper(input);
    let to_peek = if peek_all { folds.len() } else { 1 };

    let res = folds
        .into_iter()
        .take(to_peek)
        .fold(dots, |acc, (dir, fold)| {
            acc.into_iter()
                .map(|(x, y)| match (&dir, x, y) {
                    (Dir::X, x, y) if x > fold => (fold - (x - fold), y),
                    (Dir::Y, x, y) if y > fold => (x, fold - (y - fold)),
                    (_, other_x, other_y) => (other_x, other_y),
                })
                .collect()
        });

    let max_x = res.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = res.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut s = "\n".to_string();
    for y in 0..=max_y {
        for x in 0..=max_x {
            s.push_str(if res.contains(&(x, y)) { "#" } else { "." });
        }
        s.push_str("\n");
    }
    println!("{s}");

    res.len()
}

enum Dir {
    X,
    Y,
}

fn parse_paper(input: &str) -> (HashSet<(usize, usize)>, Vec<(Dir, usize)>) {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = folds
        .lines()
        .map(|line| {
            let (dir, num) = line.split_once("=").unwrap();
            let num = num.parse().unwrap();
            match dir.chars().last().unwrap() {
                'x' => (Dir::X, num),
                'y' => (Dir::Y, num),
                _ => unreachable!(),
            }
        })
        .collect();
    (dots, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = fold_paper(&example_input, false);
        let res1 = fold_paper(&input, false);
        assert_eq!(res1_example, 17);
        assert_eq!(res1, 708);

        let res2_example = fold_paper(&example_input, true);
        let res2 = fold_paper(&input, true);
        assert_eq!(res2_example, 16);
        assert_eq!(res2, 104);
    }
}
