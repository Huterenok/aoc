use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

/// BOOM

pub fn open_valves(input: &str) -> u32 {
    let valves = parse_valves(input);

    let graph = init_graph(&valves);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let len = dist.len();

    let init_mask: u64 = (1 << len) - 1;

    let (flow, _) = simulate(&valves, &dist, init_mask, start_idx, 30);

    flow
}

fn teach_elephants(input: &str) -> u32 {
    let valves = parse_valves(input);

    let graph = init_graph(&valves);
    let dist = floyd_warshall(graph);

    let start_idx = valves.iter().position(|x| x.name == "AA").unwrap();
    let init_mask: u64 = (1 << dist.len()) - 1;

    let (_, elf_memo) = simulate(&valves, &dist, init_mask, start_idx, 26);
    let (_, elephant_memo) = simulate(&valves, &dist, init_mask, start_idx, 26);

    let max_flow = elf_memo.iter().fold(0, |max, (&elf_mask, &elf_flow)| {
        elephant_memo
            .iter()
            .fold(max, |max, (&mask, &elephant_flow)| {
                if (!mask) & (!elf_mask) & init_mask == 0 {
                    return max.max(elephant_flow + elf_flow);
                }

                max
            })
    });

    max_flow
}

fn simulate(
    valves: &Vec<SimplifiedValve>,
    dist: &Vec<Vec<u32>>,
    init_mask: u64,
    start_idx: usize,
    minutes: u32,
) -> (u32, HashMap<u64, u32>) {
    let non_zero_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, x)| x.flow > 0)
        .map(|(i, _)| i)
        .collect();

    let flow = 0;
    let mut mask_flow: HashMap<u64, u32> = HashMap::new();

    let flow = traveling_salesman(
        valves,
        &mut mask_flow,
        &non_zero_valves,
        &dist,
        init_mask,
        minutes,
        flow,
        start_idx,
        0,
    );

    (flow, mask_flow)
}

fn traveling_salesman(
    valves: &Vec<SimplifiedValve>,
    memo: &mut HashMap<u64, u32>,
    non_zero_valves: &Vec<usize>,
    dist: &Vec<Vec<u32>>,
    mask: u64,
    minutes: u32,
    flow: u32,
    i: usize,
    depth: u32,
) -> u32 {
    let mut max_flow = flow;

    memo.insert(mask, *memo.get(&mask).unwrap_or(&0).max(&flow));

    for &j in non_zero_valves.iter() {
        let cur_minutes = minutes
            .checked_sub(dist[i][j])
            .and_then(|x| x.checked_sub(1))
            .unwrap_or(0);

        if (mask & (1 << j)) == 0 || cur_minutes <= 0 {
            continue;
        }

        let cur_mask = mask & !(1 << j);

        let cur_flow = flow + (cur_minutes * valves[j].flow);

        max_flow = max_flow.max(traveling_salesman(
            valves,
            memo,
            non_zero_valves,
            dist,
            cur_mask,
            cur_minutes,
            cur_flow,
            j,
            depth + 1,
        ));
    }

    return max_flow;
}

fn floyd_warshall(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let l = graph.len();
    let mut dist = graph.clone();

    for k in 0..l {
        for i in 0..l {
            for j in 0..l {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}

fn init_graph(list: &Vec<SimplifiedValve>) -> Vec<Vec<u32>> {
    let l = list.len();
    let mut graph = vec![vec![u32::MAX / 4; l]; l];

    list.iter().enumerate().for_each(|(i, x)| {
        x.to.iter().for_each(|&j| graph[i][j as usize] = 1);
    });

    graph
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: u32,
    to: Vec<String>,
}

struct SimplifiedValve {
    name: String,
    flow: u32,
    to: Vec<usize>,
}

pub fn parse_valves(input: &str) -> Vec<SimplifiedValve> {
    let valves: Vec<Valve> = input.lines().map(parse_line).collect();
    let index_map: HashMap<String, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();
    valves
        .into_iter()
        .map(|v| SimplifiedValve {
            flow: v.flow,
            name: v.name,
            to: v
                .to
                .into_iter()
                .map(|x| *index_map.get(x.as_str()).unwrap())
                .collect(),
        })
        .collect()
}

fn valve_name(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_uppercase() && c.is_alphabetic())(input)
}

fn flow_rate(input: &str) -> IResult<&str, u32> {
    map_res(preceded(tag("flow rate="), digit1), str::parse)(input)
}

fn single_valve_connection(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, valve) = preceded(tag(" leads to valve "), valve_name)(input)?;
    Ok((input, vec![valve]))
}

fn multiple_valve_connections(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        tag("s lead to valves "),
        separated_list0(tag(", "), valve_name),
    )(input)
}

fn valve_connections(input: &str) -> IResult<&str, Vec<&str>> {
    alt((multiple_valve_connections, single_valve_connection))(input)
}

fn parse_line(input: &str) -> Valve {
    let (_, valve) = tuple((
        preceded(tag("Valve "), valve_name),
        preceded(take_until("flow rate="), flow_rate),
        preceded(tag("; tunnel"), valve_connections),
    ))(input)
    .unwrap();
    Valve {
        name: valve.0.to_string(),
        flow: valve.1,
        to: valve.2.into_iter().map(str::to_string).collect(),
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

        let res1_example = open_valves(&example_input);
        let res1 = open_valves(&input);
        assert_eq!(res1_example, 1651);
        assert_eq!(res1, 1986);

        let res2_example = teach_elephants(&example_input);
        let res2 = teach_elephants(&input);
        assert_eq!(res2_example, 1707);
        assert_eq!(res2, 2464);
    }
}
