use aoc;

fn main() {
  aoc::solve(
    &|path| aoc::lines(path).first().unwrap().to_owned(),
    &p1,
    &p2,
  );
}

fn p1(data: String) -> usize {
  return find(&data, 4);
}

fn p2(data: String) -> usize {
  return find(&data, 14);
}

fn find(data: &String, n: usize) -> usize {
  return data
    .chars()
    .collect::<Vec<char>>()
    .windows(n)
    .enumerate()
    .find_map(|(i, window)| {
      if window
        .iter()
        .map(|&x| x)
        .collect::<aoc::CharMask>()
        .into_iter()
        .count()
        == n
      {
        Some(i + n)
      } else {
        None
      }
    })
    .unwrap();
}
