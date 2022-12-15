use std::collections::HashSet;

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      return aoc::lines(path)
        .iter()
        .map(|line| {
          // Sensor at x=9, y=16: closest beacon is at x=10, y=16
          let mut parts = line
            .split_terminator([',', '=', ' ', ':'])
            .filter_map(|part| part.parse::<i64>().ok());

          let sensor = Point {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
          };
          let beacon = Point {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
          };
          let distance = sensor.manhattan(&beacon);
          return (sensor, beacon, distance);
        })
        .collect::<Vec<_>>();
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn manhattan(&self, other: &Self) -> u64 {
    return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
  }
}

fn p1(data: Vec<(Point, Point, u64)>) -> usize {
  let target = if data.len() < 20 { 10 } else { 2_000_000 };

  let taken = data
    .iter()
    .map(|(_, b, _)| b.clone())
    .filter(|p| p.y == target)
    .collect::<HashSet<_>>();

  let scanned = data
    .iter()
    .filter_map(|(s, _, beacon_dist)| match s.y.abs_diff(target) {
      y_dist if y_dist > *beacon_dist => None,
      // This means the sensor overlaps the target line. And from position `s.x`.
      // There are `y-m` blocks to each side of `s.x` in that line that were
      // scanned, so it's impossible for there to be an unknown beacon there.
      y_dist => Some((s.x, beacon_dist - y_dist)),
    })
    .flat_map(|(x, n)| ((x - n as i64)..=(x + n as i64)).map(move |x| Point { x, y: target }))
    .collect::<HashSet<_>>();

  return scanned.difference(&taken).count();
}

fn p2(data: Vec<(Point, Point, u64)>) -> i64 {
  let seach_space = if data.len() < 20 { 20 } else { 4_000_000 };
  let range = 0..=seach_space;

  let perimeter = data
    .iter()
    .flat_map(|(s, _, d)| {
      // There is only one posibility, so we only need to scan one more onion peel.
      let d = *d as i64 + 1;
      let x_left = s.x - d..s.x;
      let x_right = s.x..s.x + d;
      let y_up = s.y - d..s.y;
      let y_down = s.y..s.y + d;
      let diag_left_up = x_left.clone().zip(y_up.clone());
      let diag_left_down = x_left.zip(y_down.clone());
      let diag_right_up = x_right.clone().zip(y_up);
      let diag_right_down = x_right.zip(y_down);
      diag_left_up
        .chain(diag_left_down)
        .chain(diag_right_up)
        .chain(diag_right_down)
        .filter(|(x, y)| range.contains(x) && range.contains(y))
        .map(|(x, y)| Point { x, y })
    })
    .collect::<HashSet<_>>();

  // It is possible that beacons create a "cavity" such that the missing beacon
  // actually exists inside the scanner perimeters but it is shadowed by the others.
  // This can be solved by scanning the squares adjacent to all beacons. However,
  // the input data didn't have such case.
  let unscanned = perimeter
    .iter()
    .filter(|p| data.iter().all(|(s, _, d)| p.manhattan(s) > *d))
    .next()
    .unwrap();

  return tuning_frequency(unscanned);
}

fn tuning_frequency(p: &Point) -> i64 {
  const FREQUENCY: i64 = 4_000_000;
  return p.x * FREQUENCY + p.y;
}
