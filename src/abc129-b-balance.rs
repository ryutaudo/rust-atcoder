use proconio::input;

fn main() {
  input! {
    w: [i32],
  }
  let mut s1 = 0;
  let mut s2 = 0;
  let mut diff = 100;
  for x in &w {
    s2 += x;
  }
  for x in &w {
    s1 += x;
    s2 -= x;
    diff = diff.min((s1 - s2).abs());
    if s1 > s2 {
      break;
    }
  }
  println!("{}", diff);
}