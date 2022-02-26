use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", if a > 2 * b { a - 2 * b } else { 0 });
}
