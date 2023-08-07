mod error;
mod expression;
mod result;

use error::SchierkeError;
use expression::Expression;
use result::SchierkeResult;

#[derive(Default, PartialEq)]
pub struct Schierke {}

impl Schierke {
    pub fn new() -> Schierke {
        Schierke {}
    }

    pub fn eval(&self, exp: Expression) -> Result<SchierkeResult, SchierkeError> {
        match exp {
            Expression::Number(n) => Ok(SchierkeResult::Number(n)),
            Expression::Add(a) => {
                if a.len() != 2 {
                    return Err(SchierkeError::UnknownExpression);
                }

                let mut result = 0;

                result += match self.eval(a[0].clone())? {
                    SchierkeResult::Number(n) => n,
                    _ => return Err(SchierkeError::UnknownExpression),
                };

                result += match self.eval(a[1].clone())? {
                    SchierkeResult::Number(n) => n,
                    _ => return Err(SchierkeError::UnknownExpression),
                };

                Ok(SchierkeResult::Number(result))
            }
            Expression::String(s) => Ok(SchierkeResult::String(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use super::Schierke;

    #[test]
    fn sample() {
        let schierke = Schierke::new();

        assert_eq!(schierke.eval(Expression::Number(2)), Ok(super::SchierkeResult::Number(2)));
        assert_eq!(schierke.eval(Expression::String("hello".to_string())), Ok(super::SchierkeResult::String("hello".to_string())));
        assert_eq!(schierke.eval(Expression::Add(vec![Expression::Number(2), Expression::Number(2)])), Ok(super::SchierkeResult::Number(4)));

        // Let's get crazy!
        assert_eq!(schierke.eval(Expression::Add(vec![Expression::Number(2), Expression::Add(vec![Expression::Number(2), Expression::Number(2)])])), Ok(super::SchierkeResult::Number(6)));
    }
}
