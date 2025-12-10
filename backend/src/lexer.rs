use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParentheses,
    CloseParentheses,
    Symbol(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
}

/// The lexer transforms text into tokens which are to be fed into the parser.
pub struct Lexer {
    /// The lines with which this lexer has been initialized to transform into tokens.
    /// `Rc` is used as a container around the ExactSizeIterator trait which is used to allow token streaming.
    lines: Rc<RefCell<dyn ExactSizeIterator<Item = String>>>,
    /// Current line being processed
    current_line: String,
    /// Position in the current line
    pos: usize,
}

impl Lexer {
    /// Creates a new lexer from an iterator of lines
    pub fn new(lines: Rc<RefCell<dyn ExactSizeIterator<Item = String>>>) -> Self {
        Lexer {
            lines,
            current_line: String::new(),
            pos: 0,
        }
    }

    /// Creates a lexer from a string
    pub fn from_str(input: &str) -> Self {
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        let iter = lines.into_iter();
        Lexer::new(Rc::new(RefCell::new(iter)))
    }

    /// Creates a lexer from a file
    pub fn from_file<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
    {
        Ok(Lexer::from_str(crate::wasm::read_file(path)?.as_str()))
    }

    /// Gets the next character without consuming it
    fn peek(&self) -> Option<char> {
        self.current_line.chars().nth(self.pos)
    }

    /// Consumes and returns the next character
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skips comments (from semicolon to end of line)
    fn skip_comment(&mut self) {
        if self.peek() == Some(';') {
            self.pos = self.current_line.len();
        }
    }

    /// Loads the next line if available
    fn load_next_line(&mut self) -> bool {
        if let Some(line) = self.lines.borrow_mut().next() {
            self.current_line = line;
            self.pos = 0;
            true
        } else {
            false
        }
    }

    /// Reads a string literal
    fn read_string(&mut self) -> Result<String, String> {
        self.advance(); // consume opening quote
        let mut result = String::new();

        while let Some(ch) = self.advance() {
            match ch {
                '"' => return Ok(result),
                '\\' => {
                    if let Some(escaped) = self.advance() {
                        match escaped {
                            'n' => result.push('\n'),
                            't' => result.push('\t'),
                            'r' => result.push('\r'),
                            '\\' => result.push('\\'),
                            '"' => result.push('"'),
                            _ => result.push(escaped),
                        }
                    }
                }
                _ => result.push(ch),
            }
        }

        Err("Unterminated string".to_string())
    }

    /// Reads a number
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_numeric() || ch == '.' || ch == '-' || ch == '+' || ch == 'e' || ch == 'E' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    /// Reads a symbol or keyword
    fn read_symbol(&mut self) -> String {
        let mut symbol = String::new();

        while let Some(ch) = self.peek() {
            if Self::is_delimiter(ch) {
                break;
            }
            symbol.push(ch);
            self.advance();
        }

        symbol
    }

    /// Checks if a character is a delimiter
    fn is_delimiter(ch: char) -> bool {
        ch.is_whitespace() || "()\"';".contains(ch)
    }

    /// Gets the next token
    pub fn next_token(&mut self) -> Option<Result<Token, String>> {
        loop {
            self.skip_whitespace();

            // If we're at the end of the line, try to load the next one
            if self.pos >= self.current_line.len() {
                if !self.load_next_line() {
                    return None;
                }
                continue;
            }

            self.skip_comment();

            // Check again after skipping comments
            if self.pos >= self.current_line.len() {
                continue;
            }

            match self.peek()? {
                '(' => {
                    self.advance();
                    return Some(Ok(Token::OpenParentheses));
                }
                ')' => {
                    self.advance();
                    return Some(Ok(Token::CloseParentheses));
                }
                '\'' => {
                    self.advance();
                    return Some(Ok(Token::Quote));
                }
                '`' => {
                    self.advance();
                    return Some(Ok(Token::Quasiquote));
                }
                ',' => {
                    self.advance();
                    if self.peek() == Some('@') {
                        self.advance();
                        return Some(Ok(Token::UnquoteSplicing));
                    }
                    return Some(Ok(Token::Unquote));
                }
                '"' => {
                    return Some(self.read_string().map(Token::String));
                }
                '#' => {
                    self.advance();
                    match self.peek() {
                        Some('t') | Some('T') => {
                            self.advance();
                            return Some(Ok(Token::Boolean(true)));
                        }
                        Some('f') | Some('F') => {
                            self.advance();
                            return Some(Ok(Token::Boolean(false)));
                        }
                        _ => return Some(Err("Invalid boolean literal".to_string())),
                    }
                }
                ch if ch.is_numeric() || (ch == '-' || ch == '+') && self.peek_is_numeric() => {
                    return Some(Ok(Token::Number(self.read_number())));
                }
                _ => {
                    let symbol = self.read_symbol();
                    if !symbol.is_empty() {
                        return Some(Ok(Token::Symbol(symbol)));
                    }
                }
            }
        }
    }

    /// Helper to check if the next character after current is numeric
    fn peek_is_numeric(&self) -> bool {
        self.current_line
            .chars()
            .nth(self.pos + 1)
            .map(|ch| ch.is_numeric())
            .unwrap_or(false)
    }

    /// Collects all tokens into a vector
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(token_result) = self.next_token() {
            tokens.push(token_result?);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::from_str("(define x 42)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::OpenParentheses,
                Token::Symbol("define".to_string()),
                Token::Symbol("x".to_string()),
                Token::Number(42.0),
                Token::CloseParentheses,
            ]
        );
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::from_str(r#"(print "hello world")"#);
        let tokens = lexer.tokenize().unwrap();

        assert!(matches!(tokens[2], Token::String(_)));
    }

    #[test]
    fn test_booleans() {
        let mut lexer = Lexer::from_str("#t #f");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, vec![Token::Boolean(true), Token::Boolean(false),]);
    }

    #[test]
    fn test_quote() {
        let mut lexer = Lexer::from_str("'(1 2 3)");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Quote);
    }
}
