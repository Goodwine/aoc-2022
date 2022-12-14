use std::collections::{HashSet, LinkedList};

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect::<Vec<Vec<u8>>>()
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<Vec<u8>>) -> usize {
  // Problem states S->E, but it's faster to go E to S.
  let start = find(b'E', &data);
  return shortest_path_len(&data, start, b'S');
}

fn p2(data: Vec<Vec<u8>>) -> usize {
  // Problem states any(a)->E, but it's faster to go E to any(a).
  let start = find(b'E', &data);
  return shortest_path_len(&data, start, b'a');
}

fn shortest_path_len(data: &Vec<Vec<u8>>, start: Point, end: u8) -> usize {
  let mut queue: LinkedList<(usize, Point)> = LinkedList::from([(0, start)]);
  let mut seen: HashSet<Point> = HashSet::new();
  while !queue.is_empty() {
    let (steps, current) = queue.pop_front().unwrap();
    if !seen.insert(current) {
      continue;
    }

    let Point(i, j) = current;
    if data[i][j] == end {
      return steps;
    }

    for p in current.candidates(&data, &seen) {
      queue.push_back((steps + 1, p));
    }
  }

  return std::usize::MAX;
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

impl Point {
  fn candidates(&self, data: &Vec<Vec<u8>>, seen: &HashSet<Point>) -> Vec<Point> {
    let min_i = if self.0 == 0 { 0 } else { -1 };
    let min_j = if self.1 == 0 { 0 } else { -1 };
    let max_i = data.len();
    let max_j = data[0].len();
    let current = fix_altitude(data[self.0][self.1]);
    return (min_i..=1)
      .flat_map(|di: i32| {
        (min_j..=1)
          .map(move |dj: i32| ((self.0 as i32 + di) as usize, (self.1 as i32 + dj) as usize))
      })
      // XOR makes sure only one changed, not both and not none.
      .filter(|&(i, j)| (i == self.0) != (j == self.1))
      .filter(|&(i, j)| i < max_i && j < max_j)
      .map(|(i, j)| (i, j, fix_altitude(data[i][j])))
      // The problem states that you can jump up +1 or down any number.
      // However, navigating the maze (most mazes?) is faster backwards.
      // So instead walking from the "end" to the "start" we check that we can
      // only drop down -1 or up any number.
      .filter(|&(_, _, c)| c >= current - 1)
      .map(|(i, j, _)| Point(i, j))
      .filter(|p| !seen.contains(p))
      .collect();
  }
}

fn find(target: u8, data: &Vec<Vec<u8>>) -> Point {
  let (i, j, _) = data
    .iter()
    .enumerate()
    .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, c)| (i, j, c)))
    .find(|(_, _, c)| **c == target)
    .unwrap();
  return Point(i, j);
}

fn fix_altitude(c: u8) -> u8 {
  match c {
    b'S' => b'a',
    b'E' => b'z',
    _ => c,
  }
}
