use std::collections::HashSet;

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

#[derive(Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
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
    .map(|b| b.eval(time, Bag(0), Bag(1)))
    .collect();
}

impl Blueprint {
  fn options(
    &self,
    backpack: &Bag,
    robots: &Bag,
    geodes: usize,
    time: usize,
  ) -> Vec<(Bag, Bag, usize)> {
    let mut options = vec![(Bag(backpack.0 + robots.0), robots.clone(), geodes)];

    if backpack.contains(&self.geode) {
      let bp = Bag(backpack.0 - &self.geode.0 + robots.0);
      // Add all geodes that would be created by a geode robot during the time left.
      options.push((bp, robots.clone(), geodes + time - 1));
    }
    if backpack.contains(&self.obsidian) {
      let bp = Bag(backpack.0 - &self.obsidian.0 + robots.0);
      let r = Bag(robots.0 + (1 << 64));
      options.push((bp, r, geodes));
    }
    if backpack.contains(&self.clay) {
      let bp = Bag(backpack.0 - &self.clay.0 + robots.0);
      let r = Bag(robots.0 + (1 << 32));
      options.push((bp, r, geodes));
    }
    if backpack.contains(&self.ore) {
      let bp = Bag(backpack.0 - &self.ore.0 + robots.0);
      let r = Bag(robots.0 + 1);
      options.push((bp, r, geodes));
    }

    return options;
  }

  /// Because the problem statement mentions how getting geode robots earlier will
  /// give better returns, instead of using DP to explode all the options one by
  /// one and using DP to avoid looking at certain branches.. we can use BFS with
  /// with aggressive prunning after each layer.
  ///
  /// Prunning happens by giving the most importance to geodes-generated followed
  /// by robots and followed by whatever is in the backpack. Instead of exploring
  /// we can discard every layer under a threshold.
  /// (e.g. discard anything beyond 1000).
  fn eval(&self, time: usize, backpack: Bag, robots: Bag) -> usize {
    // The order of [GEODES, ROBOTS, BACKPACK] in the tuple is important for prunning.
    // because we call sort().
    let mut work = vec![(0, robots.clone(), backpack)];

    for time in (1..=time).rev() {
      let next_work = work
        .iter()
        // Everything below here is messy (swapping left and right) because I had
        // to change the ordering of the tuple for sorting because I didn't want to
        // implement custom sorting.
        .flat_map(|(geodes, r, bp)| self.options(bp, r, *geodes, time))
        .collect::<HashSet<_>>();

      work = next_work
        .iter()
        .map(|(bp, r, geodes)| (*geodes, r.clone(), bp.clone()))
        .collect();

      work.sort_by(|a, b| a.cmp(b).reverse());
      work.truncate(1000);
    }

    return *work.iter().map(|(g, ..)| g).max().unwrap();
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
