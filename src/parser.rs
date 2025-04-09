use crate::TokenHandler;

/// Represents a binary expression consisting of two sub-expressions and an operator.
/// 
/// # Fields
/// 
/// * `Box<Expr>` - The left-hand side expression of the binary operation.
/// * `String` - The operator as a string (e.g., "+", "-", "*", "/").
/// * `Box<Expr>` - The right-hand side expression of the binary operation.
/// 
/// This variant is used to model binary operations such as `2 + 3` or `a * b`.
#[derive(Debug)]
pub enum Expr {
    Literal(String), // For true, false, nil, and numbers
    Binary(Box<Expr>, String, Box<Expr>), // For binary expressions like 2 + 3
}

impl Expr {
    pub fn evaluate(&self) -> String {
        match self {
            Expr::Literal(value) => value.clone(), // Return the literal value directly
            Expr::Binary(left, op, right) => {
                format!("({} {} {})", left.evaluate(), op, right.evaluate())
            }
        }
    }
}

pub struct ParserHandler {
    tokens: Vec<String>,
}

impl ParserHandler {
    pub fn new(tokens: &Vec<String>) -> ParserHandler {
        ParserHandler {
            tokens: tokens.clone(),
        }
    }

    /// Parses the tokens and generates an AST.
    pub fn parse(&mut self) -> Result<Expr, String> {
        if self.tokens.is_empty() {
            return Err("No tokens to parse".to_string());
        }

        // Start parsing the expression
        self.parse_expression()
    }

    /// Parses an expression (e.g., 2 + 3).
    fn parse_expression(&mut self) -> Result<Expr, String> {
        // Parse the left-hand side of the expression
        let mut expr = self.parse_primary()?;

        // Handle binary operators (e.g., +, -, etc.)
        while let Some(op) = self.match_operator() {
            let right = self.parse_primary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn match_operator(&mut self) -> Option<String> {
        if let Some(token) = self.tokens.last() {
            if ["+", "-", "*", "/"].contains(&token.as_str()) {
                return self.tokens.pop();
            }
        }
        None
    }

    /// Parses a primary expression (e.g., literals like true, false, nil, or numbers).
    fn parse_primary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.tokens.pop() {
            match token.as_str() {
                "true" | "false" | "nil" => Ok(Expr::Literal(token)),
                _ if token.parse::<f64>().is_ok() => Ok(Expr::Literal(token)), // Handle numbers
                _ => Err(format!("Unexpected token: {}", token)),
            }
        } else {
            Err("No tokens to parse".to_string())
        }
    }
}

pub fn handle_command(command: &str, file_contents: &str) {
    if command == "parse" {
        let mut handler = TokenHandler::default();
        handler.scan_token(&file_contents);

        let tokens = handler.get_tokens().clone();
        let mut parser = ParserHandler::new(&tokens);

        match parser.parse() {
            Ok(ast) => println!("{}", ast.evaluate()), // Use evaluate to print the result
            Err(err) => eprintln!("Parse error: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_true_literal() {
        let tokens = vec!["true".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Ok(Expr::Literal("true".to_string())));
    }

    #[test]
    fn test_parse_false_literal() {
        let tokens = vec!["false".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Ok(Expr::Literal("false".to_string())));
    }

    #[test]
    fn test_parse_nil_literal() {
        let tokens = vec!["nil".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Ok(Expr::Literal("nil".to_string())));
    }

    #[test]
    fn test_parse_unexpected_token() {
        let tokens = vec!["unexpected".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Err("Unexpected token: unexpected".to_string()));
    }

    #[test]
    fn test_parse_empty_input() {
        let tokens: Vec<String> = vec![];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Err("No tokens to parse".to_string()));
    }

    #[test]
    fn test_parse_number_literal() {
        let tokens = vec!["42".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(result, Ok(Expr::Literal("42".to_string())));
    }

    #[test]
    fn test_parse_binary_expression() {
        let tokens = vec!["2".to_string(), "+".to_string(), "3".to_string()];
        let mut parser = ParserHandler::new(&tokens);
        let result = parser.parse();
        assert_eq!(
            result,
            Ok(Expr::Binary(
                Box::new(Expr::Literal("2".to_string())),
                "+".to_string(),
                Box::new(Expr::Literal("3".to_string()))
            ))
        );
    }
}