use proconio::input;

fn main() {
  input! {
    sx: f32,
    sy: f32,
    gx: f32,
    gy: f32,
  }

  println!("{}", sy * (gx - sx) / (gy + sy) + sx);
}