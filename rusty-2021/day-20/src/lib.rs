use itertools::Itertools;

fn lets_lit(input: &str, steps: usize) -> usize {
    let (rule, image) = parse_image(input);
    (0..steps)
        .into_iter()
        .fold(image, |image, _| {
            (0..image[0].len() + 2)
                .cartesian_product(0..image.len() + 2)
                .fold(
                    vec![vec![0; image[0].len() + 2]; image.len() + 2],
                    |mut acc, (x, y)| {
                        let num = img_to_num((x as i32 - 1, y as i32 - 1), &image);

                        if rule[num] == '#' {
                            acc[y][x] = 1;
                        }
                        acc
                    },
                )
        })
        .into_iter()
        .flatten()
        .filter(|&p| p == 1)
        .count()
}

fn img_to_num((x, y): (i32, i32), image: &[Vec<u8>]) -> usize {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .enumerate()
    .fold(0, |acc, (i, (new_x, new_y))| {
        if new_x < 0 || new_y < 0 || new_x >= image[0].len() as i32 || new_y >= image.len() as i32 {
            acc
        } else {
            let add = image[new_y as usize][new_x as usize] as usize;
            acc + ((1 & add) << (8 - i))
        }
    })
}

fn parse_image(input: &str) -> (Vec<char>, Vec<Vec<u8>>) {
    let (rule, image) = input.split_once("\n\n").unwrap();
    (
        rule.chars().collect(),
        image
            .lines()
            .map(|line| line.chars().map(|c| (c == '#') as u8).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::lets_lit;

    #[test]
    fn yays() {
        let example_input = fs::read_to_string("example_input.txt").unwrap();
        let input = fs::read_to_string("input.txt").unwrap();

        let res1_example = lets_lit(&example_input, 2);
        let res1 = lets_lit(&input, 2);
        assert_eq!(res1_example, 35);
        assert_eq!(res1, 5316);

        let res2_example = lets_lit(&example_input, 50);
        let res2 = lets_lit(&input, 50);
        assert_eq!(res2_example, 3351);
        assert_eq!(res2, 16728);
    }
}
