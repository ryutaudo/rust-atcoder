use proconio::input;

fn main() {
  input! {
    n: i32,
    d: i32,
  }
  // 切り上げのときは割られる数をa、 割る数をbとした時、
  // (a + (b-1)) / b
  println!("{}", (n + 2 * d) / (2 * d + 1));
}
