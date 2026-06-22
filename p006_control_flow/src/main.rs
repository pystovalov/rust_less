use std::io;
macro_rules! line {
    () => {
        println!("=============");
    };
}
macro_rules! control_flow {
    ($number:expr) => {
        if $number < 0 {
            println!("number negative");
        } else if $number > 0 {
            println!("number positive");
        } else {
            println!("ZERO");
        }
    };
}
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Please number positive, negative or zero: ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed");
        let number: i32 = number.trim().parse().expect("Please enter a number");
        control_flow!(number);
        if number % 2 == 0 && number > 0 {
            println!("Number devided to 2");
        }
        if count == 5 {
            break;
        }
    }
    line!();
    println!("EXIT");
    line!();
}
