use proconio::input;

fn main() {
  input! {
    n: i32,
    x: i32,
    a: [(i32, i32); n],
  }
  let mut index = 1;
  let mut alc = 0;
  for (v, p) in &a {
    alc += v * p;
    // 100で割ると小数点が切り捨てられるため、度数の方に100をかけて比較する
    if alc > x * 100 {
      println!("{}", index);
      return;
    }
    index += 1;
  }
  println!("-1");
}