fn main() {
    let name = "aaa";
   // name = "aa";
    println!("{}", name);
   // let gender = "bbb";
    //let _age = 18;
    let name = "bbb";
    println!("{}", name);
    let mut a = "aaaaaa";
    a = "bbbbbbbb";
    println!("{}", a);


    let x = 1;

    {
        let x = x + 2;
        println!("x = {}", x);
    }
    {
        let x = x + 1;
        println!("x = {}", x);
    }
        println!("x = {}", x);
}
