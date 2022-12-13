use std::{cmp::Ordering, str::FromStr, string::ParseError};

use aoc;

fn main() {
  aoc::solve(
    &|path| -> Vec<Packet> {
      return aoc::lines(path)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::from_str(line.as_str()).unwrap())
        .collect();
    },
    &p1,
    &p2,
  );
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
  Number(u8),
  List(Vec<Packet>),
}

impl Packet {
  fn parse(line: &[char]) -> (Self, usize) {
    return match line[0] {
      '[' => Self::parse_list(line),
      '0'..='9' => Self::parse_number(line),
      _ => panic!("impossible"),
    };
  }

  fn parse_list(line: &[char]) -> (Self, usize) {
    assert_eq!(line[0], '[');

    let mut packets = vec![];

    let mut i = 1;
    loop {
      match line[i] {
        '[' | '0'..='9' => {
          let (list, next_pos) = Self::parse(&line[i..]);
          packets.push(list);
          i += next_pos;
        }
        ']' => return (Self::List(packets), i + 1),
        _ => i += 1, // ignore commas
      }
    }
  }

  fn parse_number(line: &[char]) -> (Self, usize) {
    let number_raw = line
      .iter()
      .take_while(|c| match c {
        '0'..='9' => true,
        _ => false,
      })
      .collect::<String>();

    return (Self::Number(number_raw.parse().unwrap()), number_raw.len());
  }
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> Ordering {
    return match (self, other) {
      (Self::Number(a), Self::Number(b)) => a.cmp(b),
      (Self::List(a), Self::List(b)) => a
        .iter()
        .zip(b)
        .map(|(a, b)| a.cmp(b))
        .find(|ordering| ordering.is_ne())
        .unwrap_or(a.len().cmp(&b.len())),
      (Self::List(_), Self::Number(b)) => self.cmp(&Self::List(vec![Self::Number(*b)])),
      (Self::Number(a), Self::List(_)) => Self::List(vec![Self::Number(*a)]).cmp(other),
    };
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}

impl FromStr for Packet {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    return Ok(Self::parse(s.chars().collect::<Vec<char>>().as_slice()).0);
  }
}

fn p1(data: Vec<Packet>) -> usize {
  return data
    .chunks(2)
    .enumerate()
    .filter(|(_, pair)| pair[0].cmp(&pair[1]).is_le())
    .map(|(index, _)| index + 1)
    .sum();
}

fn p2(data: Vec<Packet>) -> usize {
  let mut data = data.clone();

  let p1 = Packet::from_str("[[2]]").unwrap();
  let p2 = Packet::from_str("[[6]]").unwrap();

  data.push(p1.clone());
  data.push(p2.clone());

  data.sort();

  return data
    .iter()
    .enumerate()
    .filter(|(_, packet)| **packet == p1 || **packet == p2)
    .map(|(i, _)| i + 1)
    .reduce(|acc, v| acc * v)
    .unwrap();
}
