use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Graph<'a> {
    pub nodes: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut nodes = HashMap::new();
        input.lines().into_iter().for_each(|line| {
            let (key, vals) = line.split_once(": ").unwrap();
            vals.split(" ").for_each(|val| {
                nodes.entry(key).or_insert(HashSet::new()).insert(val);
                nodes.entry(val).or_insert(HashSet::new()).insert(key);
            })
        });

        Self { nodes }
    }
}

pub fn cut_da_three_wirez(input: String) -> usize {
    let graph = Graph::new(&input);
    let mut freq = HashMap::new();

    for key in graph.nodes.keys() {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(*key);
        visited.insert(*key);

        while let Some(pos) = queue.pop_front() {
            for child in graph.nodes.get(pos).unwrap() {
                if visited.insert(child) {
                    let key = if pos < *child {
                        [pos, *child]
                    } else {
                        [*child, pos]
                    };

                    let entry = freq.entry(key).or_insert(0);
                    *entry += 1;

                    queue.push_back(*child);
                }
            }
        }
    }

    let mut order = freq.iter().collect::<Vec<_>>();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();

    let cut: Vec<_> = order.iter().take(3).map(|p| *p.0).collect();
    let start = *graph.nodes.keys().next().unwrap();
    let mut size = 1;

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(pos) = queue.pop_front() {
        for child in graph.nodes.get(pos).unwrap() {
            let key = if pos < *child {
                [pos, *child]
            } else {
                [*child, pos]
            };

            if cut.contains(&key) {
                continue;
            }

            if visited.insert(*child) {
                size += 1;
                queue.push_back(child);
            }
        }
    }

    size * (graph.nodes.len() - size)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cut_da_three_wirez;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        assert_eq!(54, cut_da_three_wirez(example_input));
        assert_eq!(582692, cut_da_three_wirez(input));
    }
}
