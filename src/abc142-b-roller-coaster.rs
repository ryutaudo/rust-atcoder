use proconio::input;

fn main() {
  input! {
    n: usize,
    k: i32,
    vh: [i32; n],
  }
  let mut ans = 0;
  for h in vh {
    if h >= k {
      ans += 1;
    }
  }
  println!("{}", ans);
}