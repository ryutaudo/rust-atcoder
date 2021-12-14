use proconio::input;

fn main() {
  input! {
    n: usize,
    d: f64,
    p: [(f64, f64); n],
  }
  let mut count = 0;
  for &(x, y) in &p {
    if x.hypot(y) <= d {
      count += 1;
    }
  }
  println!("{}", count);
}