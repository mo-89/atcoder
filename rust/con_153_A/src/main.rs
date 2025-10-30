use proconio::input;

fn main() {
    input!{
        m: [u32; 2]
    }

    let quotient = m[0] / m[1];
    let remainder = m[0] % m[1];
    let count = if remainder == 0 {
        quotient
    } else {
        quotient + 1
    };

    println!("{}", count);
}
