#![allow(clippy::match_single_binding)]

mod environment;
mod error;
mod expression;

use environment::Environment;
use error::SchierkeError;
use expression::Expression;

#[derive(Default, PartialEq, Clone)]
pub struct Schierke {
    global: Environment,
}

impl Schierke {
    pub fn new() -> Schierke {
        Schierke {
            global: Environment::default(),
        }
    }

    pub fn eval(
        &mut self,
        exp: Expression,
        env: Option<Environment>,
    ) -> Result<Expression, SchierkeError> {
        let mut _e: Environment = match env {
            Some(e) => e,
            None => self.global.clone(),
        };

        let rev: Result<Expression, SchierkeError> = match exp.clone() {
            Expression::Number(e) => Ok(Expression::Number(e)),
            Expression::String(e) => Ok(Expression::String(e)),
            Expression::Variable(e) => match e.len() {
                1 => {
                    let result = _e.lookup(e[0].clone().to_string());
                    Ok(result.unwrap())
                }
                2 => {
                    let result = _e.define(e[0].clone(), self.eval(e[1].clone(), None).unwrap());
                    Ok(result)
                }
                _ => Err(SchierkeError::TooMuchArguments),
            },
            Expression::Add(e)
            | Expression::Subtract(e)
            | Expression::Multiply(e)
            | Expression::Divide(e) => {
                if e.len() != 2 {
                    return Err(SchierkeError::UnknownExpression);
                }

                let mut result = 0;

                result += match self.eval(e[0].clone(), None)? {
                    n => match i64::try_from(n) {
                        Ok(n) => n,
                        Err(_) => return Err(SchierkeError::UnknownExpression),
                    },
                };

                match exp.clone() {
                    Expression::Add(_) => {
                        result += match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }
                    Expression::Subtract(_) => {
                        result -= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            }
                        }
                    }
                    Expression::Multiply(_) => {
                        result *= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }
                    Expression::Divide(_) => {
                        result /= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }
                    _ => {
                        return Err(SchierkeError::UnknownExpression);
                    }
                };

                Ok(Expression::Number(result))
            }
        };

        self.global.load(_e);

        rev
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use super::Schierke;

    #[test]
    fn number() {
        let mut schierke = Schierke::new();

        // Number expression
        let expression = Expression::Number(2);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(2)));
    }

    #[test]
    fn string() {
        let mut schierke = Schierke::new();

        // String expression
        let expression = Expression::String("hello".to_string());
        assert_eq!(
            schierke.eval(expression, None),
            Ok(Expression::String("hello".to_string()))
        );
    }

    #[test]
    fn add() {
        let mut schierke = Schierke::new();

        // Add expression
        let expression = Expression::Add(vec![Expression::Number(2), Expression::Number(2)]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(4)));
    }

    #[test]
    fn add_complex() {
        let mut schierke = Schierke::new();

        // More complex add expression
        let expression = Expression::Add(vec![
            Expression::Number(2),
            Expression::Add(vec![Expression::Number(2), Expression::Number(2)]),
        ]);

        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(6)));
    }

    #[test]
    fn subtract() {
        let mut schierke = Schierke::new();

        // Subtract expression
        let expression = Expression::Subtract(vec![Expression::Number(2), Expression::Number(2)]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(0)));
    }

    #[test]
    fn subtract_complex() {
        let mut schierke = Schierke::new();

        // More complex subtract expression
        let expression = Expression::Subtract(vec![
            Expression::Number(2),
            Expression::Subtract(vec![Expression::Number(2), Expression::Number(2)]),
        ]);

        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(2)));
    }

    #[test]
    fn multiply() {
        let mut schierke = Schierke::new();

        // Multiply expression
        let expression = Expression::Multiply(vec![Expression::Number(4), Expression::Number(3)]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(12)));
    }

    #[test]
    fn multiply_complex() {
        let mut schierke = Schierke::new();

        // More complex multiply expression
        let expression = Expression::Multiply(vec![
            Expression::Number(4),
            Expression::Multiply(vec![Expression::Number(3), Expression::Number(2)]),
        ]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(24)));
    }

    #[test]
    fn variable() {
        let mut schierke = Schierke::new();

        // Variable set expression
        let expression = Expression::Variable(vec![
            Expression::String("x".to_string()),
            Expression::Number(2),
        ]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(2)));

        // Variable get expression
        let expression = Expression::Variable(vec![Expression::String("x".to_string())]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(2)));
    }

    #[test]
    fn variable_complex() {
        let mut schierke = Schierke::new();

        // Variable set expression but way more complex
        let expression = Expression::Variable(vec![
            Expression::String("x".to_string()),
            Expression::Add(vec![Expression::Number(2), Expression::Add(vec![Expression::Number(2), Expression::Number(2)])]),
        ]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(6)));

        // Variable get expression but way more complex
        let expression = Expression::Variable(vec![Expression::String("x".to_string())]);
        assert_eq!(schierke.eval(expression, None), Ok(Expression::Number(6)));
    }
}
