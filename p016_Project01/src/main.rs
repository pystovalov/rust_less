use std::{
    ffi::os_str::Display,
    fmt::{self, Formatter},
};
#[derive(Debug, Clone, Copy)]
enum Operaton {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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

    println!("Калькулятор: примеры вычислений\n");
    for expr in expressions {
        let result = expr.evalute();
        history.push((expr.clone(), result.clone()));
        match result {
            Ok(value) => println!("{} = {}", expr_to_string(&expr), value),
            Err(e) => println!("{} Ошибка: {}", expr_to_string(&expr), e),
        }
    }

    println!("\nИстория (всего записей: {})", history.len());
    for (i, (expr, res)) in history.iter().enumerate() {
        println!("[{}] {} -> {:?}", i + 1, expr_to_string(expr), res);
    }

    let test_char = '*';
    if SUPPORTED_OPS.contains(&test_char) {
        println!("\nСимвол '{}' поддерживается", test_char);
    } else {
        println!("\nСимвол '{}' не поддерживается", test_char);
    }
}
fn expr_to_string(e: &Expression) -> String {
    let op_char = match e.op {
        Operaton::Add => '+',
        Operaton::Sub => '-',
        Operaton::Mul => '*',
        Operaton::Div => '/',
    };
    format!("{} {} {}", e.left, op_char, e.right)
}
