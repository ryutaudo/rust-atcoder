use proconio::input;

fn main() {
  input! {
    a: i32,
    p: i32,
  }
  println!("{}", (a * 3 + p) / 2);
}