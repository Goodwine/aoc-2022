use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let data: Vec<Monkey> = aoc::lines(path)
        .split(|line| line.is_empty())
        .map(|lines| Monkey::from_lines(lines))
        .collect();

      // This is technically not the LCM, but all numbers happen to be prime.
      let lcm = data
        .iter()
        .map(|m| m.modulo)
        .reduce(|acc, v| acc * v)
        .unwrap();

      return (data, lcm);
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Debug)]
struct Monkey {
  items: Vec<u64>,
  operator: Operator,
  /// X = X op CHANGE.
  /// When `change` is None it means X = X op X.
  change: Option<u64>,
  modulo: u64,
  pass: usize,
  fail: usize,
  counter: usize,
}

impl Monkey {
  /// Parses:
  /// ```
  /// Monkey 0:
  ///   Starting items: [ITEMS]
  ///   Operation: new = old [OP] [CHANGE]
  ///   Test: divisible by [MODULO]
  ///     If true: throw to monkey [PASS]
  ///     If false: throw to monkey [FAIL]
  /// ```
  fn from_lines(lines: &[String]) -> Self {
    let mut op = lines[2]
      .split("= old ")
      .last()
      .unwrap()
      .split_ascii_whitespace();
    return Self {
      items: lines[1]
        .split(": ")
        .last()
        .unwrap()
        .split(", ")
        .map(|n| n.parse().unwrap())
        .collect(),
      operator: match op.next().unwrap() {
        "*" => Operator::Multiply,
        "+" => Operator::Add,
        _ => panic!("impossible"),
      },
      change: op.next().unwrap().parse().ok(),
      modulo: lines[3].split("by ").last().unwrap().parse().unwrap(),
      pass: lines[4].split("monkey ").last().unwrap().parse().unwrap(),
      fail: lines[5].split("monkey ").last().unwrap().parse().unwrap(),
      counter: 0,
    };
  }

  fn inspect_all(&mut self, worry_divisor: &u64, worry_mod: &u64) -> Vec<(u64, usize)> {
    self.counter += self.items.len();
    let updates = self
      .items
      .iter()
      .map(|item| {
        let worry_level = (self.operator.apply(item, self.change) / worry_divisor) % worry_mod;
        let target = if worry_level % &self.modulo == 0 {
          self.pass
        } else {
          self.fail
        };
        return (worry_level, target);
      })
      .collect();
    self.items.clear();
    return updates;
  }
}

#[derive(Clone, Debug)]
enum Operator {
  Add,
  Multiply,
}

impl Operator {
  fn apply(&self, a: &u64, b: Option<u64>) -> u64 {
    return match (self, b) {
      (Operator::Add, None) => a * 2,
      (Operator::Add, Some(b)) => a + b,
      (Operator::Multiply, None) => a * a,
      (Operator::Multiply, Some(b)) => a * b,
    };
  }
}

fn p1((data, lcm): (Vec<Monkey>, u64)) -> usize {
  return simulate(data, 20, 3, lcm);
}

fn p2((data, lcm): (Vec<Monkey>, u64)) -> usize {
  return simulate(data, 10_000, 1, lcm);
}

fn simulate(data: Vec<Monkey>, rounds: usize, worry_divisor: u64, worry_mod: u64) -> usize {
  let mut data = data.clone();

  for _round in 0..rounds {
    for i in 0..data.len() {
      let updates = data[i].inspect_all(&worry_divisor, &worry_mod);
      for (worry_level, target) in updates {
        data[target].items.push(worry_level);
      }
    }
  }

  let mut counters: Vec<usize> = data.iter().map(|monkey| monkey.counter).collect();
  counters.sort_unstable();
  return counters[counters.len() - 1] * counters[counters.len() - 2];
}
