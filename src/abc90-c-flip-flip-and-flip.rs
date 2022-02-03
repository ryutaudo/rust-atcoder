use proconio::input;

fn main() {
  input! {
    input_n: i64,
    input_m: i64,
  }
  let n = if input_n == 1 { 1 } else { input_n - 2 };
  let m = if input_m == 1 { 1 } else { input_m - 2 };
  println!("{}", n * m);
}