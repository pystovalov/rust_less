use std::{
    ffi::os_str::Display,
    fmt::{self, Formatter},
};
#[derive(Debug)]
enum Operaton {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum CalcError {
    DivisionByZero,
    UnknownOperation,
}
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Ошибка: Деление на ноль"),
            CalcError::UnknownOperation => write!(f, "Ошибка: Неизвестная операция"),
        }
    }
}
#[derive(Debug)]
struct Expression {
    left: f64,
    op: Operaton,
    right: f64,
}
impl Expression {
    fn new(left: f64, op: Operaton, right: f64) -> Self {
        Self { left, op, right }
    }
    fn evalute(&self) -> Result<f64, CalcError> {
        match self.op {
            Operaton::Add => Ok(self.left + self.right),
            Operaton::Sub => Ok(self.left - self.right),
            Operaton::Mul => Ok(self.left * self.right),
            Operaton::Div => {
                if self.right == 0.0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    Ok(self.left / self.right)
                }
            }
        }
    }
}
fn main() {
    const SUPPORTED_OPS: [char; 4] = ['+', '-', '*', '/'];
    let mut history: Vec<(Expression, Result<f64, CalcError>)> = Vec::new();
    let expressions = vec![
        Expression::new(10.0, Operaton::Add, 5.0),
        Expression::new(10.0, Operaton::Sub, 3.0),
        Expression::new(4.0, Operaton::Mul, 3.4),
        Expression::new(100.0, Operaton::Div, 0.0),
    ];
}
