use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn run_da_elves(input: &str) -> (usize, usize) {
    let mut elves = parse_elves(input);
    elves.reserve(5000);
    let mut moves: HashMap<Elf, Vec<Elf>> = HashMap::with_capacity(5000);

    let (mut p1, mut p2) = (0, 0);

    for i in 0.. {
        for Elf(x, y) in &elves {
            let (x, y) = (*x, *y);
            let maybe_north = !elves.contains(&Elf(x, y - 1))
                && !elves.contains(&Elf(x + 1, y - 1))
                && !elves.contains(&Elf(x - 1, y - 1));
            let maybe_west = !elves.contains(&Elf(x - 1, y))
                && !elves.contains(&Elf(x - 1, y + 1))
                && !elves.contains(&Elf(x - 1, y - 1));
            let maybe_east = !elves.contains(&Elf(x + 1, y))
                && !elves.contains(&Elf(x + 1, y + 1))
                && !elves.contains(&Elf(x + 1, y - 1));
            let maybe_south = !elves.contains(&Elf(x, y + 1))
                && !elves.contains(&Elf(x + 1, y + 1))
                && !elves.contains(&Elf(x - 1, y + 1));

            if !maybe_east && !maybe_north && !maybe_west {
                continue;
            }

            let possible_variants = [
                (maybe_north, Elf(x, y - 1)),
                (maybe_west, Elf(x - 1, y)),
                (maybe_east, Elf(x + 1, y)),
                (maybe_south, Elf(x, y + 1)),
            ];

            for j in 0..possible_variants.len() {
                let (is_free, pos) = possible_variants[(i + j) % possible_variants.len()];
                if is_free {
                    moves.entry(pos).or_default().push(Elf(x, y));
                    break;
                }
            }
        }
        let mut moved = false;
        for (elf, to) in moves.drain() {
            if to.len() == 1 {
                moved = true;
                elves.remove(&to[0]);
                elves.insert(elf);
            }
        }
        if !moved {
            p2 = i + 1;
            break;
        }

        if i == 9 {
            let (&minx, &maxx) = elves
                .iter()
                .map(|Elf(x, _)| x)
                .minmax()
                .into_option()
                .unwrap();
            let (&miny, &maxy) = elves
                .iter()
                .map(|Elf(_, y)| y)
                .minmax()
                .into_option()
                .unwrap();
            p1 = (minx..=maxx)
                .cartesian_product(miny..=maxy)
                .filter(|(x, y)| !elves.contains(&Elf(*x, *y)))
                .count();
        }
    }

    (p1, p2)
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct Elf(i32, i32);

pub fn parse_elves(input: &str) -> HashSet<Elf> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Elf(x as i32, y as i32))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn yays() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let example_input = fs::read_to_string("./example_input.txt").unwrap();

        let (p1_example, p2_example) = run_da_elves(&example_input);
        let (p1_res, p2_res) = run_da_elves(&input);
        assert_eq!(p1_example, 110);
        assert_eq!(p1_res, 4025);
        assert_eq!(p2_example, 20);
        assert_eq!(p2_res, 935);
    }
}

fn main(input: &str) -> (usize, usize) {
  let mut state = input.lines()
    .enumerate()
    .flat_map(|(x,l)| l.bytes()
      .enumerate()
      .filter(|&(_,b)| b == b'#')
      .map(move |(y,_)| (x as i32, y as i32))
    )
    .collect::<HashSet<_>>();
  state.reserve(5000);
  let (mut p1, mut p2) = (0,0);
  let mut proposals = HashMap::<_,Vec<_>>::with_capacity(10000);
  for t in 0.. {
    for &(x,y) in &state {
      let ns = [
        state.contains(&(x-1,y-1)), state.contains(&(x-1,y)), state.contains(&(x-1,y+1)),
        state.contains(&(x,  y-1)),                           state.contains(&(x,  y+1)),
        state.contains(&(x+1,y-1)), state.contains(&(x+1,y)), state.contains(&(x+1,y+1)),
      ];
      if ns.iter().all(|b| !b) {
        continue;
      }
      let props = [
        (!ns[0] && !ns[1] && !ns[2], (x-1,y)),
        (!ns[5] && !ns[6] && !ns[7], (x+1,y)),
        (!ns[0] && !ns[3] && !ns[5], (x,y-1)),
        (!ns[2] && !ns[4] && !ns[7], (x,y+1)),
      ];
      for i in 0..4 {
        let (free, pos) = props[(t + i) % 4];
        if free {
          proposals.entry(pos).or_default().push((x,y));
          break;
        }
      }
    }
    let mut moved = false;
    for (pos, props) in proposals.drain() {
      if props.len() == 1 {
        moved = true;
        state.remove(&props[0]);
        state.insert(pos);
      }
    }
    if !moved {
      p2 = t+1;
      break;
    }
    if t == 9 {
      let (&minx, &maxx) = state.iter().map(|(x,_)| x).minmax().into_option().unwrap();
      let (&miny, &maxy) = state.iter().map(|(_,y)| y).minmax().into_option().unwrap();
      p1 = (minx..=maxx).cartesian_product(miny..=maxy).filter(|p| !state.contains(p)).count();
    }
  }
  (p1,p2)
}