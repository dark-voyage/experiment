use std::fmt::Debug;

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Number(i64),
    String(String),
    Variable(Vec<Expression>),
    Add(Vec<Expression>),
    Subtract(Vec<Expression>),
    Multiply(Vec<Expression>),
    Divide(Vec<Expression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(s) => write!(f, "{}", s),
            Expression::Variable(v) => write!(f, "{}", v[0]),
            Expression::Add(v) => write!(f, "{} + {}", v[0], v[1]),
            Expression::Subtract(v) => write!(f, "{} - {}", v[0], v[1]),
            Expression::Multiply(v) => write!(f, "{} * {}", v[0], v[1]),
            Expression::Divide(v) => write!(f, "{} / {}", v[0], v[1]),
        }
    }
}
