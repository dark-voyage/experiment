mod environment;
mod error;
mod expression;
mod result;

use environment::Environment;
use error::SchierkeError;
use expression::Expression;
use result::SchierkeResult;

#[derive(Default, PartialEq)]
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
    ) -> Result<SchierkeResult, SchierkeError> {
        let mut _e: Environment = match env {
            Some(e) => e,
            None => self.global.clone(),
        };

        let rev = match exp.clone() {
            Expression::Number(e) => Ok(SchierkeResult::Number(e)),
            Expression::String(e) => Ok(SchierkeResult::String(e)),
            Expression::Variable(e) => match e.len() {
                1 => {
                    let result = _e.lookup(e[0].clone().to_string());
                    Ok(SchierkeResult::Expression(result.unwrap()))
                }
                2 => {
                    let result = _e.define(e[0].clone(), e[1].clone());
                    Ok(SchierkeResult::Expression(result))
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
                    SchierkeResult::Number(n) => n,
                    _ => return Err(SchierkeError::UnknownExpression),
                };

                match exp.clone() {
                    Expression::Add(_) => {
                        result += match self.eval(e[1].clone(), None)? {
                            SchierkeResult::Number(n) => n,
                            _ => return Err(SchierkeError::UnknownExpression),
                        };
                    }
                    Expression::Subtract(_) => {
                        result -= match self.eval(e[1].clone(), None)? {
                            SchierkeResult::Number(n) => n,
                            _ => return Err(SchierkeError::UnknownExpression),
                        };
                    }
                    Expression::Multiply(_) => {
                        result *= match self.eval(e[1].clone(), None)? {
                            SchierkeResult::Number(n) => n,
                            _ => return Err(SchierkeError::UnknownExpression),
                        };
                    }
                    Expression::Divide(_) => {
                        result /= match self.eval(e[1].clone(), None)? {
                            SchierkeResult::Number(n) => n,
                            _ => return Err(SchierkeError::UnknownExpression),
                        };
                    }
                    _ => {
                        return Err(SchierkeError::UnknownExpression);
                    }
                };

                Ok(SchierkeResult::Number(result))
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
    use crate::result::SchierkeResult;

    #[test]
    fn number() {
        let mut schierke = Schierke::new();

        // Number expression
        let exp = Expression::Number(2);
        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(2)));
    }

    #[test]
    fn string() {
        let mut schierke = Schierke::new();

        // String expression
        let exp = Expression::String("hello".to_string());
        assert_eq!(
            schierke.eval(exp, None),
            Ok(SchierkeResult::String("hello".to_string()))
        );
    }

    #[test]
    fn add() {
        let mut schierke = Schierke::new();

        // Add expression
        let exp = Expression::Add(vec![Expression::Number(2), Expression::Number(2)]);
        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(4)));

        // More complex add expression
        let exp = Expression::Add(vec![
            Expression::Number(2),
            Expression::Add(vec![Expression::Number(2), Expression::Number(2)]),
        ]);

        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(6)));
    }

    #[test]
    fn subtract() {
        let mut schierke = Schierke::new();

        // Subtract expression
        let exp = Expression::Subtract(vec![Expression::Number(2), Expression::Number(2)]);
        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(0)));

        // More complex subtract expression
        let exp = Expression::Subtract(vec![
            Expression::Number(2),
            Expression::Subtract(vec![Expression::Number(2), Expression::Number(2)]),
        ]);

        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(2)));
    }

    #[test]
    fn multiply() {
        let mut schierke = Schierke::new();

        // Multiply expression
        let exp = Expression::Multiply(vec![Expression::Number(4), Expression::Number(3)]);
        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(12)));

        // More complex multiply expression
        let exp = Expression::Multiply(vec![
            Expression::Number(4),
            Expression::Multiply(vec![Expression::Number(3), Expression::Number(2)]),
        ]);
        assert_eq!(schierke.eval(exp, None), Ok(SchierkeResult::Number(24)));
    }

    #[test]
    fn variable() {
        let mut schierke = Schierke::new();

        // Variable set expression
        let exp = Expression::Variable(vec![
            Expression::String("x".to_string()),
            Expression::Number(2),
        ]);
        assert_eq!(
            schierke.eval(exp, None),
            Ok(SchierkeResult::Expression(Expression::Number(2)))
        );

        // Variable get expression
        let exp = Expression::Variable(vec![Expression::String("x".to_string())]);
        assert_eq!(
            schierke.eval(exp, None),
            Ok(SchierkeResult::Expression(Expression::Number(2)))
        );
    }
}
