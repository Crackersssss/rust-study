fn main() {
    let x = 26;
    let a = if x < 20 {
        println!("x < 20");
        x + 10
    } else if x < 30 {
        println!("x < 30");
        x + 5
    } else {
        println!("x >= 30");
        x
    };
    println!("{}", a);

    'outer: loop {
        'inner: while true {
            break 'outer;
        }
    }
}
