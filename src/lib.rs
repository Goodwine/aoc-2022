use std::{
  fmt::{Debug, Display},
  fs,
  time::Instant,
};

fn measure_time<T, X>(f: &dyn Fn(X) -> T, arg: X) -> (T, Duration) {
  let start = Instant::now();
  return (f(arg), Duration(start.elapsed()));
}

fn measure_and_print<T, X>(name: &str, f: &dyn Fn(X) -> T, arg: X)
where
  T: Debug,
{
  let (result, duration) = measure_time(f, arg);
  println!("{name} ({duration}): {:#?}", result);
}

pub fn lines(path: String) -> Vec<String> {
  return fs::read_to_string(path)
    .unwrap()
    .trim()
    .split("\n")
    .map(|line| line.to_string())
    .collect();
}

fn internal_solve<D, T1, T2>(
  size: &str,
  reader: &dyn Fn(String) -> D,
  p1: &dyn Fn(D) -> T1,
  p2: &dyn Fn(D) -> T2,
) where
  T1: Debug,
  T2: Debug,
  D: Clone,
{
  let current_exe = std::env::current_exe()
    .unwrap()
    .file_name()
    .unwrap()
    .to_os_string()
    .into_string()
    .unwrap();

  let (data, duration) = measure_time(reader, format!("./data/{current_exe}/{size}.txt"));
  println!("\nread ({duration}) --- {size}");
  measure_and_print("part 1", &p1, data.clone());
  measure_and_print("part 2", &p2, data);
}

pub fn solve<D, T1, T2>(reader: &dyn Fn(String) -> D, p1: &dyn Fn(D) -> T1, p2: &dyn Fn(D) -> T2)
where
  T1: Debug,
  T2: Debug,
  D: Clone,
{
  internal_solve("small", reader, p1, p2);
  internal_solve("input", reader, p1, p2);
}

struct Duration(std::time::Duration);

impl Display for Duration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let seconds: u128 = self.0.as_secs().into();
    let millis = self.0.as_millis() % 1_000;
    let micros = self.0.as_micros() % 1_000;
    let nanos = self.0.as_nanos() % 1_000;

    let output = [(seconds, "s"), (millis, "m"), (micros, "µ"), (nanos, "n")]
      .iter()
      .filter_map(|&(n, u)| {
        if n == 0 {
          None
        } else {
          Some(format!("{n}{u}"))
        }
      })
      .collect::<Vec<String>>()
      .join(" ");

    return write!(f, "{output}");
  }
}
