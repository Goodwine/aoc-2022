use std::collections::{HashSet, LinkedList};

use aoc;

fn main() {
  aoc::solve(
    &|path| -> HashSet<Point> {
      aoc::lines(path)
        .iter()
        .map(|line| line.split(",").map(|part| part.parse().unwrap()))
        .map(|mut iter| Point {
          x: iter.next().unwrap(),
          y: iter.next().unwrap(),
          z: iter.next().unwrap(),
        })
        .collect()
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
  x: i64,
  y: i64,
  z: i64,
}

impl Point {
  fn adjacent(&self) -> Vec<Self> {
    return (-1..=1)
      .flat_map(|dx| (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| (dx, dy, dz))))
      // Skip self.
      .filter(|&(dx, dy, dz)| dx != 0 || dy != 0 || dz != 0)
      // remove diagonals.
      .filter(|&(dx, dy, dz)| (dx == 0 && dy == 0) || (dx == 0 && dz == 0) || (dy == 0 && dz == 0))
      .map(|(dx, dy, dz)| Self {
        x: self.x + dx,
        y: self.y + dy,
        z: self.z + dz,
      })
      .collect();
  }
}

fn p1(data: HashSet<Point>) -> usize {
  return data
    .iter()
    .map(|p| 6 - p.adjacent().iter().filter(|p| data.contains(p)).count())
    .sum();
}

const SEARCH_AREA: i64 = 30;

fn p2(data: HashSet<Point>) -> usize {
  let mut seen: HashSet<Point> = HashSet::new();

  let mut count = 0;
  let mut bfs: LinkedList<Point> = LinkedList::from([Point { x: 0, y: 0, z: 0 }]);
  while !bfs.is_empty() {
    let current = bfs.pop_front().unwrap();
    if !seen.insert(current.clone()) {
      continue;
    }
    let neighbors = current.adjacent();

    let (solid, air): (Vec<_>, Vec<_>) = neighbors
      .iter()
      .filter(|p| {
        p.x >= -1
          && p.y >= -1
          && p.z >= -1
          && p.x < SEARCH_AREA
          && p.y < SEARCH_AREA
          && p.z < SEARCH_AREA
      })
      .partition(|p| data.contains(p));

    count += solid.len();
    for air in air {
      bfs.push_back(air.clone());
    }
  }

  return count;
}
