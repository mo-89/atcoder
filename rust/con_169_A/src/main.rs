use proconio::input;

fn main() {
    input! {
        m: [u32; 2],
    }

    let sum = m[0] * m[1];
    println!("{}", sum);
}
