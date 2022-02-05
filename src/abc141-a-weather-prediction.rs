use proconio::input;

fn main() {
  input! {
    s: String,
  }
  let next = 
    if s == "Sunny" { "Cloudy"} 
    else if s == "Cloudy" { "Rainy" }
    else { "Sunny" };
  println!("{}", next);
}