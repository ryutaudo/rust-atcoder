use proconio::input;

fn main() {
  input! {
    n: i32,
    l: i32,
  }
  let mut min_taste = l + n - 1;
  for i in 1..=n {
    let taste = l + i - 1;
    if min_taste.abs() > taste.abs() {
      min_taste = taste;
    }
  }
  println!("{}", (2 * l + n - 1) * n / 2 - min_taste);
}