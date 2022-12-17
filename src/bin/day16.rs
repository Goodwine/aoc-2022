use std::collections::{HashMap, HashSet, LinkedList};

use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let valves = aoc::lines(path)
        .iter()
        .map(|line| {
          let mut parts = line
            .split_terminator([' ', '=', ',', ';'])
            .filter(|part| !part.is_empty())
            .skip(1);
          let name = parts.next().unwrap().to_string();
          let mut parts = parts.skip(3);
          let flow = parts.next().unwrap().parse().unwrap();
          let parts = parts.skip(4);
          let valves = parts.map(|s| s.to_string()).collect::<Vec<String>>();

          return (name, flow, valves);
        })
        .collect::<Vec<_>>();

      let name_to_index = valves
        .iter()
        .enumerate()
        .map(|(i, (name, ..))| (name.clone(), i))
        .collect::<HashMap<_, _>>();

      return valves
        .iter()
        .map(|(name, flow, _)| {
          let edges = valves
            .iter()
            .map(|(other, ..)| shortest_path(name, other, &valves, &name_to_index))
            .enumerate()
            .filter(|&(i, cost)| cost > 0 && valves[i].1 > 0)
            .collect::<Vec<_>>();
          return Valve {
            name: name.clone(),
            flow: *flow,
            edges,
          };
        })
        .collect::<Vec<_>>();
    },
    &p1,
    &p2,
  );
}

fn shortest_path<T>(
  from: &String,
  to: &String,
  valves: &Vec<(String, T, Vec<String>)>,
  name_to_index: &HashMap<String, usize>,
) -> usize {
  let mut seen: HashSet<&String> = HashSet::new();
  let mut queue: LinkedList<(usize, &String)> = LinkedList::from([(0, from)]);
  while !queue.is_empty() {
    let (cost, current) = queue.pop_front().unwrap();
    if current == to {
      return cost;
    }

    if !seen.insert(current) {
      continue;
    }

    let (.., connects_to) = &valves[*name_to_index.get(current).unwrap()];
    for name in connects_to {
      queue.push_back((cost + 1, name))
    }
  }

  panic!("not a fully connected graph!");
}

#[derive(Hash, Clone, Debug)]
struct Valve {
  name: String,
  flow: usize,
  edges: Vec<(usize, usize)>,
}

impl Valve {
  fn next_edges(&self, valve_state: &BitMask, limit: &usize) -> Vec<&(usize, usize)> {
    return self
      .edges
      .iter()
      // Can't walk to valves that are too far away.
      .filter(|(_, cost)| limit > cost)
      // No point on walking to valves that are already open.
      .filter(|(valve, _)| valve_state.is_closed(valve))
      .collect();
  }
}

#[derive(Clone, Copy)]
struct BitMask(u64);

impl BitMask {
  fn open(&self, valve: &usize) -> Self {
    return BitMask(self.0 | (1 << valve));
  }
  fn is_open(&self, valve: &usize) -> bool {
    return self.0 & (1 << valve) != 0;
  }
  fn is_closed(&self, valve: &usize) -> bool {
    return !self.is_open(valve);
  }
}

fn p1(data: Vec<Valve>) -> usize {
  let valve_state = BitMask(0);
  const DAYS: usize = 30;
  let start = data.iter().position(|v| v.name == "AA").unwrap();
  return mochila(
    &data,
    valve_state,
    &start,
    // This way we simulate only having one worker because it'll never become available.
    &(0, std::usize::MAX),
    0,
    DAYS - 1,
  );
}

fn p2(data: Vec<Valve>) -> usize {
  let valve_state = BitMask(0);
  const DAYS: usize = 30;
  let start = data.iter().position(|v| v.name == "AA").unwrap();
  return mochila(&data, valve_state, &start, &(start, 0), 0, DAYS - 1);
}

fn mochila(
  data: &Vec<Valve>,
  valve_state: BitMask,
  active: &usize,
  limbo: &(usize, usize),
  total_flow: usize,
  limit: usize,
) -> usize {
  let current = &data[*active];

  let max_release = current
    .next_edges(&valve_state, &limit)
    .iter()
    .map(|(valve, cost)| {
      // This adjust is necessary because while we only select valves with certain
      // flow rate, we may start with a valve that has zero flow rate.
      let adjust = current.flow.clamp(0, 1);
      return mochila(
        data,
        valve_state.open(valve),
        valve,
        limbo,
        total_flow + current.flow * limit,
        limit - cost - adjust,
      );
    })
    .max()
    .unwrap_or(total_flow + current.flow * limit);

  return max_release;
}
