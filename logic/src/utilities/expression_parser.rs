use evalexpr::eval;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use subst::substitute;

use super::roll_dice;

#[derive(Debug, Default)]
pub struct ExpressionContext {
    data: HashMap<String, String>,
}

impl ExpressionContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// ## set
    /// Set a key-value pair in the context.
    ///
    /// ### Arguments
    /// * `key` - A string slice that holds the key.
    /// * `value` - A string slice that holds the value.
    ///
    /// ### Example
    /// ```
    /// use crate::logic::utilities::expression_parser::ExpressionContext;
    /// let mut context = ExpressionContext::new();
    /// context.set("name", "Alice");
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    /// ## get
    /// Get a value from the context by key.
    /// Returns a reference to the value if it exists.
    /// Returns None if the key does not exist.
    ///
    /// ### Arguments
    /// * `key` - A string slice that holds the key.
    ///
    /// ### Example
    /// ```
    /// use crate::logic::utilities::expression_parser::ExpressionContext;
    /// let mut context = ExpressionContext::new();
    /// context.set("name", "Alice");
    /// let name = context.get("name");
    /// ```
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// ## resolve
    /// Resolve an expression that contains variables.
    /// The expression can contain variables that are resolved from the context.
    /// Returns the resolved expression as a string.
    /// Returns an error if the expression cannot be resolved.
    ///     
    /// ### Arguments
    /// * `expression` - A string slice that holds the expression to resolve.
    ///
    /// ### Example
    /// ```
    /// use crate::logic::utilities::expression_parser::ExpressionContext;
    /// let mut context = ExpressionContext::new();
    /// context.set("name", "Alice");
    /// let result = context.resolve("Hello, $name!");
    /// ```
    pub fn resolve(&self, expression: &str) -> Result<String, Box<dyn std::error::Error>> {
        let result = substitute(expression, &self.data)?;

        Ok(result)
    }

    /// ## resolve_calculate
    /// Resolve an expression that contains arithmetic operations.
    /// The expression can contain variables that are resolved from the context.
    /// The expression can also contain dice rolls.
    /// Returns the result of the expression as an integer.
    /// Returns an error if the expression cannot be resolved.
    ///
    /// ### Arguments
    /// * `expression` - A string slice that holds the expression to resolve.
    ///
    /// ### Example
    /// ```
    /// use crate::logic::utilities::expression_parser::ExpressionContext;
    /// let mut context = ExpressionContext::new();
    /// context.set("name", "Alice");
    /// let result = context.resolve_calculate("2 + 2");
    /// ```
    pub fn resolve_calculate(&self, expression: &str) -> Result<i64, Box<dyn Error>> {
        // first do the substitution as needed.
        let substituted_expression = substitute(expression, &self.data)?;

        // our expression could contain a dice roll, if that's the case we need to resolve it first.
        let re = Regex::new(r".*(?P<roll>roll\(.*\)).*").unwrap();
        // we might *not* have a dice roll, so account for that possibility.
        let resolved_expression = match re.captures(substituted_expression.as_str()) {
            Some(captures) => {
                let roll = captures.name("roll").unwrap().as_str();
                let roll_result = roll_dice(roll.to_string());
                substituted_expression.replace(roll, roll_result.to_string().as_str())
            }
            None => substituted_expression,
        };

        // now that we have resolved our dice roll, we can evaluate the expression.
        let result = match eval(resolved_expression.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        let computed_value = match result.as_number() {
            Ok(v) => v.round() as i64,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(computed_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut context = ExpressionContext::new();
        context.set("name", "Alice");
        context.set("age", "30");

        let result = context
            .resolve("Hello, $name! You are $age years old.")
            .unwrap();

        assert_eq!(result, "Hello, Alice! You are 30 years old.");
    }

    #[test]
    fn test_resolve_calculate() {
        let mut context = ExpressionContext::new();
        context.set("name", "Alice");
        context.set("age", "30");

        let result = context.resolve_calculate("2 + 2").unwrap();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_resolve_calculate_with_context() {
        let mut context = ExpressionContext::new();
        context.set("name", "Alice");
        context.set("age", "30");

        let result = context.resolve_calculate("2 + ${age}").unwrap();

        assert_eq!(result, 32);
    }

    #[test]
    fn test_resolve_calculate_with_dice_roll() {
        let mut context = ExpressionContext::new();
        context.set("name", "Alice");
        context.set("age", "30");

        let result = context.resolve_calculate("2 + roll(2d6)").unwrap();

        assert!((4..=14).contains(&result));
    }

    #[test]
    fn test_resolve_calculate_with_dice_roll_and_context() {
        let mut context = ExpressionContext::new();
        context.set("intelligence", "10");
        context.set("dexterity", "10");

        let result = context
            .resolve_calculate("$dexterity + roll(2d6) + ${intelligence}")
            .unwrap();

        assert!((22..=34).contains(&result));
    }

    #[test]
    fn test_resolve_calculate_with_decimals() {
        let mut context = ExpressionContext::new();
        context.set("intelligence", "10");
        context.set("luck", ".7");

        let result = context
            .resolve_calculate("$intelligence  * ${luck}")
            .unwrap();

        assert!(result == 7);
    }
}
