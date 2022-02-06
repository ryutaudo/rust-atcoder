use proconio::input;

fn main() {
  input! {
    n: f32,
  }

  if n % 2.0 == 0.0 {
    println!("{}", 1.0 / 2.0);
  } else {
    println!("{}", (n + 1.0) / 2.0 / n);
  }
}