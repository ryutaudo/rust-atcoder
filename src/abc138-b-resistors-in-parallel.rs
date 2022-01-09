use proconio::input;

fn main() {
  input! {
    a: [f64],
  }
  let mut denomitator = 0.0;
  for i in &a {
    denomitator += 1.0 / i;
  }
  println!("{}", 1.0 / denomitator);
}