use aoc;

fn main() {
  aoc::solve(
    &|path| {
      let mut cwd: Vec<String> = vec![];
      let mut fs: Vec<FsKind> = vec![FsKind::Dir("/".to_string(), None)];

      for line in aoc::lines(path) {
        if line.starts_with("$") {
          if line.starts_with("$ cd ..") {
            cwd.pop();
          } else if line.starts_with("$ cd /") {
            cwd.truncate(0);
          } else if line.starts_with("$ cd") {
            let name = line.strip_prefix("$ cd ").unwrap().to_string();
            cwd.push(name);
          }
          // $ ls doesn't need to be handled.
        } else {
          let mut parts = line.splitn(2, " ");
          let size_or_dir = parts.next().unwrap();
          let name = parts.next().unwrap();
          let full_name = if cwd.is_empty() {
            format!("/{name}")
          } else {
            let cwd = cwd.join("/");
            format!("/{cwd}/{name}")
          };

          if size_or_dir.starts_with("d") {
            fs.push(FsKind::Dir(full_name + "/", None));
          } else {
            fs.push(FsKind::File(full_name, size_or_dir.parse().unwrap()));
          }
        }
      }

      fs.sort_unstable_by_key(|v| v.name());
      update_dir_size(&mut fs, 0);

      return fs;
    },
    &p1,
    &p2,
  );
}

#[derive(Clone, Debug)]
enum FsKind {
  Dir(String, Option<usize>),
  File(String, usize),
}

impl FsKind {
  fn name(&self) -> String {
    match self {
      FsKind::Dir(name, _) => name.to_string(),
      FsKind::File(name, _) => name.to_string(),
    }
  }
  fn size(&self) -> usize {
    match self {
      FsKind::Dir(_, Some(size)) => *size,
      FsKind::File(_, size) => *size,
      _ => panic!("impossible"),
    }
  }
}

fn p1(data: Vec<FsKind>) -> usize {
  return data
    .iter()
    .filter_map(|v| match v {
      FsKind::Dir(_, Some(size)) if *size <= 100_000 => Some(size),
      _ => None,
    })
    .sum();
}

fn p2(data: Vec<FsKind>) -> usize {
  const FS_SPACE: usize = 70_000_000;
  const NEED: usize = 30_000_000;
  let have = FS_SPACE - data[0].size();
  let want = NEED - have;

  return data
    .iter()
    .filter_map(|v| match v {
      FsKind::Dir(_, Some(size)) if *size >= want => Some(*size),
      _ => None,
    })
    .min()
    .unwrap();
}

fn update_dir_size(fs: &mut Vec<FsKind>, current_index: usize) -> usize {
  let prefix = fs[current_index].name();
  let mut total_size = 0;
  let mut i = current_index + 1;
  let mut last_index = fs.len();
  while i < last_index {
    if !fs[i].name().starts_with(&prefix) {
      last_index = i;
      continue;
    }
    match fs[i] {
      FsKind::Dir(_, _) => {
        let next_i = update_dir_size(fs, i);
        total_size += fs[i].size();
        i = next_i;
      }
      FsKind::File(_, size) => {
        total_size += size;
        i += 1;
      }
    }
  }
  fs[current_index] = FsKind::Dir(prefix, Some(total_size));
  return last_index;
}
