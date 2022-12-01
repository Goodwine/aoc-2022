use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let mut data = aoc::lines(path)
        .split(|line| line.is_empty())
        .map(|elf| elf.iter().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
      data.sort();
      return data;
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<i32>) -> i32 {
  return data.last().unwrap().to_owned();
}

fn p2(data: Vec<i32>) -> i32 {
  return data.iter().rev().take(3).sum();
}
