use proconio::input;

fn main() {
  input! {
    v: [i32],
  }

  let mut max = -1;
  let mut ans = -1;

  for i in 2..1000 {
    let mut gcd_ness = 0;
    for a in &v {
      if a % i == 0 {
        gcd_ness += 1;
      }
    }
    if max < gcd_ness {
      max = gcd_ness;
      ans = i;
    }
  }

  println!("{}", ans);
}