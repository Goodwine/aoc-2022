use std::collections::HashSet;

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| {
          let mut parts = line.splitn(2, " ");
          return match parts.next().unwrap() {
            "noop" => Operation::Noop,
            "addx" => Operation::AddX(parts.next().unwrap().parse().unwrap()),
            _ => panic!("impossible!"),
          };
        })
        .collect::<Vec<Operation>>()
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<Operation>) -> i64 {
  let mut computer = Computer::new(data);
  let targets: HashSet<usize> = HashSet::from_iter((20..=220).step_by(40));

  let mut sum = 0;
  for _ in 0..=220 {
    computer.step();
    if targets.contains(&computer.clock) {
      sum += computer.clock as i64 * computer.x
    }
  }

  return sum;
}

fn p2(data: Vec<Operation>) -> () {
  let mut computer = Computer::new(data);
  let mut crt: HashSet<(usize, usize)> = HashSet::new();
  const ROWS: usize = 6;
  const COLS: usize = 40;

  for i in 0..=(ROWS * COLS) {
    computer.step();
    let row = i / COLS;
    let col = i % COLS;
    if (computer.x - 1..=computer.x + 1).any(|v| v == col as i64) {
      crt.insert((row, col));
    }
  }

  let result = (0..ROWS)
    .map(|i| {
      (0..COLS)
        .map(|j| if crt.contains(&(i, j)) { '█' } else { ' ' })
        .collect::<String>()
    })
    .reduce(|acc, v| format!("{acc}\n{v}"))
    .unwrap();

  println!("{result}");
}

#[derive(Clone)]
enum Operation {
  AddX(i64),
  Noop,
}

impl Operation {
  fn cycles(&self) -> usize {
    match self {
      Self::AddX(_) => 2,
      Self::Noop => 1,
    }
  }
}

struct Computer {
  x: i64,
  pc: usize,
  clock: usize,
  program: Vec<Operation>,
  delay: usize,
  delayed_op: Option<Operation>,
}

impl Computer {
  fn new(program: Vec<Operation>) -> Self {
    return Computer {
      x: 1,
      pc: 0,
      clock: 0,
      delay: 0,
      program,
      delayed_op: None,
    };
  }

  fn step(&mut self) {
    self.clock += 1;
    if self.delay > 0 {
      self.delay -= 1;
      return;
    }
    if self.pc >= self.program.len() {
      return;
    }
    match self.delayed_op {
      Some(Operation::AddX(v)) => self.x += v,
      _ => (),
    };

    let op = &self.program[self.pc];
    self.delayed_op = Some(op.clone());
    self.delay = op.cycles() - 1;
    self.pc += 1;
  }
}
