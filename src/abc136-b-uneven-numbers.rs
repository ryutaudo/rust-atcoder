use proconio::input;

fn main() {
  input! {
    n: i32,
  }
  let mut ans = 0;
  for i in 1..=n {
    let digit = i.to_string().chars().count();
    if digit % 2 != 0 {
      ans += 1;
    }
  }
  println!("{}", ans);
}