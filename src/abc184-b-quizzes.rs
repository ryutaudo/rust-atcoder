use proconio::{
  input,
  marker::Chars,
};

fn main() {
  input! {
    _: i32,
    mut x: i32,
    s: Chars,
  }
  for &c in &s {
    if c == 'o' {
      x += 1;
    } else if x > 0 {
      x -= 1;
    }
  }
  println!("{}", x);
}
