use proconio::input;

fn main() {
  input! {
    v: [f64]
  }
  let manhattan_distance = {
    let mut d = 0.;
    for &x in &v {
      d += x.abs();
    }
    d
  };
  let euclid_distance = {
    let mut d = 0.;
    for &x in &v {
        d += x * x;
    }
    d.sqrt()
  };
  let chebyshev_distance = {
    let mut d = 0.;
    for &x in &v {
      d = x.abs().max(d);
    }
    d
  };
  println!("{}", manhattan_distance);
  println!("{}", euclid_distance);
  println!("{}", chebyshev_distance);
}
