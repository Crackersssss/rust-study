fn main() {
    let a = if true {
        println!("true");
        33
    } else {
        println!("false");
        44
    };
    println!("a is {}", a);
}
