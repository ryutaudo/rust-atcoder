use proconio::input;

fn is_mm(n: i32) -> bool {
  1 <= n && n <= 12
}

fn resolve(s: &String) -> &str {
  // 文字列メソッドparse()はResult型を返す。
  // "文字列"は文字列リテラルで &strで表される。
  let first: i32 = s[0..2].parse().unwrap();
  let second: i32 = s[2..].parse().unwrap();
  if is_mm(first) && is_mm(second) {
		return "AMBIGUOUS"
  } else if is_mm(first) {
    return "MMYY"
  } else if is_mm(second){
    return "YYMM"
  }
  "NA"
}

fn main() {
  input! {
    s: String,
  }
  let ans = resolve(&s);
  println!("{}", ans);
}