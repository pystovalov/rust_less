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
    let mut students_grades: HashMap<_, _> = names.into_iter().zip(grades.into_iter()).collect();
    print_var!(students_grades);
    line!();
    let mut grade = students_grades.get("Bob");
    match grade {
        Some(g) => println!("Bob grade is {}", g),
        None => println!("No grade found for Bob"),
    }
    line!();
    println!("Alice grade is{}", students_grades["Alice"]);
    line!();
    grade = students_grades.get("Alice");
    match grade {
        Some(g) => println!(" < Alice grade: {}", g),
        None => println!("Alice not found"),
    }
    students_grades.insert("Alice", 99);
    grade = students_grades.get("Alice");
    match grade {
        Some(g) => println!(" > Alice grade: {}", g),
        None => println!("Alice not found"),
    }
    line!();
}
