use proconio::input;

fn main() {
  input! {
    a: i32,
    b: i32,
  }
  let mut ans = 0;
  if a >= 13 {
    ans = b;
  } else if 6 <= a && a <= 12 {
    ans = b / 2;
  }
  println!("{}", ans);
}