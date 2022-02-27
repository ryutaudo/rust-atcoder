use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let mut ans = "No";
    for i in 1..=9 {
        if n % i as f64 == 0.0 && n / i as f64 <= 9.0 {
            ans = "Yes";
            break;
        }
    }
    println!("{}", ans);
}
