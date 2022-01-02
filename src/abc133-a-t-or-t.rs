use proconio::input;

fn main() {
  input! {
    n: i32,
    a: i32,
    b: i32,
  }
  println!("{}", if n * a > b { b } else { n * a });
}
