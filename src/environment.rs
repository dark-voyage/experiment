use crate::error::SchierkeError;
use crate::expression::Expression;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Environment {
    variables: HashMap<String, Expression>,
}

impl Environment {
    pub fn new(env: Option<Environment>) -> Environment {
        match env {
            Some(e) => e,
            None => Environment::default(),
        }
    }

    pub fn define(&mut self, name: Expression, value: Expression) -> Expression {
        self.variables.insert(name.to_string(), value.clone());
        value
    }

    pub fn lookup(&self, name: String) -> Result<Expression, SchierkeError> {
        match self.variables.get(name.as_str()) {
            Some(e) => Ok(e.clone()),
            None => Err(SchierkeError::UndefinedVariable),
        }
    }
}
