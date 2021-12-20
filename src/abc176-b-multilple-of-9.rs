use proconio::{
  input,
  marker::Bytes,
};

fn main() {
  input! {
    n: Bytes,
  }
  let mut sum = 0;
  for c in &n {
    let digit = c - b'0';
    sum += digit as i32;
  }
  println!("{}", if sum % 9 == 0 { "Yes" } else { "No" });
}