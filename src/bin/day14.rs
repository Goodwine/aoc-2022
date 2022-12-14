use std::{collections::HashMap, fmt::Display};

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let segments = aoc::lines(path)
        .iter()
        .flat_map(|line| {
          line
            .split(" -> ")
            .map(|p| {
              let mut iter = p.splitn(2, ",").map(|n| n.parse::<u64>().unwrap());
              return Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
              };
            })
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
            .map(|segment| segment.to_vec())
            .collect::<Vec<Vec<Point>>>()
        })
        .collect::<Vec<_>>();
      let mut grid: HashMap<Point, Tile> = HashMap::new();
      for segment in segments {
        match segment.as_slice() {
          [Point { x: a, y: b }, Point { x: c, y: d }] => {
            let (&a, &c) = if a > c { (c, a) } else { (a, c) };
            let (&b, &d) = if b > d { (d, b) } else { (b, d) };
            for x in a..=c {
              for y in b..=d {
                grid.insert(Point { x, y }, Tile::Rock);
              }
            }
          }
          _ => panic!("impossible"),
        }
      }
      grid.insert(Point { x: 500, y: 0 }, Tile::Start);
      return Grid::new(grid);
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Debug)]
enum Tile {
  Start,
  Rock,
  Sand,
}

/// (X,Y) coordinates. Higher Y means lower vertically.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
  x: u64,
  y: u64,
}

impl Point {
  fn down(&self) -> Point {
    return Self {
      x: self.x,
      y: self.y + 1,
    };
  }
  fn diag_left(&self) -> Point {
    return Self {
      x: self.x - 1,
      y: self.y + 1,
    };
  }
  fn diag_right(&self) -> Point {
    return Self {
      x: self.x + 1,
      y: self.y + 1,
    };
  }
}

#[derive(Clone, Debug)]
struct Grid {
  grid: HashMap<Point, Tile>,
  abyss: u64,
  min_x: u64,
  max_x: u64,
}

impl Grid {
  fn new(grid: HashMap<Point, Tile>) -> Self {
    return Self {
      abyss: *grid.keys().map(|Point { y, .. }| y).max().unwrap() + 3,
      min_x: *grid.keys().map(|Point { x, .. }| x).min().unwrap() - 3,
      max_x: *grid.keys().map(|Point { x, .. }| x).max().unwrap() + 3,
      grid,
    };
  }

  fn add_sand(&mut self, p: &Point) -> Option<bool> {
    let can_move = self.try_spot(&p.down())?
      || self.try_spot(&p.diag_left())?
      || self.try_spot(&p.diag_right())?;

    if !can_move {
      self.grid.insert(p.clone(), Tile::Sand);
    }
    return Some(true);
  }

  fn try_spot(&mut self, p: &Point) -> Option<bool> {
    if p.y >= self.abyss {
      return None;
    }
    if p.x <= self.min_x || p.x >= self.max_x {
      return Some(false);
    }
    if !self.grid.contains_key(p) {
      return self.add_sand(p);
    }
    return Some(false);
  }
}

fn p1(mut data: Grid) -> usize {
  let start = Point { x: 500, y: 0 };

  while data.add_sand(&start).is_some() {}

  return data
    .grid
    .values()
    .filter(|tile| match tile {
      Tile::Sand => true,
      _ => false,
    })
    .count();
}

/// Originally this ran a full simulation taking about half a second. Not bad.
/// But we don't have to actually simulate the entire horizontal space, we can
/// instead limit our simulation to a few steps left-and-right from the farthest
/// rock. Then we can assume the rest will always be full.
///
/// This optimization cuts down the problem from O(H²) to O(H*W).
/// on my PC this goes from 500ms to 40ms
fn p2(mut data: Grid) -> usize {
  let min_x = data.min_x;
  let max_x = data.max_x;
  let floor = data.abyss - 1;

  for x in min_x..=max_x {
    data.grid.insert(Point { x, y: floor }, Tile::Rock);
  }

  let start = Point { x: 500, y: 0 };
  loop {
    match data.grid.get(&start) {
      Some(Tile::Start) => data.add_sand(&start),
      _ => break,
    };
  }

  let sand_tiles = data
    .grid
    .iter()
    .filter_map(|(p, v)| match v {
      Tile::Sand => Some(p),
      _ => None,
    })
    .collect::<Vec<_>>();

  let min_x = sand_tiles.iter().map(|Point { x, .. }| x).min().unwrap();
  let max_x = sand_tiles.iter().map(|Point { x, .. }| x).max().unwrap();

  let air_height_left = sand_tiles
    .iter()
    .filter_map(|Point { x, y }| if x == min_x { Some(y) } else { None })
    .min()
    .unwrap();
  let air_height_right = sand_tiles
    .iter()
    .filter_map(|Point { x, y }| if x == max_x { Some(y) } else { None })
    .min()
    .unwrap();

  let adjust = data.abyss - 2;
  let min_left = adjust - air_height_left;
  let min_right = adjust - air_height_right;

  let left_imaginary_sand = sum_zero_to_n(min_left) as usize;
  let right_imaginary_sand = sum_zero_to_n(min_right) as usize;
  return sand_tiles.len() + left_imaginary_sand + right_imaginary_sand;
}

fn sum_zero_to_n(n: u64) -> u64 {
  return n * (n + 1) / 2;
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let grid: String = (0..self.abyss)
      .flat_map(|y| {
        (self.min_x..=self.max_x)
          .map(move |x| match self.grid.get(&Point { x, y }) {
            None => ' ',
            Some(Tile::Start) => '•',
            Some(Tile::Rock) => '█',
            Some(Tile::Sand) => '▒',
          })
          .chain(['\n'])
      })
      .collect();
    return write!(f, "{grid}");
  }
}
