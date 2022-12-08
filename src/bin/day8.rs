use std::collections::HashSet;

use aoc;

fn main() {
  aoc::solve(
    &|path| -> Vec<Vec<u8>> {
      aoc::lines(path)
        .iter()
        // Offset by 1 so the trees go from 1 to 10. This allows the "unknown height" for a tree
        // to be 0, that way we don't need signed integers.
        .map(|line| line.chars().map(|c| c as u8 - b'0' + 1).collect())
        .collect()
    },
    &p1,
    &p2,
  );
}

type Point = (usize, usize);

/// Walk from one edge to another until the next element is too tall.
///
/// TODO: make it terser? it repeats a lot of code. It basically walks from one
/// end to the other keeping track of the largest element so far and stops when
/// the next element is smaller.
fn p1(data: Vec<Vec<u8>>) -> usize {
  let mut seen: HashSet<Point> = HashSet::new();

  // Check line by line
  for i in 0..data.len() {
    let line = &data[i];
    let mut last = 0;
    for j in 0..line.len() {
      let height = line[j];
      if height <= last {
        continue;
      }
      last = height;
      seen.insert((i, j));
    }

    last = 0;
    for j in (0..line.len()).rev() {
      let height = line[j];
      if height <= last {
        continue;
      }
      last = height;
      seen.insert((i, j));
    }
  }

  // Check row by row
  for j in 0..data[0].len() {
    let mut last = 0;
    for i in 0..data.len() {
      let height = data[i][j];
      if height <= last {
        continue;
      }
      last = height;
      seen.insert((i, j));
    }

    last = 0;
    for i in (0..data.len()).rev() {
      let height = data[i][j];
      if height <= last {
        continue;
      }
      last = height;
      seen.insert((i, j));
    }
  }

  return seen.len();
}

/// TODO: Use DP instead of bruteforce. For example each cell could keep track
/// of each direction "how many are shorter than me". Then on the lookup phase
/// hypothetically could be something like..
///
/// ```
/// let mut right = 1;
/// while right < scenic[i].len() {
///   if data[i][j+right] >= height { break; }
///   right += scenic[i][j+right].right
/// }
/// ```
///
/// Basically a cell will answer how many can be seen from there so instead of
/// checking each of the following cells, we can jump straight to the next cell
/// that can't be seen by it. We repeat while the next cell is shorter.
fn p2(data: Vec<Vec<u8>>) -> usize {
  let mut scenic: Vec<Vec<usize>> = data.iter().map(|line| vec![1; line.len()]).collect();

  for i in 1..(data.len() - 1) {
    for j in 1..(data[i].len() - 1) {
      let height = data[i][j];
      let (left, right) = data[i].split_at(j);
      let left = left
        .iter()
        .skip(1)
        .rev()
        .take_while(|v| **v < height)
        .count();
      let right = right
        .iter()
        .skip(1)
        .rev()
        .skip(1)
        .rev()
        .take_while(|v| **v < height)
        .count();

      let column = data.iter().skip(1).rev().skip(1).rev().map(|line| line[j]);
      let up = column
        .clone()
        .take(i - 1)
        .rev()
        .take_while(|v| *v < height)
        .count();
      let down = column.skip(i).take_while(|v| *v < height).count();

      scenic[i][j] *= left + 1;
      scenic[i][j] *= right + 1;
      scenic[i][j] *= up + 1;
      scenic[i][j] *= down + 1;
    }
  }

  return scenic.iter().flatten().max().unwrap().to_owned();
}

fn _print_seen(data: &Vec<Vec<u8>>, seen: &HashSet<Point>) {
  for i in 0..data.len() {
    for j in 0..data[0].len() {
      if seen.contains(&(i, j)) {
        print!(".");
      } else {
        print!("{}", data[i][j]);
      }
    }
    println!();
  }
}

fn _print_scenic(scenic: &Vec<Vec<usize>>) {
  for i in 0..scenic.len() {
    println!("{:?}", scenic[i]);
  }
}
