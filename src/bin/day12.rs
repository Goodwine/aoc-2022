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
  let start = find(b'S', &data);
  let end = find(b'E', &data);

  return shortest_path_len(&data, start, end);
}

fn p2(data: Vec<Vec<u8>>) -> usize {
  let end = find(b'E', &data);

  return data
    .iter()
    .enumerate()
    .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, c)| (i, j, c)))
    .filter_map(|(i, j, c)| match c {
      b'S' | b'a' => Some(Point(i, j)),
      _ => None,
    })
    .map(|start| shortest_path_len(&data, start, end))
    .min()
    .unwrap();
}

fn shortest_path_len(data: &Vec<Vec<u8>>, start: Point, end: Point) -> usize {
  let mut queue: LinkedList<(usize, Point)> = LinkedList::from([(0, start)]);
  let mut seen: HashSet<Point> = HashSet::new();
  while !queue.is_empty() {
    let (steps, current) = queue.pop_front().unwrap();
    if !seen.insert(current) {
      continue;
    }

    if current == end {
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
    let current = match data[self.0][self.1] {
      b'S' => b'a' - 1,
      v => v,
    };
    return (min_i..=1)
      .flat_map(|di: i32| {
        (min_j..=1)
          .map(move |dj: i32| ((self.0 as i32 + di) as usize, (self.1 as i32 + dj) as usize))
      })
      // XOR makes sure only one changed, not both and not none.
      .filter(|&(i, j)| (i == self.0) != (j == self.1))
      .filter(|&(i, j)| i < max_i && j < max_j)
      .filter(|&(i, j)| data[i][j] <= current + 1)
      .map(|(i, j)| Point(i, j))
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
