fn main() {
    let number = 3;
    match number {
        1..4 => println!("one"),
        20 => println!("two"),
        30 => println!("three"),
        _ => println!("Something else"),
    }
}
