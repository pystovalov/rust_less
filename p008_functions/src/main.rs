fn main() {
    greet("Alexander");
    println!("{}", sqrt(11));
    let num = 2;
    let x = match num {
        3 => println!("{}", num),
        _ => println!("------"),
    };
    x
}
fn greet(name: &str) {
    println!("Hello, {}", name);
}
fn sqrt(number: i32) -> i32 {
    number * number
}
