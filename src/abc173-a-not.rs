use proconio::input;

fn main(){
  input! {
    x: i8,
  }

  if x == 0 {
    println!("1");
  } else {
    println!("0");
  }
}