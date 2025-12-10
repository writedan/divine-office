use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Expr {
    /// Number literal
    Number(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Boolean(bool),
    /// Symbol/identifier
    Symbol(String),
    /// List of expressions
    List(Vec<Expr>),
    /// Quoted expression
    Quote(Box<Expr>),
    /// Quasiquoted expression
    Quasiquote(Box<Expr>),
    /// Unquoted expression
    Unquote(Box<Expr>),
    /// Unquote-splicing expression
    UnquoteSplicing(Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Gets the current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Consumes and returns the current token
    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Checks if we're at the end of the token stream
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Parses a single expression
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            None => Err("Unexpected end of input".to_string()),
            Some(Token::OpenParentheses) => self.parse_list(),
            Some(Token::CloseParentheses) => Err("Unexpected closing parenthesis".to_string()),
            Some(Token::Number(n)) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            Some(Token::String(s)) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::String(string))
            }
            Some(Token::Boolean(b)) => {
                let bool_val = *b;
                self.advance();
                Ok(Expr::Boolean(bool_val))
            }
            Some(Token::Symbol(s)) => {
                let symbol = s.clone();
                self.advance();
                Ok(Expr::Symbol(symbol))
            }
            Some(Token::Quote) => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Quote(Box::new(expr)))
            }
            Some(Token::Quasiquote) => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Quasiquote(Box::new(expr)))
            }
            Some(Token::Unquote) => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Unquote(Box::new(expr)))
            }
            Some(Token::UnquoteSplicing) => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::UnquoteSplicing(Box::new(expr)))
            }
        }
    }

    /// Parses a list (S-expression)
    fn parse_list(&mut self) -> Result<Expr, String> {
        // Consume opening parenthesis
        self.advance();

        let mut elements = Vec::new();

        loop {
            match self.peek() {
                None => return Err("Unclosed list - expected closing parenthesis".to_string()),
                Some(Token::CloseParentheses) => {
                    self.advance();
                    return Ok(Expr::List(elements));
                }
                _ => {
                    let expr = self.parse_expr()?;
                    elements.push(expr);
                }
            }
        }
    }

    /// Parses all expressions in the token stream
    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut expressions = Vec::new();

        while !self.is_at_end() {
            expressions.push(self.parse_expr()?);
        }

        Ok(expressions)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{}", match self {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Boolean(true) => "#t".to_string(),
            Expr::Boolean(false) => "#f".to_string(),
            Expr::Symbol(s) => s.clone(),
            Expr::List(exprs) => {
                let items: Vec<String> = exprs.iter().map(|e| e.to_string()).collect();
                format!("({})", items.join(" "))
            }
            Expr::Quote(expr) => format!("'{}", expr.to_string()),
            Expr::Quasiquote(expr) => format!("`{}", expr.to_string()),
            Expr::Unquote(expr) => format!(",{}", expr.to_string()),
            Expr::UnquoteSplicing(expr) => format!(",@{}", expr.to_string()),
        })
    }
}

#[cfg(test)]
impl Expr {
    /// Checks if the expression is a list
    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
    }

    /// Checks if the expression is an atom (not a list)
    pub fn is_atom(&self) -> bool {
        !self.is_list()
    }

    /// Gets the list elements if this is a list
    pub fn as_list(&self) -> Option<&Vec<Expr>> {
        match self {
            Expr::List(exprs) => Some(exprs),
            _ => None,
        }
    }

    /// Gets the symbol name if this is a symbol
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Expr::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_number() {
        let mut lexer = Lexer::from_str("42");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn test_parse_symbol() {
        let mut lexer = Lexer::from_str("define");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr, Expr::Symbol("define".to_string()));
    }

    #[test]
    fn test_parse_list() {
        let mut lexer = Lexer::from_str("(+ 1 2)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        
        match expr {
            Expr::List(elements) => {
                assert_eq!(elements.len(), 3);
                assert_eq!(elements[0], Expr::Symbol("+".to_string()));
                assert_eq!(elements[1], Expr::Number(1.0));
                assert_eq!(elements[2], Expr::Number(2.0));
            }
            _ => panic!("Expected a list"),
        }
    }

    #[test]
    fn test_parse_nested_list() {
        let mut lexer = Lexer::from_str("(define (square x) (* x x))");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        
        match expr {
            Expr::List(elements) => {
                assert_eq!(elements.len(), 3);
                assert_eq!(elements[0], Expr::Symbol("define".to_string()));
                assert!(elements[1].is_list());
                assert!(elements[2].is_list());
            }
            _ => panic!("Expected a list"),
        }
    }

    #[test]
    fn test_parse_quote() {
        let mut lexer = Lexer::from_str("'(1 2 3)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        
        match expr {
            Expr::Quote(inner) => {
                assert!(inner.is_list());
            }
            _ => panic!("Expected a quote"),
        }
    }

    #[test]
    fn test_parse_multiple_expressions() {
        let mut lexer = Lexer::from_str("(define x 10) (+ x 5)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        assert_eq!(exprs.len(), 2);
    }

    #[test]
    fn test_parse_string() {
        let mut lexer = Lexer::from_str(r#""hello world""#);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr, Expr::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_boolean() {
        let mut lexer = Lexer::from_str("#t #f");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        assert_eq!(exprs[0], Expr::Boolean(true));
        assert_eq!(exprs[1], Expr::Boolean(false));
    }

    #[test]
    fn test_unclosed_list() {
        let mut lexer = Lexer::from_str("(define x");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string() {
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1.0),
            Expr::Number(2.0),
        ]);
        assert_eq!(expr.to_string(), "(+ 1 2)");
    }
}