fn main() {
    let num = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [5, 4, 3, 2, 1];
    let b = [6; 3];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element_num = num[index];
    let element_a = a[index];
    let element_b = b[index%3];

    println!("index {} is : {}", index, element_num);
    println!("index {} is : {}", index, element_a);
    println!("index {} is : {}", index, element_b);
}

use std::io;
