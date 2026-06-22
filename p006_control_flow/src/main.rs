use std::io;
macro_rules! line {
    () => {
        println!("============");
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
    let mut vector: Vec<i32> = Vec::new();
    let mut count = 0;
    loop {
        count += 1;
        println!("Please number positive, negative or zero: ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed");
        let number: i32 = number.trim().parse().expect("Please enter a number");
        vector.push(number);
        control_flow!(number);
        if count == 5 {
            break;
        }
    }
    //------------------
    line!();
    for num in vector {
        if num % 2 == 0 {
            println!("num & 2: {}", num);
        }
    }
    //------------------
    line!();
    let mut flag = true;
    while flag {
        println!("Flag: {}", flag);
        flag = false;
    }
    println!("Flag: {}", flag);
    line!();
    println!("=== EXIT ===");
    line!();
}
