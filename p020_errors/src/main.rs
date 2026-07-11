fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}
fn main() {
    let result = divide(9, 0);
    match result {
        Ok(val) => println!("result = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
