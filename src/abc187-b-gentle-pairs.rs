use proconio::input;

fn main() {
  input! {
    v: [(f32, f32)],
  }
  let mut count = 0;
  for i in 0..v.len() {
    for j in 0..i {
      let (dx, dy) = {
        let (x1, y1) = v[i];
        let (x2, y2) = v[j];
        ((x1 - x2).abs(), (y1 - y2).abs())
      };

      if dx >= dy {
        count += 1;
      }
    }
  }
  println!("{}", count);
}
