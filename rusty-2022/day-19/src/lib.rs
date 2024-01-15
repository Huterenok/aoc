use derive_more::Constructor;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::Regex;

fn sum_geodes(input: &str) -> usize {
    let blueprints = parse_blueprints(input);
    blueprints
        .into_par_iter()
        .enumerate()
        .map(|(i, blueprint)| find_blueprint_score(&blueprint, 24) * (i + 1))
        .sum()
}

fn sum_more_geodes(input: &str) -> usize {
    let blueprints = parse_blueprints(input);
    blueprints
        .into_par_iter()
        .take(3)
        .map(|blueprint| find_blueprint_score(&blueprint, 32))
        .product()
}

fn find_blueprint_score(blueprint: &Blueprint, time: usize) -> usize {
    let state = State::new([0, 0, 0, 0], time, [1, 0, 0, 0]);
    let max_materials = blueprint
        .recipes
        .iter()
        .fold([0, 0, 0, u64::MAX as usize], |acc, r| {
            [acc[0].max(r[0]), acc[1].max(r[1]), acc[2].max(r[2]), acc[3]]
        });

    process_blueprint(blueprint, &state, max_materials, None, 0)
}

fn process_blueprint(
    blueprint: &Blueprint,
    state: &State,
    max_materials: [usize; 4],
    prev_skipped: Option<&[usize]>,
    best: usize,
) -> usize {
    if state.time == 1 {
        return state.materials[3] + state.robots[3];
    }

    if optimistic_best(state, 3) < best {
        return 0;
    }

    if optimistic_best(state, 2) < max_materials[2] {
        return state.materials[3] + state.robots[3] * state.time;
    }

    let mut new_state = state.clone();
    new_state.time -= 1;
    (0..4).for_each(|i| new_state.materials[i] += new_state.robots[i]);

    if state.can_build_robot(&blueprint, 3, max_materials) {
        new_state.build_robot(3, blueprint);
        return process_blueprint(&blueprint, &new_state, max_materials, None, best);
    }

    let robots_available = (0..3)
        .filter(|i| state.can_build_robot(blueprint, *i, max_materials))
        .collect::<Vec<usize>>();
    let mut best = best;

    for &robot_id in &robots_available {
        if prev_skipped
            .map(|ls| ls.contains(&robot_id))
            .unwrap_or(false)
        {
            continue;
        }

        new_state.build_robot(robot_id, blueprint);
        let score = process_blueprint(blueprint, &new_state, max_materials, None, best);
        best = score.max(best);
        new_state.unbuild_robot(robot_id, blueprint);
    }

    let score = process_blueprint(
        blueprint,
        &new_state,
        max_materials,
        Some(&robots_available),
        best,
    );
    best = score.max(best);

    best
}

fn optimistic_best(state: &State, material_id: usize) -> usize {
    state.materials[material_id]
        + state.robots[material_id] * state.time
        + state.time * (state.time - 1) / 2
}

#[derive(Debug, Constructor, Clone)]

pub struct State {
    pub materials: [usize; 4],
    pub time: usize,
    pub robots: [usize; 4],
}

impl State {
    pub fn collect_materials(&mut self) {
        self.robots.iter().enumerate().for_each(|(i, robot_count)| {
            self.materials[i] += robot_count;
        });
    }

    pub fn can_build_robot(
        &self,
        blueprint: &Blueprint,
        robot_id: usize,
        max_materials: [usize; 4],
    ) -> bool {
        self.materials
            .iter()
            .zip(blueprint.recipes[robot_id])
            .all(|(sm, bm)| *sm >= bm)
            && self.robots[robot_id] <= max_materials[robot_id]
    }

    fn build_robot(&mut self, robot_id: usize, blueprint: &Blueprint) {
        self.robots[robot_id] += 1;
        blueprint.recipes[robot_id]
            .iter()
            .enumerate()
            .for_each(|(i, m)| {
                self.materials[i] -= m;
            })
    }

    pub fn build_state(&self, blueprint: &Blueprint, robot_id: usize) -> Self {
        let (mut materials, mut robots) = (self.materials, self.robots);
        blueprint.recipes[robot_id]
            .iter()
            .enumerate()
            .for_each(|(i, rm)| {
                materials[i] -= rm;
            });
        robots[robot_id] += 1;

        Self {
            materials,
            robots,
            time: self.time,
        }
    }

    fn unbuild_robot(&mut self, robot_id: usize, blueprint: &Blueprint) {
        self.robots[robot_id] -= 1;
        blueprint.recipes[robot_id]
            .iter()
            .enumerate()
            .for_each(|(i, m)| {
                self.materials[i] += m;
            })
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let matcher = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|line| {
            let matched: Vec<usize> = matcher
                .find_iter(line)
                .skip(1)
                .map(|capture| capture.as_str().parse().unwrap())
                .collect();
            Blueprint::new([
                [matched[0], 0, 0],
                [matched[1], 0, 0],
                [matched[2], matched[3], 0],
                [matched[4], 0, matched[5]],
            ])
        })
        .collect()
}

#[derive(Debug, Constructor)]
pub struct Blueprint {
    pub recipes: [[usize; 3]; 4],
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let res1_example = sum_geodes(&example_input);
        let res1 = sum_geodes(&input);
        assert_eq!(res1_example, 33);
        assert_eq!(res1, 1981);

        let res2_example = sum_more_geodes(&example_input);
        let res2 = sum_more_geodes(&input);
        // assert_eq!(res2_example, 62);
        assert_eq!(res2, 10962);
    }
}
