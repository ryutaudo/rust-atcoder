use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
    a: [[i32; w]; h]
  }
  let mut sum = 0;
  let mut min = 101;
  for row in &a {
    for value in row {
      sum += value;
      min = min.min(*value);
    }
  }
  println!("{}", sum - min * h as i32 * w as i32);
}