use std::{cmp, num::ParseIntError, str::FromStr};

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      aoc::lines(path)
        .iter()
        .map(|line| {
          let mut parts = line.split(',');
          return (parts.next().unwrap(), parts.next().unwrap());
        })
        .map(|(a, b)| (Range::from_str(a).unwrap(), Range::from_str(b).unwrap()))
        .collect::<Vec<(Range, Range)>>()
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, PartialEq)]
struct Range {
  start: u32,
  end: u32,
}

impl FromStr for Range {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split('-');
    return Ok(Range {
      start: parts.next().unwrap().parse()?,
      end: parts.next().unwrap().parse()?,
    });
  }
}

impl Range {
  fn overlap(&self, other: &Range) -> Option<Range> {
    let range = Range {
      start: cmp::max(self.start, other.start),
      end: cmp::min(self.end, other.end),
    };
    return if range.start > range.end {
      None
    } else {
      Some(range)
    };
  }
}

fn p1(data: Vec<(Range, Range)>) -> usize {
  return data
    .iter()
    .filter(|(a, b)| match &a.overlap(b) {
      None => return false,
      Some(overlap) => return overlap == a || overlap == b,
    })
    .count();
}

fn p2(data: Vec<(Range, Range)>) -> usize {
  return data.iter().filter(|(a, b)| a.overlap(b) != None).count();
}
