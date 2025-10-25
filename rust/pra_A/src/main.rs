use std::io;

fn main() {

    let mut str_1 = String::new();
    let mut str_2_tmp = String::new();
    // let mut str_2 = String::new();
    // let mut str_3 = String::new();
    let mut str_4 = String::new();

    io::stdin().read_line(&mut str_1).expect("str_1 error");
    io::stdin().read_line(&mut str_2_tmp).expect("str_2_tmp error");
    io::stdin().read_line(&mut str_4).expect("str_4 error");

    let int_1: i32 = str_1.trim().parse().expect("int conversion error str_1");

    let str_array: Vec<&str> = str_2_tmp.split(' ').collect();

    let str_2 = str_array[0].to_string();
    let str_3 = str_array[1].to_string();
    let int_2: i32 = str_2.trim().parse().expect("int conversion error str_2");
    let int_3: i32 = str_3.trim().parse().expect("int conversion error str_3");    

    let sum: i32 = int_1 + int_2 + int_3;

    println!("{} {}", sum, str_4);

}
