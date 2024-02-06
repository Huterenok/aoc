use std::collections::HashSet;

fn win_squidward(input: &str) -> usize {
    let (nums, boards) = parse_bingo(input);

    for i in 5..nums.len() {
        for board in &boards {
            if let Some(winner) = find_winner(&nums[..i], board) {
                return winner;
            }
        }
    }

    unreachable!()
}

fn lets_lose(input: &str) -> usize {
    let (nums, boards) = parse_bingo(input);
    let mut boards = boards.iter().collect::<HashSet<_>>();
    for i in 5..nums.len() {
        let winners = boards
            .iter()
            .filter_map(|&board| find_winner(&nums[0..i], board).map(|points| (board, points)))
            .collect::<Vec<_>>();
        for (b, _) in &winners {
            boards.remove(b);
        }
        if boards.is_empty() {
            return winners[0].1;
        }
    }
    unreachable!()
}

fn count_points(nums: &[usize], board: &[Vec<usize>]) -> usize {
    board
        .iter()
        .flatten()
        .filter(|x| !nums.contains(x))
        .sum::<usize>()
        * nums.last().unwrap()
}

fn find_winner(nums: &[usize], board: &[Vec<usize>]) -> Option<usize> {
    for x in 0..5 {
        if (0..5).all(|y| nums.contains(&board[x][y])) {
            return Some(count_points(nums, board));
        }
        if (0..5).all(|y| nums.contains(&board[y][x])) {
            return Some(count_points(nums, board));
        }
    }
    None
}

fn parse_bingo(input: &str) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let mut bingo_parts = input.split("\n\n");
    let nums = bingo_parts
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let boards = bingo_parts
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    (nums, boards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = win_squidward(&example_input);
        let res1 = win_squidward(&input);
        assert_eq!(res1_example, 4512);
        assert_eq!(res1, 71708);

        let res2_example = lets_lose(&example_input);
        let res2 = lets_lose(&input);
        assert_eq!(res2_example, 1924);
        assert_eq!(res2, 34726)
    }
}
