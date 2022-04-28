fn main() {
    let a = 33_333_33_i32;
    println!("{}", a);

    let n: i32 = std::i32::MAX;
    println!("{}", n);
   // println!("{}", n + 1);
   let b = 0b101_i32;
   let c = 0o17;
   let d = 0xac;
   println!("{}, {}, {}", b, c, d);
   println!("{}", 3_u8.pow(2));

}
