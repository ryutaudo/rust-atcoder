use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!(
        "{}",
        if a >= 1 && a <= 9 && b >= 1 && b <= 9 {
            a * b
        } else {
            -1
        }
    );
}
