use proconio::input;

fn main() {
  input! {
    v: [(i64, i64)],
  }
  let mut sum = 0;
  for (a, b) in &v {
    sum += (a + b) * (b - a + 1) / 2;
  }
  println!("{}", sum);
}
