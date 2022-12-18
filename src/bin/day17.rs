use std::{
  collections::{HashMap, HashSet},
  ops::RangeInclusive,
};

use aoc;

fn main() {
  aoc::solve(
    &|path| aoc::lines(path)[0].chars().collect::<Vec<_>>(),
    &p1,
    &p2,
  );
}

#[derive(Clone, PartialEq)]
struct Figure {
  /// The coordinates are sorted vertically from highest to lowest.
  /// Horizontal position is undefined w.r.t. sorting.
  coords: Vec<Point>,
}

impl Figure {
  fn push<F>(&self, blocks: &HashSet<Point>, transform: F) -> Figure
  where
    F: Fn(&Point) -> Point,
  {
    let coords: Vec<_> = self.coords.iter().map(transform).collect();
    if coords
      .iter()
      .any(|p @ &Point { x, y }| x <= 0 || x >= WALL || y <= 0 || blocks.contains(p))
    {
      return self.clone();
    }
    return Figure { coords };
  }

  fn move_up(&self, blocks: &HashSet<Point>, delta: u64) -> Figure {
    return self.push(blocks, |&Point { x, y }| Point { x, y: y + delta });
  }
  fn push_left(&self, blocks: &HashSet<Point>) -> Figure {
    return self.push(blocks, |&Point { x, y }| Point { x: x - 1, y });
  }
  fn push_right(&self, blocks: &HashSet<Point>) -> Figure {
    return self.push(blocks, |&Point { x, y }| Point { x: x + 1, y });
  }
  fn fall(&self, blocks: &HashSet<Point>) -> Figure {
    return self.push(blocks, |&Point { x, y }| Point { x, y: y - 1 });
  }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Point {
  x: u64,
  y: u64,
}

/// The problem says the gap is 3, so the delta is N+1.
const DELTA_FROM_TALLEST: u64 = 4;

/// The problem says the gap is 2, so the delta is N+1.
const DELTA_FROM_LEFT: u64 = 3;

fn shapes() -> Vec<Figure> {
  return vec![
    Figure {
      coords: vec![
        Point {
          x: DELTA_FROM_LEFT,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT + 2,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT + 3,
          y: 0,
        },
      ],
    },
    Figure {
      coords: vec![
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 2,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT + 2,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 0,
        },
      ],
    },
    Figure {
      coords: vec![
        Point {
          x: DELTA_FROM_LEFT + 2,
          y: 2,
        },
        Point {
          x: DELTA_FROM_LEFT + 2,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT + 2,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 0,
        },
      ],
    },
    Figure {
      coords: vec![
        Point {
          x: DELTA_FROM_LEFT,
          y: 3,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 2,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 0,
        },
      ],
    },
    Figure {
      coords: vec![
        Point {
          x: DELTA_FROM_LEFT,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 1,
        },
        Point {
          x: DELTA_FROM_LEFT,
          y: 0,
        },
        Point {
          x: DELTA_FROM_LEFT + 1,
          y: 0,
        },
      ],
    },
  ];
}

/// There are walls 0 and 8. That leaves 7 spaces in between.
const WALL: u64 = 8;

fn p1(data: Vec<char>) -> u64 {
  return simulate(data, 2022);
}

fn p2(data: Vec<char>) -> u64 {
  return simulate(data, 1000000000000);
}

// I have no idea why I only have to look at two lines.
const SEARCH_SPACE: u64 = 2;

fn simulate(data: Vec<char>, count: usize) -> u64 {
  let before_dp_starts = data.len(); // just so that things are settled.
  let mut dp: HashMap<(usize, usize, String), (u64, usize)> = HashMap::new();

  let mut blocks: HashSet<Point> = HashSet::new();
  let mut highest = 0;
  let mut extra_height = 0; // Needed so we don't mess up with the algorithm.

  let shapes = shapes();
  let mut j = 0;
  let mut i = 0;

  while i < count {
    let mut current = shapes[i % shapes.len()].move_up(&blocks, highest + DELTA_FROM_TALLEST);
    loop {
      // If the end of the list is reached, it repeats!
      current = match data[j % data.len()] {
        '<' => current.push_left(&blocks),
        '>' => current.push_right(&blocks),
        _ => panic!("impossible"),
      };
      j += 1;
      let fall = current.fall(&blocks);
      if fall == current {
        break;
      }
      current = fall;
    }

    highest = highest.max(current.coords[0].y);
    for p in current.coords {
      blocks.insert(p);
    }

    // Simulate the pentrix but use a window of SEARCH_SPACE lines to figure
    // out if we have seen this pattern before. Note that we don't scan downwards
    // for the pattern but rather we keep a DP map of patterns, next_piece, next_pos.
    if j > before_dp_starts {
      let dp_key = (
        i % shapes.len(),
        j % data.len(),
        stringify_grid(&blocks, None, (highest - SEARCH_SPACE + 1)..=highest),
      );
      if let Some((previous_height, previous_i)) = dp.insert(dp_key, (highest, i)) {
        dp.clear(); // Just in case, don't wanna mess up.
        let pattern_height = highest - previous_height;
        let pieces_per_cycle = i - previous_i;
        let pieces_left = count - i;
        let cycles_left = pieces_left / pieces_per_cycle;
        extra_height = pattern_height * cycles_left as u64;
        // Skip ahead on the list of pieces to add.
        i += pieces_per_cycle * cycles_left;
      }
    }
    i += 1;
  }

  return highest + extra_height;
}

fn stringify_grid(
  blocks: &HashSet<Point>,
  figure: Option<&Figure>,
  r: RangeInclusive<u64>,
) -> String {
  return r
    .rev()
    .flat_map(|y| {
      (0..=WALL)
        .map(move |x| match (x, y, figure) {
          (0, _, _) | (_, 0, _) | (WALL, _, _) => '█',
          (_, _, _) if blocks.contains(&Point { x, y }) => '#',
          (_, _, Some(Figure { coords })) if coords.contains(&Point { x, y }) => '@',
          _ => '.',
        })
        .chain(['\n'])
    })
    .collect();
}

fn _print_grid(blocks: &HashSet<Point>, figure: Option<&Figure>, r: RangeInclusive<u64>) {
  println!("{}", stringify_grid(blocks, figure, r));
}
