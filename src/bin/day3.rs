use std::collections::HashSet;

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| line.chars())
        .map(|line| {
          line
            .map(|c| match c {
              'a'..='z' => (c as u8) - b'a' + 1,
              'A'..='Z' => (c as u8) - b'A' + 1 + 26,
              _ => panic!("impossible!"),
            })
            .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
    },
    &p1,
    &p2,
  );
}

fn p1(data: Vec<Vec<u8>>) -> u32 {
  return data
    .iter()
    .map(|line| {
      let mid = line.len() / 2;
      let (left, right) = line.split_at(mid);
      let a: HashSet<&u8> = left.iter().collect();
      let b: HashSet<&u8> = right.iter().collect();
      return a.intersection(&b).next().unwrap().to_owned().to_owned() as u32;
    })
    .sum();
}

fn p2(data: Vec<Vec<u8>>) -> u32 {
  return data
    .chunks(3)
    .map(|chunk| {
      let mut a: HashSet<&u8> = chunk[0].iter().collect();
      let b: HashSet<&u8> = chunk[1].iter().collect();
      let c: HashSet<&u8> = chunk[2].iter().collect();
      a.retain(|&k| b.contains(k) && c.contains(k));

      return a.iter().next().unwrap().to_owned().to_owned() as u32;
    })
    .sum();
}
