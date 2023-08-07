#![allow(clippy::match_single_binding)]

mod environment;
mod error;
mod expression;

use environment::Environment;
use error::SchierkeError;
use expression::Expression;

/// Schierke is a simple functional lab language
#[derive(Default, PartialEq, Clone)]
pub struct Schierke {
    /// The global environment where global variables are stored
    global: Environment,
}

impl Schierke {
    /// Create a new Schierke instance
    pub fn new() -> Schierke {
        Schierke {
            global: Default::default(),
        }
    }

    /// Evaluate an expression
    ///
    /// # Arguments
    /// * `exp` - The expression to evaluate
    /// * `env` - The variable environment
    ///
    /// # Returns
    /// * `Result<Expression, SchierkeError>` - The result of the expression
    pub fn eval(
        &mut self,
        exp: Expression,
        env: Option<Environment>,
    ) -> Result<Expression, SchierkeError> {
        // Create a new environment where we can store variables
        // If we're given an environment, use that instead
        // This environment is global, so it will be used for all expressions
        let mut _e: Environment = match env {
            Some(e) => e,
            None => self.global.clone(),
        };

        // Evaluate of the expression
        let rev: Result<Expression, SchierkeError> = match exp.clone() {
            // Number expression
            Expression::Number(e) => Ok(Expression::Number(e)),

            // String expression
            Expression::String(e) => Ok(Expression::String(e)),

            // Variable expression
            Expression::Variable(e) => match e.len() {
                // If the length is 1, we're looking up a variable
                1 => {
                    let result = _e.lookup(e[0].clone().to_string());
                    Ok(result.unwrap())
                }

                // If the length is 2, we're defining a variable
                2 => {
                    let result = _e.define(e[0].clone(), self.eval(e[1].clone(), None).unwrap());
                    Ok(result)
                }

                // If the length is anything else, we're doing something wrong
                _ => Err(SchierkeError::TooMuchArguments),
            },

            // Add expression
            Expression::Add(e)

            // Subtract expression
            | Expression::Subtract(e)

            // Multiply expression
            | Expression::Multiply(e)

            // Divide expression
            | Expression::Divide(e) => {
                // If the length is not 2, we're doing something wrong
                if e.len() != 2 {
                    return Err(SchierkeError::UnknownExpression);
                }

                // Create initial result variable to save the result of the expression
                let mut result = 0;

                // Evaluate the first expression and add it to the result
                result += match self.eval(e[0].clone(), None)? {
                    n => match i64::try_from(n) {
                        Ok(n) => n,
                        Err(_) => return Err(SchierkeError::UnknownExpression),
                    },
                };

                // Evaluate the second expression and depending on the expression,
                // add, subtract, multiply, or divide it to the result
                match exp.clone() {
                    // If the second expression is addition, add the result by the second expression
                    Expression::Add(_) => {
                        result += match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }

                    // If the second expression is subtraction, subtract the result by the second expression
                    Expression::Subtract(_) => {
                        result -= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            }
                        }
                    }

                    // If the second expression is multiplication, multiply the result by the second expression
                    Expression::Multiply(_) => {
                        result *= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }

                    // If the second expression is division, divide the result by the second expression
                    Expression::Divide(_) => {
                        result /= match self.eval(e[1].clone(), None)? {
                            n => match i64::try_from(n) {
                                Ok(n) => n,
                                Err(_) => return Err(SchierkeError::UnknownExpression),
                            },
                        };
                    }

                    // If the expression is anything else, we're doing something wrong
                    _ => {
                        return Err(SchierkeError::UnknownExpression);
                    }
                };

                // Return the result
                Ok(Expression::Number(result))
            }
        };

        // Save the environment to the global environment
        // This is so we can use the variables in the global environment
        // after the expression has been evaluated
        self.global.load(_e);

        // Return the result
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
