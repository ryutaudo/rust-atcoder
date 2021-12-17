use proconio::input;

fn main() {
  input! {
    n: u32,
    r: [i32; 2_i32.pow(n)],
  }
  
  let first_half = &r[..r.len() / 2];
  let last_half = &r[r.len() / 2..];

  let first_max = {
    let mut index = 0;
    for i in 1..first_half.len() {
      if first_half[i] > first_half[index] {
        index = i;
      }
    }
    index
  };
  let last_max = {
    let mut index = 0;
    for i in 1..last_half.len() {
      if last_half[i] > last_half[index] {
        index = i;
      }
    }
    index
  };
  if first_half[first_max] > last_half[last_max] {
    println!("{}", last_max + r.len() / 2 + 1);
  } else {
    println!("{}", first_max + 1);
  }
}