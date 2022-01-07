use proconio::input;

fn main() {
  input! {
    k: i32,
    x: i32,
  }
  for i in (x - k + 1)..=(x + k - 1) {
    if i.abs() <= 1000000 {
      print!("{} ", i);
    }
  }
}