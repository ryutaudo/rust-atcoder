use proconio::input;

fn main() {
  input! {
    n: usize,
    r: [(String, i32); n],
  }
  // sort() メソッドは昇順に並び替えるので、点数を最大値から引いたものでベクタを作り直す
  let mut order = Vec::new();
  for i in 0..n {
    // Stringはコピー不可能なので参照する
    order.push((&r[i].0, 100 - r[i].1, i + 1));
  }
  order.sort();
  for (_, _, i) in &order {
    println!("{}", i);
  }
}