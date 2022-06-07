fn f0() {
    println!("Hello world");
}

fn f1(a: i32, b: i32) {
    println!("a: {}, b: {}", a, b);
}

fn f2(a: i32, b: i32) -> i32 {
    return a + b;
}

fn f3(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    f0();
    f1(1, 2);
    println!("f2: {}", f2(1, 2));
    println!("f3: {}", f3(1, 2));
}
