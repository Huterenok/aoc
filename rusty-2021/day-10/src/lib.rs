fn score_corrupted_chunks(input: &str) -> usize {
    input
        .lines()
        .map(|line| score_chunk(line).1.unwrap_or(0))
        .sum()
}

fn build_incomplete_chunks(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| match score_chunk(line) {
            (stack, None) => Some(stack),
            (_, Some(_)) => None,
        })
        .map(|stack| {
            stack.into_iter().rev().fold(0, |acc, c| {
                5 * acc
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn score_chunk(input: &str) -> (Vec<char>, Option<usize>) {
    let mut stack = Vec::new();
    let score = input.chars().find_map(|c| match c {
        '[' | '{' | '(' | '<' => {
            stack.push(c);
            None
        }
        ']' if stack.pop() != Some('[') => Some(57),
        '}' if stack.pop() != Some('{') => Some(1197),
        ')' if stack.pop() != Some('(') => Some(3),
        '>' if stack.pop() != Some('<') => Some(25137),
        _ => None,
    });

    (stack, score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("./example_input.txt").unwrap();
        let input = fs::read_to_string("./input.txt").unwrap();

        let res1_example = score_corrupted_chunks(&example_input);
        let res1 = score_corrupted_chunks(&input);
        assert_eq!(res1_example, 26397);
        assert_eq!(res1, 311949);

        let res2_example = build_incomplete_chunks(&example_input);
        let res2 = build_incomplete_chunks(&input);
        assert_eq!(res2_example, 288957);
        assert_eq!(res2, 3042730309)
    }
}
