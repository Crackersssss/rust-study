fn main() {
    let str = String::from("hello world");
    let s = first_world(&str);
    print!("{}", s);
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
