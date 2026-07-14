pub mod child {
    pub fn child_function() {
        println!("This is the child function.");
    }
    pub fn call_parent_function() {
        super::parent_funcion();
    }
}
pub fn parent_funcion() {
    println!("This is the parent function.");
}
pub use child::child_function;
