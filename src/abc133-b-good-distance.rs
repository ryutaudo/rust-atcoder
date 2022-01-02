use proconio::input;

fn main() {
  input! {
    n: usize,
    d: usize,
    x: [[i32; d]; n],
  }
  let mut count = 0;
  for i in 0..n {
    for j in i + 1..n {
      let mut dist = 0;
      for k in 0..d {
        dist += (x[i][k] - x[j][k]).pow(2);
      }
      for l in 0..=dist {
        if l * l == dist {
          count += 1;
          break;
        }
      }
    }
  }
  println!("{}", count);
}
