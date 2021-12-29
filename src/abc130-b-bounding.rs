use proconio::input;

fn main() {
  input! {
    n: i32,
    mut x: i32,
    b: [i32; n],
  }
  let mut count = 1;
  for l in &b {
    x -= l;
    if x < 0 {
      break;
    }
    count += 1;
  }
  println!("{}", count);
}
