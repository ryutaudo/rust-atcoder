use proconio::input;

fn main() {
  input! {
    v: [i64],
  }
  let mut sum = 0;
  let mut max = 0;
  for i in v {
    max = if max > i { max } else { i };
    sum += max - i;
  }
  println!("{}", sum);
}
