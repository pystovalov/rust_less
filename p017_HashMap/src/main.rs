use std::collections::HashMap;
macro_rules! line {
    () => {
        println!("----------------");
    };
}
macro_rules! print_var {
    ($ver:expr) => {
        println!("{:?}", $ver);
    };
}

fn main() {
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 84);
    student_grades.insert("Bob", 34);
    print_var!(student_grades);
    line!();

    let names = vec!["Alice", "Bob", "Charlie"];
    let grades = vec![23, 58, 20];
    let students_grades: HashMap<_, _> = names.into_iter().zip(grades.into_iter()).collect();
    print_var!(students_grades);
    line!();
}
