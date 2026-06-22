fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("word: {}, length: {}", s, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
