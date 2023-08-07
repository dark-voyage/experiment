use std::fmt::Debug;

#[derive(PartialEq)]
pub enum Expression {
    Number(i64),
    String(String),
    Add(Vec<Expression>),
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(s) => write!(f, "{}", s),
            Expression::Add(v) => {
                let mut s = String::new();
                for e in v {
                    s.push_str(&format!("{:?}", e));
                }
                write!(f, "{}", s)
            }
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        match self {
            Expression::Number(n) => Expression::Number(*n),
            Expression::String(s) => Expression::String(s.clone()),
            Expression::Add(v) => {
                let mut new_v = Vec::new();
                for e in v {
                    new_v.push(e.clone());
                }
                Expression::Add(new_v)
            }
        }
    }
}