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
            Expr::Literal(value) => value.clone(),
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

    /// Parses a primary expression (e.g., literals like true, false, nil, or numbers).
    fn parse_primary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.tokens.pop() {
            match token.as_str() {
                "true" | "false" | "nil" => Ok(Expr::Literal(token)),
                _ => {
                    if let Ok(_) = token.parse::<f64>() {
                        Ok(Expr::Literal(token))
                    } else {
                        Err(format!("Unexpected token: {}", token))
                    }
                }
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    /// Matches and consumes a binary operator (e.g., +, -, etc.).
    fn match_operator(&mut self) -> Option<String> {
        if let Some(token) = self.tokens.last() {
            if ["+", "-", "*", "/"].contains(&token.as_str()) {
                return self.tokens.pop();
            }
        }
        None
    }
}