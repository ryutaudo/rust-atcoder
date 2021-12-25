use proconio::{input, marker::Chars};

fn main() {
  input! {
    n: usize,
    k: usize,
    mut s: Chars,
  }
  // .to_ascii_lowercase でchar型を返す
  // .to_lowercase はIterable型を返す
  s[k - 1] = s[k - 1].to_ascii_lowercase();
  for c in s {
    print!("{}",c);
  }
}