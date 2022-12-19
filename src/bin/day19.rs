use std::collections::{HashMap, HashSet};

use aoc;
use rayon::prelude::*;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| Blueprint::from(line))
        .collect::<Vec<_>>()
    },
    &p1,
    &p2,
  );
}

#[derive(Clone)]
struct Blueprint {
  ore: Bag,
  clay: Bag,
  obsidian: Bag,
  geode: Bag,
}

impl From<&String> for Blueprint {
  fn from(line: &String) -> Self {
    let mut iter = line
      .split_terminator([' ', ':', '.'])
      .filter(|s| !s.is_empty())
      .filter_map(|s| s.parse::<u128>().ok())
      .skip(1);

    return Self {
      ore: Bag(iter.next().unwrap()),
      clay: Bag(iter.next().unwrap()),
      obsidian: Bag(iter.next().unwrap() + (iter.next().unwrap() << 32)),
      geode: Bag(iter.next().unwrap() + (iter.next().unwrap() << 64)),
    };
  }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Bag(u128);

fn p1(data: Vec<Blueprint>) -> usize {
  return solve(data, 24)
    .iter()
    .enumerate()
    .map(|(i, v)| (i + 1) * v)
    .sum();
}

fn p2(mut data: Vec<Blueprint>) -> usize {
  data.truncate(3);
  return solve(data, 32).iter().fold(1, |acc, v| acc * v);
}

fn solve(data: Vec<Blueprint>, time: usize) -> Vec<usize> {
  return data
    .par_iter()
    .map(|b| b.eval(time, &Bag(0), &Bag(1), &mut HashMap::new(), 0))
    .enumerate()
    .map(|(i, v)| {
      println!("done {}, {v}", i + 1);
      return v;
    })
    .collect();
}

impl Blueprint {
  fn options(&self, backpack: &Bag, robots: &Bag, time: usize) -> HashSet<(Bag, Bag, usize)> {
    let mut options: HashSet<(Bag, Bag, usize)> =
      HashSet::from([(backpack.clone(), robots.clone(), 0)]);

    if backpack.contains(&self.geode) {
      let bp = Bag(backpack.0 - &self.geode.0);
      // Add all geodes that would be created by a geode robot during the time left.
      options.insert((bp, robots.clone(), time - 1));
    }
    if backpack.contains(&self.obsidian) {
      let bp = Bag(backpack.0 - &self.obsidian.0);
      let r = Bag(robots.0 + (1 << 64));
      options.insert((bp, r, 0));
    }
    if backpack.contains(&self.clay) {
      let bp = Bag(backpack.0 - &self.clay.0);
      let r = Bag(robots.0 + (1 << 32));
      options.insert((bp, r, 0));
    }
    if backpack.contains(&self.ore) {
      let bp = Bag(backpack.0 - &self.ore.0);
      let r = Bag(robots.0 + 1);
      options.insert((bp, r, 0));
    }

    return options;
  }

  fn eval(
    &self,
    time: usize,
    backpack: &Bag,
    robots: &Bag,
    dp: &mut HashMap<(usize, u128, u128), usize>,
    geodes: usize,
  ) -> usize {
    if time == 0 {
      return geodes;
    }

    let k = (time, backpack.0, robots.0);
    match dp.get(&k) {
      // Don't explore a branch that would result in fewer geodes.
      Some(v) if v >= &geodes => return 0,
      // Record how many geodes are to be generated so far.
      _ => dp.insert(k, geodes),
    };

    let result = self
      .options(&backpack, &robots, time)
      .iter()
      .map(|(bp, r, dg)| self.eval(time - 1, &Bag(bp.0 + robots.0), r, dp, geodes + dg))
      .max()
      .unwrap();

    return result;
  }
}

const ORE_MASK: u128 = std::u32::MAX as u128;
const CLAY_MASK: u128 = ORE_MASK << 32;
const OBSIDIAN_MASK: u128 = ORE_MASK << 64;

impl Bag {
  fn contains(&self, Self(other): &Self) -> bool {
    return other & ORE_MASK <= self.0 & ORE_MASK
      && other & CLAY_MASK <= self.0 & CLAY_MASK
      && other & OBSIDIAN_MASK <= self.0 & OBSIDIAN_MASK;
  }
}
