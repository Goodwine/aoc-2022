use aoc::{self, CharMask};

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<Vec<char>>) -> usize {
  return data
    .iter()
    .map(|line| {
      let mid = line.len() / 2;
      let (left, right) = line.split_at(mid);
      let mut a: CharMask = left.iter().map(|&x| x).collect();
      let b: CharMask = right.iter().map(|&x| x).collect();

      return a.intersect(&b).into_iter().find(|_| true).unwrap_or(0);
    })
    .sum();
}

fn p2(data: Vec<Vec<char>>) -> usize {
  return data
    .chunks(3)
    .map(|chunk| {
      let mut a: CharMask = chunk[0].iter().map(|&x| x).collect();
      let b: CharMask = chunk[1].iter().map(|&x| x).collect();
      let c: CharMask = chunk[2].iter().map(|&x| x).collect();
      let intersection = a.intersect(&b).intersect(&c);

      return intersection.into_iter().find(|_| true).unwrap_or(0);
    })
    .sum();
}
