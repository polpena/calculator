#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Expr {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expr::Integer(value) => *value,
            Expr::UnaryMinus(expr) => -expr.evaluate(),
            Expr::BinOp { lhs, op, rhs } => {
                let left = lhs.evaluate();
                let right = rhs.evaluate();
                match op {
                    Op::Add => left + right,
                    Op::Subtract => left - right,
                    Op::Multiply => left * right,
                    Op::Divide => {
                        if right == 0 {
                            panic!("Division by zero is not allowed");
                        }
                        left / right
                    }
                    Op::Modulo => left % right,
                }
            }
        }
    }
}