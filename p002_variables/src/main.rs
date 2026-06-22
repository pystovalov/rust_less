fn main() {
    let mut a = 2;
    println!("a = {}", a);
    a = 20;
    println!("a = {}", a);
    //--------------------
    let f_32: f32 = 3.0; //f32
    let f_64 = 2.0; //f64
    println!("f32 = {}", f_32);
    println!("f64 = {}", f_64);
    //--------------------
    let t = true;
    let f: bool = false;
    println!("t = {}", t);
    println!("f = {}", f);
    //--------------------
    let decimal = 98_222;
    let hex = 0xfd;
    let octal = 0o27;
    let binary = 0b1110_0010;
    let byte = b'A';

    println!("decimal = {}", decimal);
    println!("hex = {}", hex);
    println!("octal = {}", octal);
    println!("binary = {}", binary);
    println!("byte = {}", byte);
}
