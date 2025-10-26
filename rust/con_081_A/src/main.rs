use std::io;

fn main() {
    let mut str_1 = String::new();
    io::stdin().read_line(&mut str_1).expect("str_1 error");

    let mut count: u32 = 0;
    for c in str_1.chars() {
        if c == '1' {
            count += 1;
        }
    }

    println!("{}", count);
}
