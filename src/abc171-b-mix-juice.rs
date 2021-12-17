use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    mut f: [i32; n],
  }
  f.sort();
  let mut total = 0;
  for i in 0..k {
    total += f[i];
  }
  println!("{}", total);
}