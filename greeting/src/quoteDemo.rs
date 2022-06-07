fn main() {
    let n = 33;
    let n_ref1 = &n;
    let n_ref2 = n_ref1;
    println!("{}", std::ptr::eq(n_ref1, n_ref2));
}
