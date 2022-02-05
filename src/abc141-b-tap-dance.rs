use proconio::input;

fn main() {
  input! {
    s: String,
  }

  let mut count = 1;

  for c in s.chars() {
    if (count % 2 == 0 && c == 'R') || count % 2 == 1 && c == 'L'{
      println!("No");
      return
    }
    count += 1;
  }

  println!("Yes");
}