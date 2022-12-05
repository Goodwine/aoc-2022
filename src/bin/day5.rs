use std::collections::LinkedList;

use aoc;

fn main() {
  aoc::solve(
    &|path| -> (Vec<LinkedList<char>>, Vec<Action>) {
      let lines = aoc::lines(path);
      let mut parts = lines.splitn(2, |line| line.is_empty());
      let mut raw_stacks = parts.next().unwrap().iter().rev();
      let count = raw_stacks
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .count();

      let mut stacks = vec![LinkedList::new(); count];

      for line in raw_stacks.map(|line| line.chars()) {
        for (i, c) in line.skip(1).step_by(4).enumerate() {
          if c == ' ' {
            continue;
          }
          stacks[i].push_back(c)
        }
      }

      return (
        stacks,
        parts.next().unwrap().iter().map(Action::from).collect(),
      );
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Debug)]
struct Action {
  count: usize,
  from: usize,
  to: usize,
}

impl From<&String> for Action {
  fn from(s: &String) -> Self {
    let mut parts = s
      .split_whitespace()
      .filter_map(|part| part.parse::<usize>().ok());

    return Action {
      count: parts.next().unwrap(),
      from: parts.next().unwrap() - 1,
      to: parts.next().unwrap() - 1,
    };
  }
}

fn p1(data: (Vec<LinkedList<char>>, Vec<Action>)) -> String {
  return move_stacks(data, &|c| c.to_owned());
}

fn p2(data: (Vec<LinkedList<char>>, Vec<Action>)) -> String {
  return move_stacks(data, &|c| c.iter().rev().map(|&x| x).collect());
}

fn move_stacks(
  (stacks, steps): (Vec<LinkedList<char>>, Vec<Action>),
  transform: &dyn Fn(&LinkedList<char>) -> LinkedList<char>,
) -> String {
  let mut stacks = stacks.clone();
  for action in steps {
    if stacks[action.from].len() == 0 {
      continue;
    };

    let removed: LinkedList<char> = stacks[action.from]
      .iter()
      .rev()
      .take(action.count)
      .map(|&x| x)
      .collect();

    for v in transform(&removed) {
      stacks[action.from].pop_back();
      stacks[action.to].push_back(v);
    }
  }

  return stacks
    .iter()
    .map(|stack| stack.back().unwrap_or(&' '))
    .collect();
}
