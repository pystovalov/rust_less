fn main() {
    let mut s = "hello";
    println!("{}", s);
    s = "world";
    println!("{}", s);

    let mut str = String::from("hello");
    println!("{}", str);
    str.push_str(", world");
    println!("{}", str);
}
