use crate::error::SchierkeError;
use crate::expression::Expression;
use std::collections::HashMap;

/// Environment
/// Place where we store variables
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Environment {
    /// Variables HashMap
    variables: HashMap<String, Expression>,
}

impl Environment {
    /// Create a new environment instance, if one is not provided
    pub fn new(env: Option<Environment>) -> Environment {
        match env {
            Some(e) => e,
            None => Default::default(),
        }
    }

    /// Setter | store a variable in the environment
    pub fn define(
        &mut self,
        name: Expression,
        value: Expression
    ) -> Expression {
        self.variables.insert(
            name.to_string(),
            value.clone(),
        );
        value
    }

    /// Getter | lookup a variable in the environment
    pub fn lookup(&self, name: String) -> Result<Expression, SchierkeError> {
        match self.variables.get(name.as_str()) {
            Some(e) => Ok(e.clone()),
            None => Err(SchierkeError::UndefinedVariable),
        }
    }

    /// Load changes from another environment
    pub fn load(&mut self, env: Environment) {
        self.variables = env.variables;
    }
}
