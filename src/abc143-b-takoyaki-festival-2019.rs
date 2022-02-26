use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n]
    }
    let mut total = 0;
    for i in 0..n {
        for j in i + 1..n {
            total += d[i] * d[j];
        }
    }
    println!("{}", total);
}
