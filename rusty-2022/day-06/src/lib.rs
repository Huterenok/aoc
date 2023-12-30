use itertools::Itertools;

pub fn detect_marker(input: &str, unique_seq_count: usize) -> usize {
    input
        .as_bytes()
        .windows(unique_seq_count)
        .enumerate()
        .find(|(_, seq)| seq.iter().unique().count() == seq.len())
        .unwrap()
        .0
        + unique_seq_count
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = detect_marker(&example_input, 4);
        let res1 = detect_marker(&input, 4);
        assert_eq!(res1_example, 7);
        assert_eq!(res1, 1238);

        let res2_example = detect_marker(&example_input, 14);
        let res2 = detect_marker(&input, 14);
        assert_eq!(res2_example, 19);
        assert_eq!(res2, 3037);
    }
}
