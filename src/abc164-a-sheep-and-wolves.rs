use proconio::input;

fn main() {
  input! {
    s: i8,
    w: i8,
  }

  if w >= s {
    println!("unsafe");
  } else {
    println!("safe");
  }
}
