macro_rules! debug_vec {
    ($v:expr) => {
        println!("{:?}", $v);
        println!("----------------");
    };
}
fn main() {
    let mut v = vec![1, 3, 4];
    debug_vec!(v);
    v.push(22);
    debug_vec!(v);
    v.pop();
    debug_vec!(v);
    v.insert(2, 55);
    debug_vec!(v);
    println!("index 1: {}", v[1]);

    match v.get(6) {
        Some(var) => println!("element {}", var),
        None => println!("element not fined"),
    }

    for item in &v {
        print!("{} ", item);
    }
    debug_vec!(v);

    println!("len vector: {}", v.len());
    println!("is empty: {}", v.is_empty());
    println!("conteints: {}", v.contains(&4));
}
