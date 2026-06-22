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
    //---------------------
    let tupl: (i32, f64, u8) = (500, 3.4, 2);
    let (x, y, z) = tupl;
    let one_var = tupl.0;
    let two_var = tupl.1;
    let three_var = tupl.2;
    println!("x:{} = {}:one_var", x, one_var);
    println!("y:{} = {}:two_var", y, two_var);
    println!("z:{} = {}:three_var", z, three_var);
    //-------------------------
    let arr = [1, 2, 3, 4, 5];
    println!("index 1: {}", arr[1]);
    println!("index 4: {}", arr[4]);
}
