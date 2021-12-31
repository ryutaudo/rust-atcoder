use proconio::input;

fn main() {
  input! {
    n: usize,
    p: [i32; n],
  }
  let mut count = 0;
  for i in 0..n - 2 {
    if p[i] < p[i + 1] && p[i + 1] < p[i + 2] || p[i + 2] < p[i + 1] && p[i + 1] < p[i] {
      count += 1;
    }
  }
  println!("{}", count);
}