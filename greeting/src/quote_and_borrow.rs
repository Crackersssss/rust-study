
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(s: &mut String) {
    print!("{}", s);
    s.push_str(", world");
    print!("{}", s);
}
