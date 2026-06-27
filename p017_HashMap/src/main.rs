use std::collections::HashMap;
macro_rules! line {
    () => {
        println!("----------------");
    };
}

fn main() {
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 84);
    student_grades.insert("Bob", 34);
    println!("{:?}", student_grades);
    line!();
}
