use std::io;

fn main() {
    let mut str_1 = String::new();
    let mut str_2 = String::new();

    io::stdin().read_line(&mut str_1).expect("str_1 error");
    io::stdin().read_line(&mut str_2).expect("str_2 error");

    let int_1: i32 = str_1.trim().parse().expect("int conversion error str_1");
    let int_2: i32 = str_2.trim().parse().expect("int conversion error str_2");

    let diff: i32 = int_1 - int_2;
    println!("{}", diff);
}
