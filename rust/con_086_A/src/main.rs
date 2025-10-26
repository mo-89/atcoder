use std::io;

fn main() {
    let mut str_1 = String::new();
    io::stdin().read_line(&mut str_1).expect("str_1 error");

    let str_array: Vec<&str> = str_1.split(' ').collect();
    let str_2 = str_array[0].to_string();
    let str_3 = str_array[1].to_string();

    let num_1: i32 = str_2.trim().parse().expect("i32 conversion error str_2");
    let num_2: i32 = str_3.trim().parse().expect("i32 conversion error str_3");

    let multiply = num_1 * num_2;
 
    if multiply % 2 === 0 {
        println!("{}", "Even");
    } else {
        println!("{}", "Odd");
    }
}
