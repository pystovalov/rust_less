#[derive(Debug)]
struct RectName {
    name: String,
}

#[derive(Debug)]
struct Rectungle {
    rec_name: RectName,
    height: i32,
    width: i32,
}
fn main() {
    let recnm = RectName {
        name: String::from("Rectangle One"),
    };
    let mut r1 = Rectungle {
        rec_name: recnm,
        height: 20,
        width: 40,
    };
    modify_dimensions2(&mut r1);
    println!("h: {}, w:{}", r1.height, r1.width);
    println!("#==============#");
    //println!("{}", recnm.name);
    println!(
        "name: {}, H: {}, W: {}",
        r1.rec_name.name, r1.height, r1.width
    );
    println!("#==============#");
    println!("{:?}", r1);
}
fn modify_dimensions2(rect: &mut Rectungle) {
    rect.height = 8888;
    rect.width = 3333;
}
