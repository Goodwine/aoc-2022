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
  fn open(self, valve: &usize) -> Self {
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
    (&0, std::usize::MAX),
    0,
    DAYS,
  );
}

fn p2(data: Vec<Valve>) -> usize {
  let valve_state = BitMask(0);
  const DAYS: usize = 30;
  const ELEPHANT_TRAINING: usize = 4;
  let start = data.iter().position(|v| v.name == "AA").unwrap();
  return parallel_mochila(
    &data,
    &start,
    &start,
    valve_state,
    DAYS - ELEPHANT_TRAINING,
    0,
  );
}

fn parallel_mochila(
  data: &Vec<Valve>,
  a: &usize,
  b: &usize,
  valve_state: BitMask,
  limit: usize,
  total_flow: usize,
) -> usize {
  let valve_a = &data[*a];
  let valve_b = &data[*b];

  // Pay the cost of opening the valve.
  let adjust = valve_a.flow.clamp(0, 1);
  let limit = limit - adjust;
  let total_flow = total_flow + (valve_a.flow + valve_b.flow) * limit;

  let a_edges = valve_a.next_edges(&valve_state, &limit);
  let b_edges = valve_b.next_edges(&valve_state, &limit);

  let singles = if let ([a], b @ [..]) | (b @ [..], [a]) = (a_edges.as_slice(), b_edges.as_slice())
  {
    let valve_a = &data[a.0];
    b.iter()
      .filter(|(v, _)| *v == a.0)
      .chain([a])
      // -1 to pay the cost of opening the last valve.
      .map(|(_, cost)| limit - cost - 1)
      .map(|limit| total_flow + valve_a.flow * limit)
      .max()
  } else {
    None
  };

  let with_limbo = a_edges
    .iter()
    .flat_map(|a| b_edges.iter().map(move |b| (a, b)))
    .filter(|((a, _), (b, _))| a != b)
    // Flip to make sure cost for "a" is always lower than or equal to "b"
    .map(|(a, b)| if a.1 < b.1 { (a, b) } else { (b, a) })
    .map(|((a, cost_a), (b, cost_b))| {
      mochila(
        data,
        valve_state.open(a).open(b),
        a,
        (b, cost_b - cost_a),
        total_flow,
        limit - cost_a,
      )
    })
    .max();

  // We must return total_flow rather than zero in the event nothing else is found
  // because this whole thing is accumulating in the arguments.
  return singles.max(with_limbo).unwrap_or(total_flow);
}

fn mochila(
  data: &Vec<Valve>,
  valve_state: BitMask,
  active: &usize,
  (limbo, limbo_left): (&usize, usize),
  total_flow: usize,
  limit: usize,
) -> usize {
  if limbo_left == 0 {
    return parallel_mochila(data, active, limbo, valve_state, limit, total_flow);
  }

  let current = &data[*active];
  let adjust = current.flow.clamp(0, 1);
  let limit = limit - adjust;
  let limbo_left = limbo_left - adjust;
  let total_flow = total_flow + current.flow * limit;

  let max_release = current
    .next_edges(&valve_state, &limit)
    .iter()
    .map(|(valve, cost)| {
      let cost = cost;
      let valve_state = valve_state.open(valve);

      if limbo_left >= *cost {
        return mochila(
          data,
          valve_state,
          valve,
          (limbo, limbo_left - cost),
          total_flow,
          limit - cost,
        );
      }

      return mochila(
        data,
        valve_state,
        limbo,
        (valve, cost - limbo_left),
        total_flow,
        limit - limbo_left,
      );
    })
    .max()
    .unwrap_or(total_flow);

  return max_release;
}
