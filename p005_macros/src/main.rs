use std::io;
macro_rules! line {
    () => {
        println!("-------------------");
    };
}
macro_rules! hello_name {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}
fn main() {
    println!("Please enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed read line");
    let name = name.trim();
    line!();
    hello_name!(name);
    line!();
}
