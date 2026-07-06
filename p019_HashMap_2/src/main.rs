use std::collections::HashMap;
fn main() {
    let mut student_grades: HashMap<String, i32> = HashMap::new();
    student_grades.insert(String::from("Alice"), 85);

    let borrowed_grades: &mut HashMap<String, i32> = &mut student_grades;
    borrowed_grades.insert(String::from("Bob"), 85);
    println!("borrowed_grades: {:?}", borrowed_grades);
    println!("origin grades: {:?}", student_grades);
}
