use crate::parser::Expr;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
    List(Vec<Value>),
    Nil,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(true) => "#t".to_string(),
            Value::Boolean(false) => "#f".to_string(),
            Value::Symbol(s) => s.clone(),
            Value::List(values) => {
                let items: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                format!("({})", items.join(" "))
            }
            Value::Nil => "()".to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(false) | Value::Nil => false,
            _ => true,
        }
    }
}

pub struct Environment {
    bindings: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.bindings.get(name)
    }
}

pub struct Runtime {
    env: Environment,
    returned_values: Vec<Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            env: Environment::new(),
            returned_values: Vec::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Nil => Ok(Value::Nil),
            
            Expr::Symbol(name) => {
                self.env.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined symbol: {}", name))
            }
            
            Expr::Quote(expr) => self.quote_to_value(expr),
            
            Expr::List(elements) => {
                if elements.is_empty() {
                    return Ok(Value::Nil);
                }

                if let Some(Expr::Symbol(op)) = elements.first() {
                    match op.as_str() {
                        "let" => self.eval_let(&elements[1..]),
                        "eq" => self.eval_eq(&elements[1..]),
                        "if" => self.eval_if(&elements[1..]),
                        "return" => self.eval_return(&elements[1..]),
                        "quote" => self.eval_quote(&elements[1..]),
                        "+" => self.eval_arithmetic(&elements[1..], |a, b| a + b),
                        "-" => self.eval_arithmetic(&elements[1..], |a, b| a - b),
                        "*" => self.eval_arithmetic(&elements[1..], |a, b| a * b),
                        "/" => self.eval_arithmetic(&elements[1..], |a, b| a / b),
                        "<" => self.eval_comparison(&elements[1..], |a, b| a < b),
                        ">" => self.eval_comparison(&elements[1..], |a, b| a > b),
                        "<=" => self.eval_comparison(&elements[1..], |a, b| a <= b),
                        ">=" => self.eval_comparison(&elements[1..], |a, b| a >= b),
                        "and" => self.eval_and(&elements[1..]),
                        "or" => self.eval_or(&elements[1..]),
                        "not" => self.eval_not(&elements[1..]),
                        _ => self.eval_application(elements),
                    }
                } else {
                    self.eval_application(elements)
                }
            }
            
            _ => Err(format!("Cannot evaluate expression: {:?}", expr)),
        }
    }

    fn eval_let(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("let requires exactly 2 arguments: (let <symbol> <value>)".to_string());
        }

        let name = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err("First argument to let must be a symbol".to_string()),
        };

        let value = self.eval(&args[1])?;
        self.env.define(name, value.clone());
        Ok(value)
    }

    fn eval_eq(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("eq requires exactly 2 arguments".to_string());
        }

        let val1 = self.eval(&args[0])?;
        let val2 = self.eval(&args[1])?;
        
        Ok(Value::Boolean(val1 == val2))
    }

    fn eval_if(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("if requires exactly 3 arguments: (if <cond> <then> <else>)".to_string());
        }

        let condition = self.eval(&args[0])?;
        
        if condition.is_truthy() {
            self.eval(&args[1])
        } else {
            self.eval(&args[2])
        }
    }

    fn eval_return(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("return requires exactly 1 argument".to_string());
        }

        let value = self.eval(&args[0])?;
        self.returned_values.push(value.clone());
        Ok(value)
    }

    fn eval_quote(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("quote requires exactly 1 argument".to_string());
        }

        self.quote_to_value(&args[0])
    }

    fn quote_to_value(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Symbol(s) => Ok(Value::Symbol(s.clone())),
            Expr::Nil => Ok(Value::Nil),
            Expr::List(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.quote_to_value(expr)?);
                }
                Ok(Value::List(values))
            }
            _ => Err("Cannot quote this expression type".to_string()),
        }
    }

    fn eval_arithmetic<F>(&mut self, args: &[Expr], op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        if args.is_empty() {
            return Err("Arithmetic operation requires at least 1 argument".to_string());
        }

        let first = self.eval(&args[0])?;
        let mut result = match first {
            Value::Number(n) => n,
            _ => return Err("Arithmetic operations require numbers".to_string()),
        };

        for arg in &args[1..] {
            let val = self.eval(arg)?;
            match val {
                Value::Number(n) => result = op(result, n),
                _ => return Err("Arithmetic operations require numbers".to_string()),
            }
        }

        Ok(Value::Number(result))
    }

    fn eval_comparison<F>(&mut self, args: &[Expr], op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        if args.len() != 2 {
            return Err("Comparison requires exactly 2 arguments".to_string());
        }

        let val1 = self.eval(&args[0])?;
        let val2 = self.eval(&args[1])?;

        match (val1, val2) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(op(n1, n2))),
            _ => Err("Comparison requires numbers".to_string()),
        }
    }

    fn eval_and(&mut self, args: &[Expr]) -> Result<Value, String> {
        for arg in args {
            let val = self.eval(arg)?;
            if !val.is_truthy() {
                return Ok(Value::Boolean(false));
            }
        }
        Ok(Value::Boolean(true))
    }

    fn eval_or(&mut self, args: &[Expr]) -> Result<Value, String> {
        for arg in args {
            let val = self.eval(arg)?;
            if val.is_truthy() {
                return Ok(Value::Boolean(true));
            }
        }
        Ok(Value::Boolean(false))
    }

    fn eval_not(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("not requires exactly 1 argument".to_string());
        }

        let val = self.eval(&args[0])?;
        Ok(Value::Boolean(!val.is_truthy()))
    }

    fn eval_application(&mut self, elements: &[Expr]) -> Result<Value, String> {
        Err(format!("Unknown function or special form: {:?}", elements[0]))
    }

    pub fn run(&mut self, exprs: Vec<Expr>) -> Result<Vec<Value>, String> {
        for expr in exprs {
            self.eval(&expr)?;
        }
        Ok(self.returned_values.clone())
    }

    pub fn get_returned_values(&self) -> &Vec<Value> {
        &self.returned_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval_str(input: &str) -> Result<Vec<Value>, String> {
        let mut lexer = Lexer::from_str(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        let mut runtime = Runtime::new();
        runtime.run(exprs)
    }

    #[test]
    fn test_let() {
        let mut lexer = Lexer::from_str("(let x 40)");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        let mut runtime = Runtime::new();
        runtime.run(exprs).unwrap();
        assert_eq!(runtime.env.get("x"), Some(&Value::Number(40.0)));
    }

    #[test]
    fn test_let_and_eq() {
        let mut lexer = Lexer::from_str("(let x 40) (return (eq x 40))");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        let mut runtime = Runtime::new();
        let results = runtime.run(exprs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Value::Boolean(true));
    }

    #[test]
    fn test_if_true() {
        let results = eval_str("(return (if #t 1 2))").unwrap();
        assert_eq!(results[0], Value::Number(1.0));
    }

    #[test]
    fn test_if_false() {
        let results = eval_str("(return (if #f 1 2))").unwrap();
        assert_eq!(results[0], Value::Number(2.0));
    }

    #[test]
    fn test_if_with_expression() {
        let results = eval_str("(let x 40) (return (if (eq x 40) 100 200))").unwrap();
        assert_eq!(results[0], Value::Number(100.0));
    }

    #[test]
    fn test_arithmetic() {
        let results = eval_str("(return (+ 1 2 3))").unwrap();
        assert_eq!(results[0], Value::Number(6.0));
    }

    #[test]
    fn test_nested_arithmetic() {
        let results = eval_str("(return (* (+ 1 2) (- 5 2)))").unwrap();
        assert_eq!(results[0], Value::Number(9.0));
    }

    #[test]
    fn test_comparison() {
        let results = eval_str("(return (< 1 2))").unwrap();
        assert_eq!(results[0], Value::Boolean(true));
        
        let results = eval_str("(return (> 1 2))").unwrap();
        assert_eq!(results[0], Value::Boolean(false));
    }

    #[test]
    fn test_complex_expression() {
        let results = eval_str("(let x 10) (let y 20) (return (if (< x y) (+ x y) (* x y)))").unwrap();
        assert_eq!(results[0], Value::Number(30.0));
    }

    #[test]
    fn test_string_operations() {
        let results = eval_str(r#"(let greeting "hello") (return (eq greeting "hello"))"#).unwrap();
        assert_eq!(results[0], Value::Boolean(true));
    }

    #[test]
    fn test_logical_and() {
        let results = eval_str("(return (and #t #t #t))").unwrap();
        assert_eq!(results[0], Value::Boolean(true));
        
        let results = eval_str("(return (and #t #f #t))").unwrap();
        assert_eq!(results[0], Value::Boolean(false));
    }

    #[test]
    fn test_logical_or() {
        let results = eval_str("(return (or #f #f #t))").unwrap();
        assert_eq!(results[0], Value::Boolean(true));
    }

    #[test]
    fn test_not() {
        let results = eval_str("(return (not #t))").unwrap();
        assert_eq!(results[0], Value::Boolean(false));
    }

    #[test]
    fn test_multiple_returns() {
        let results = eval_str("(return 1) (return 2) (return 3)").unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Value::Number(1.0));
        assert_eq!(results[1], Value::Number(2.0));
        assert_eq!(results[2], Value::Number(3.0));
    }

    #[test]
    fn test_execution_continues_after_return() {
        let results = eval_str("(let x 10) (return x) (let x 20) (return x)").unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Value::Number(10.0));
        assert_eq!(results[1], Value::Number(20.0));
    }
}