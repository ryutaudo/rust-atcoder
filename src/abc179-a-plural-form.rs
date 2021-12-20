use proconio::{
  input,
  marker::Chars,
};

fn main() {
  input! {
    s: Chars,
  }
  for c in &s {
    print!("{}", c);
  }
  if s[s.len() - 1] == 's' {
    print!("e");
  }
  println!("s");
}
