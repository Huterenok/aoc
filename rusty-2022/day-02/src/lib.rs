pub fn win_almost_every_game(input: &str) -> usize {
    let data = parse_data(input);
    data.into_iter().fold(0, |acc, (left, right)| {
        (match left.cmp(&right) {
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 0,
            std::cmp::Ordering::Less => 6,
        }) + acc
            + right as usize
    })
}

pub fn stupid_rules_to_win(input: &str) -> usize {
    let data = parse_data(input);
    data.into_iter().fold(0, |acc, (left, right)| {
        (match right {
            Sign::Rock => left.next_loser() as usize,
            Sign::Paper => 3 + left as usize,
            Sign::Scissors => 6 + left.next_winner() as usize,
        }) + acc
    })
}

pub fn parse_data(input: &str) -> Vec<(Sign, Sign)> {
    input
        .lines()
        .map(|s| {
            let (left, right) = s.split_once(" ").unwrap();
            (left.into(), right.into())
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Sign {
    pub fn next_winner(&self) -> Self {
        match self {
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn next_loser(&self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)
            | (Self::Rock, Self::Scissors) => std::cmp::Ordering::Greater,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl From<&str> for Sign {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Sign::Rock,
            "B" | "Y" => Sign::Paper,
            _ => Sign::Scissors,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = win_almost_every_game(&example_input);
        let res1 = win_almost_every_game(&input);
        assert_eq!(res1_example, 15);
        assert_eq!(res1, 12679);

        let res2_example = stupid_rules_to_win(&example_input);
        let res2 = stupid_rules_to_win(&input);
        assert_eq!(res2_example, 12);
        assert_eq!(res2, 14470);
    }
}
