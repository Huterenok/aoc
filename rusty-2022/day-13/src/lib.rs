use serde_json::Value;
use std::cmp::Ordering;

pub fn count_right_pockets(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|part| {
            part.split("\n")
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .collect::<Vec<Value>>()
        })
        .enumerate()
        .filter(|(_, parts)| check_pockets(&parts[0], &parts[1]) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn reorder_pockets(input: &str) -> usize {
    let mut pockets: Vec<Value> = input
        .split("\n\n")
        .flat_map(|part| {
            part.split("\n")
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .collect::<Vec<Value>>()
        })
        .collect();

    let dividers = vec![
        serde_json::from_str("[[2]]").unwrap(),
        serde_json::from_str("[[6]]").unwrap(),
    ];

    pockets.extend(dividers.clone());
    pockets.sort_by(check_pockets);

    pockets
        .into_iter()
        .enumerate()
        .filter(|(_, v)| dividers.contains(v))
        .map(|(i, _)| i + 1)
        .product::<usize>()
}

fn check_pockets(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(num1), Value::Number(num2)) => num1.as_u64().cmp(&num2.as_u64()),
        (Value::Array(x), Value::Array(y)) => {
            for (num1, num2) in x.iter().zip(y.iter()) {
                match check_pockets(num1, num2) {
                    Ordering::Equal => continue,
                    non_eq => return non_eq,
                }
            }
            x.len().cmp(&y.len())
        }
        (Value::Array(_), Value::Number(_)) => {
            check_pockets(left, &Value::Array(vec![right.clone()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            check_pockets(&Value::Array(vec![left.clone()]), right)
        }
        _ => panic!("RETARDED"),
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

        let res1_example = count_right_pockets(&example_input);
        let res1 = count_right_pockets(&input);
        assert_eq!(res1_example, 13);
        assert_eq!(res1, 5682);

        let res2_example = reorder_pockets(&example_input);
        let res2 = reorder_pockets(&input);
        assert_eq!(res2_example, 140);
        assert_eq!(res2, 20304);
    }
}
