use proconio::input;

fn main() {
  input! {
    n: i32,
    m: i32,
    t: i32,
    mut v: [(i32, i32); m],
  }
  v.push((t, t));
  let mut battery = n;
  let mut leave = 0;
  for &(a, b) in &v {
    battery -= a - leave;
    if battery <= 0 {
      break;
    }
    battery = n.min(battery + b - a);
    leave = b;
  }

  if battery > 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}