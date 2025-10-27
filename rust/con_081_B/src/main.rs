// use std::io;

// fn main() {
//     // const num_count_str = String::new();
//     const num_count_str: &str = io::stdin().read_line(&mut num_count_str).expect("num_count error");
//     let mut num_array_str = String::new();
//     // io::stdin().read_line(&mut num_count_str).expect("num_count error");
//     io::stdin().read_line(&mut num_array_str).expect("num_array_str error");

//     const num_count: usize = num_count_str.parse().expect("conversion error");

//     // let mut original_array: Vec<&str> = num_array_str.split(' ').collect();
//     let mut original_array: [u32; num_count] = num_array_str.split(' ').collect();
    
//     let mut divide_count: u32 = 0;
//     for num in 1..num_count {
//         let mut index: usize = num - 1;
//         if original_array[index] % 2 == 0 {
//             original_array[index] = original_array.index % 2;
//         } else {
//             break;
//         }

//         if num == num_count {
//             divide_count += 1;
//         }
//     }

//     println!("{}", num_count);
// }

// 解けなかった
