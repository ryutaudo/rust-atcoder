use proconio::input;

fn main() {
  input! {
    a: i32,
    b: i32,
  }
  let mut ans = a + b;
  ans = ans.max(a - b);
  ans = ans.max(a * b);
  println!("{}", ans);
}