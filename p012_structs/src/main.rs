struct Rectungle {
    height: i32,
    width: i32,
}
fn main() {
    let mut r1 = Rectungle {
        height: 20,
        width: 40,
    };
    modify_dimensions2(&mut r1);
    println!("h: {}, w:{}", r1.height, r1.width);
}
fn modify_dimensions2(rect: &mut Rectungle) {
    rect.height = 8888;
    rect.width = 3333;
}
