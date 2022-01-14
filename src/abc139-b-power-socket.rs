use proconio::input;

fn main() {
  input! {
    a: i32,
    b: i32,
  }
  // 切り上げのときは分子に分母 + 1を足す
  println!("{}", (b - 1 + a - 2) / (a - 1));
}