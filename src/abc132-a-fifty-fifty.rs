use proconio::{input, marker::Chars};

fn main() {
  input! {
    s: Chars,
  }
  if s[0] == s[1] && s[2] == s[3] && s[0] != s[2] ||
    s[0] == s[2] && s[1] == s[3] && s[0] != s[1] ||
    s[0] == s[3] && s[1] == s[2] && s[0] != s[1] {
      println!("Yes");
      return;
    }
  println!("No");
}