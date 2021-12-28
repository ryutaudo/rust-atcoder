use proconio::input;

fn main() {
  input! {
    mut t: [i32; 3],
  }
  t.sort();
  println!("{}", t[0] + t[1]);
}