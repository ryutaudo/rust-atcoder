use proconio::input;

fn main() {
  input! {
    n: i32,
  }
  match n {
    0..=125 => println!("4"),
    126..=211 => println!("6"),
    212..=214 => println!("8"),
    _ => println!("No matching"),
  }
}