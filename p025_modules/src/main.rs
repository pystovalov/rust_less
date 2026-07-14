mod math;
fn main() {
    let a: i32 = 6;
    let b: i32 = 5;
    println!("{} + {} = {}", a, b, math::mat::add(&a, &b));
    println!("{} - {} = {}", a, b, math::mat::sub(&a, &b));
    println!("{} * {} = {}", a, b, math::operatons::mul(&a, &b));
    println!(
        "{} / {} = {}",
        a,
        b,
        match math::operatons::div(&a, &b) {
            Some(r) => r,
            None => 0.0,
        }
    );
    math::parent::child::call_parent_function();
    math::parent::child_function();
}
