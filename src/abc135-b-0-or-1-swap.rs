use proconio::input;

fn check(v: &Vec<i32>) -> bool {
  for i in 1..v.len() {
    if v[i - 1] > v[i] {
      return false;
    }
  }
  return true;
}

fn main() {
  input! {
    n: usize,
    mut p: [i32; n],
  }
  let mut i = 100;
  let mut j = 100;

  for k in 1..n {
    if i == 100 && p[k - 1] > p[k] {
      i = k - 1;
    } else if  p[k - 1] > p[k] {
      j = k;
      break;
    }
  }
  if i == 100 && j == 100 {
    println!("YES");
    return;
  } else if i != 100 && j == 100 {
    println!("NO");
    return;
  }
  p.swap(i, j);
  println!("{}", if check(&p) { "YES" } else { "NO" });
}

