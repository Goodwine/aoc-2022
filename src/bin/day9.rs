use std::collections::HashSet;

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| {
          let (dir, number) = line.split_at(2);
          let number = number.parse().unwrap();
          return match dir {
            "U " => Direction::Up(number),
            "D " => Direction::Down(number),
            "R " => Direction::Right(number),
            "L " => Direction::Left(number),
            _ => panic!("imposible"),
          };
        })
        .collect::<Vec<Direction>>()
    },
    &p1,
    &p2,
  );
}

#[derive(Clone)]
enum Direction {
  Up(u8),
  Down(u8),
  Left(u8),
  Right(u8),
}

impl Direction {
  fn value(&self) -> u8 {
    match self {
      Self::Up(v) => *v,
      Self::Down(v) => *v,
      Self::Left(v) => *v,
      Self::Right(v) => *v,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl Point {
  fn together(&self, &Point(x2, y2): &Point) -> bool {
    let Point(x1, y1) = self;
    return x1.abs_diff(x2) <= 1 && y1.abs_diff(y2) <= 1;
  }

  fn best_move(&self, head: &Point) -> Point {
    // Sometimes the best move is not to move.
    if self.together(head) {
      return *self;
    }

    let Point(x1, y1) = self;

    // Try out all possible moves, filter for whichever are actually possible,
    // and then find the closest.
    return (-1..=1)
      .flat_map(|i| (-1..=1).map(move |j| Point(x1 + i, y1 + j)))
      .filter(|p| p.together(head))
      .min_by_key(|p| p.manhattan_dist(head))
      .unwrap();
  }

  fn manhattan_dist(&self, &Point(x2, y2): &Point) -> u32 {
    let Point(x1, y1) = self;
    return x2.abs_diff(*x1) + y2.abs_diff(*y1);
  }
}

fn p1(data: Vec<Direction>) -> usize {
  return snake(data, 2);
}

fn p2(data: Vec<Direction>) -> usize {
  return snake(data, 10);
}

fn snake(data: Vec<Direction>, length: usize) -> usize {
  let mut snake = vec![Point(0, 0); length];
  let mut seen: HashSet<Point> = HashSet::new();

  for dir in data {
    for _ in 0..dir.value() {
      match dir {
        Direction::Up(_) => snake[0].1 += 1,
        Direction::Down(_) => snake[0].1 -= 1,
        Direction::Right(_) => snake[0].0 += 1,
        Direction::Left(_) => snake[0].0 -= 1,
      }
      for i in 1..length {
        snake[i] = snake[i].best_move(&snake[i - 1]);
      }

      seen.insert(*snake.last().unwrap());
    }
  }

  return seen.len();
}
