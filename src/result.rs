use crate::expression::Expression;
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum SchierkeResult {
    Number(i64),
    String(String),
    Expression(Expression),
}

impl Debug for SchierkeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchierkeResult::Number(n) => write!(f, "{}", n),
            SchierkeResult::String(s) => write!(f, "{}", s),
            SchierkeResult::Expression(e) => write!(f, "{}", e),
        }
    }
}
