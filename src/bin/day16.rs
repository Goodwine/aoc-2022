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

#[derive(Clone, Copy)]
struct BitMask(u64, usize);

impl BitMask {
  fn open(&self, valve: &usize) -> Self {
    return BitMask(self.0 | (1 << valve), self.1);
  }
  fn is_open(&self, valve: &usize) -> bool {
    return self.0 & (1 << valve) != 0;
  }
  fn is_closed(&self, valve: &usize) -> bool {
    return !self.is_open(valve);
  }
}

fn p1(data: Vec<Valve>) -> usize {
  let mut dp: HashMap<DPKey, usize> = HashMap::new();
  let open = BitMask(0, data.len());
  return mochila(&mut dp, &data, open, &0, 0, 30);
}

fn p2(data: Vec<Valve>) -> usize {
  return data.len();
}

#[derive(Hash, PartialEq, Eq)]
struct DPKey {
  valve: usize,
  valve_state: u64,
  limit: usize,
}

fn mochila(
  dp: &mut HashMap<DPKey, usize>,
  data: &Vec<Valve>,
  valve_state: BitMask,
  valve: &usize,
  ppm: usize,
  limit: usize,
) -> usize {
  let dp_key = DPKey {
    valve: *valve,
    valve_state: valve_state.0,
    limit,
  };
  if let Some(max) = dp.get(&dp_key) {
    return *max;
  }

  let current = &data[*valve];
  let max_release = current
    .edges
    .iter()
    // Can't walk to valves that are too far away.
    .filter(|(_, cost)| &limit > cost)
    // No point on walking to valves that are already open.
    .filter(|(valve, _)| valve_state.is_closed(valve))
    .map(|(valve, cost)| {
      return mochila(
        dp,
        data,
        valve_state.open(valve),
        valve,
        current.flow + ppm,
        limit - cost,
      ) + cost * ppm;
    })
    .max()
    .unwrap_or(ppm * limit);

  dp.insert(dp_key, max_release);
  return max_release;
}
