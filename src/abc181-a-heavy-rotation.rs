use proconio::input;

fn main() {
  input! {
    n: i8,
  }

  if n % 2 == 0 {
    println!("White");
  } else {
    println!("Black");
  }
}
