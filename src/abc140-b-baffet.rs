use proconio::input;
 
fn main() {
  input! {
    n: usize,
    a: [usize; n],
    b: [i32; n],
    c: [i32; n-1]
  }
  let mut sum = 0;
  let mut prev_index = 21;
  for &i in a.iter() {
    sum += b[i-1];
    if i == prev_index +1 {
      sum += c[i-2]
    }
    prev_index = i;
  }
  println!("{}", sum);
}