fn main() {
    let arr = [11, 22, 33];
    println!("{}", arr.len());
    println!("{:?}", arr.repeat(2));
    println!("{:?}", arr.contains(&22));

    println!("{}", ["aaa","bbb"].join("ccc"));
    println!("{:?}", [[1,2],[3,4]].join(&0));
}
