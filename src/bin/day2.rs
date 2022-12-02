use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let mut data = aoc::lines(path)
        .iter()
        .map(|line| line.chars())
        .map(|mut line| (line.next().unwrap(), line.last().unwrap()))
        .collect::<Vec<(char, char)>>();
      data.sort();
      return data;
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<(char, char)>) -> i64 {
  return data.iter().map(|&round| check(round)).sum();
}

/// Checks score from around with  `(opponent, me)`.
///
/// A: Pierda, X: Pierda
/// B: Papel,  Y: Papel
/// C: Tijera, Z: Tijera
fn check((a, b): (char, char)) -> i64 {
  return match (a, b) {
    ('A', 'X') => 1 + 3,
    ('A', 'Y') => 2 + 6,
    ('A', 'Z') => 3 + 0,
    ('B', 'X') => 1 + 0,
    ('B', 'Y') => 2 + 3,
    ('B', 'Z') => 3 + 6,
    ('C', 'X') => 1 + 6,
    ('C', 'Y') => 2 + 0,
    ('C', 'Z') => 3 + 3,
    _ => panic!("impossible!"),
  };
}

fn p2(data: Vec<(char, char)>) -> i64 {
  return data
    .iter()
    // Translate from `(opponent, expected_outcome)` to `(opponent, me)`.
    .map(|&(a, b)| match (a, b) {
      // A: Pierda,   X: Lose,   X: Pierda
      // B: Papel,    Y: Draw,   Y: Papel
      // C: Tijera,   Z: Win,    Z: Tijera
      ('A', 'X') => (a, 'Z'),
      ('A', 'Y') => (a, 'X'),
      ('A', 'Z') => (a, 'Y'),
      ('B', 'X') => (a, 'X'),
      ('B', 'Y') => (a, 'Y'),
      ('B', 'Z') => (a, 'Z'),
      ('C', 'X') => (a, 'Y'),
      ('C', 'Y') => (a, 'Z'),
      ('C', 'Z') => (a, 'X'),
      _ => panic!("impossible!"),
    })
    .map(check)
    .sum();
}
