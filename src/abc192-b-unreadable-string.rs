use proconio::{
  input,
  marker::Chars,
};

fn main() {
  input! {
    s: Chars,
  }
  let mut ans = true;
  for i in 0..s.len() {
    ans &= !((i % 2 == 0) ^ s[i].is_ascii_lowercase());
  }
  println!("{}", if ans { "Yes" } else { "No" });
}
