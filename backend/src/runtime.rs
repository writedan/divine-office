use crate::wasm::read_file;
use crate::lexer::Lexer;
use crate::parser::{Parser, Expr};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
    List(Vec<Value>),
    Function(Vec<String>, Vec<Expr>),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
	Group(Vec<Element>),
    Box(Vec<Element>),
    Text(String),
    Heading(String, u8),
    Instruction(String),
    RawGabc(String),
    Title(String),
    Error(String),
    Empty,
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
            Value::Function(_, _) => "<function>".to_string(),
            Value::Nil => "()".to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(false) | Value::Nil => false,
            _ => true,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    bindings: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.bindings.get(name).or_else(|| {
            self.parent.as_ref().and_then(|p| p.get(name))
        })
    }
}

pub struct Runtime {
    env: Environment,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            env: Environment::new(),
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
                    	"import" => self.eval_import(&elements[1..]),
                        "let" => self.eval_let(&elements[1..]),
                        "defun" => self.eval_defun(&elements[1..]),
                        "eq" => self.eval_eq(&elements[1..]),
                        "if" => self.eval_if(&elements[1..]),
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

    pub fn eval_to_element(&mut self, expr: &Expr) -> Element {
        match expr {
            Expr::List(elements) if !elements.is_empty() => {
                if let Some(Expr::Symbol(op)) = elements.first() {
                    match op.as_str() {
                    	"load" => self.eval_load(&elements[1..]),
                        "box" => self.eval_box(&elements[1..]),
                        "text" => self.eval_text(&elements[1..]),
                        "heading" => self.eval_heading(&elements[1..]),
                        "instruction" => self.eval_instruction(&elements[1..]),
                        "raw-gabc" => self.eval_raw_gabc(&elements[1..]),
                        "title" => self.eval_title(&elements[1..]),
                        "if" => self.eval_if_element(&elements[1..]),
                        "let" | "defun" | "import" => {
                            match self.eval(expr) {
                                Ok(_) => Element::Empty,
                                Err(e) => Element::Error(e),
                            }
                        }
                        _ => {
                            match self.eval(expr) {
                                Ok(_) => Element::Empty,
                                Err(e) => Element::Error(e),
                            }
                        }
                    }
                } else {
                    Element::Error("List must start with a symbol".to_string())
                }
            }
            _ => {
                match self.eval(expr) {
                    Ok(_) => Element::Empty,
                    Err(e) => Element::Error(e),
                }
            }
        }
    }

    fn eval_import(&mut self, args: &[Expr]) -> Result<Value, String> {
    	if args.len() != 1 {
    		return Err("import requires exactly 1 argument: (import <file>)".to_string());
    	}

    	let path = self.eval(&args[0])?.to_string();
    	Ok(Value::String(read_file(path)?))
    }

    fn eval_load(&mut self, args: &[Expr]) -> Element {
    	if args.len() != 1 {
    		return Element::Error("load requires exactly 1 argument: (load <file>)".to_string());
    	}

    	let path = match self.eval(&args[0]) {
    		Ok(val) => val.to_string(),
    		Err(e) => return Element::Error(e)
    	};

    	let mut lexer = match Lexer::from_file(path) {
    		Ok(lexer) => lexer,
    		Err(e) => return Element::Error(e)
    	};

    	let tokens = match lexer.tokenize() {
    		Ok(tokens) => tokens,
    		Err(e) => return Element::Error(e)
    	};

    	let mut parser = Parser::new(tokens);
    	let exprs = match parser.parse() {
    		Ok(exprs) => exprs,
    		Err(e) => return Element::Error(e)
    	};

    	let mut children = Vec::new();
    	for expr in exprs {
    		let element = self.eval_to_element(&expr);
    		children.push(element);
    	}

    	Element::Group(children)
    }

    fn eval_box(&mut self, args: &[Expr]) -> Element {
        let mut children = Vec::new();
        
        for arg in args {
            let element = self.eval_to_element(arg);
            children.push(element);
        }
        
        Element::Box(children)
    }

    fn eval_text(&mut self, args: &[Expr]) -> Element {
        if args.is_empty() {
            return Element::Error("text requires at least 1 argument".to_string());
        }

        let mut text_parts = Vec::new();
        
        for arg in args {
            match self.eval(arg) {
                Ok(val) => text_parts.push(val.to_string()),
                Err(e) => return Element::Error(e),
            }
        }
        
        Element::Text(text_parts.join(" "))
    }

    fn eval_heading(&mut self, args: &[Expr]) -> Element {
        if args.len() != 2 {
            return Element::Error("heading requires exactly 2 arguments: (heading <text> <level>)".to_string());
        }

        let text = match self.eval(&args[0]) {
            Ok(val) => val.to_string(),
            Err(e) => return Element::Error(e),
        };

        let level = match self.eval(&args[1]) {
            Ok(Value::Number(n)) => {
                if n >= 1.0 && n <= 6.0 {
                    n as u8
                } else {
                    return Element::Error("heading level must be between 1 and 6".to_string());
                }
            }
            Ok(_) => return Element::Error("heading level must be a number".to_string()),
            Err(e) => return Element::Error(e),
        };

        Element::Heading(text, level)
    }

    fn eval_instruction(&mut self, args: &[Expr]) -> Element {
        if args.is_empty() {
            return Element::Error("instruction requires at least 1 argument".to_string());
        }

        let mut instruction_parts = Vec::new();
        
        for arg in args {
            match self.eval(arg) {
                Ok(val) => instruction_parts.push(val.to_string()),
                Err(e) => return Element::Error(e),
            }
        }
        
        Element::Instruction(instruction_parts.join(" "))
    }

    fn eval_raw_gabc(&mut self, args: &[Expr]) -> Element {
        if args.is_empty() {
            return Element::Error("raw-gabc requires at least 1 argument".to_string());
        }

        let mut gabc_parts = Vec::new();
        
        for arg in args {
            match self.eval(arg) {
                Ok(val) => gabc_parts.push(val.to_string()),
                Err(e) => return Element::Error(e),
            }
        }
        
        Element::RawGabc(gabc_parts.join(" "))
    }

    fn eval_title(&mut self, args: &[Expr]) -> Element {
        if args.is_empty() {
            return Element::Error("title requires at least 1 argument".to_string());
        }

        let mut title_parts = Vec::new();
        
        for arg in args {
            match self.eval(arg) {
                Ok(val) => title_parts.push(val.to_string()),
                Err(e) => return Element::Error(e),
            }
        }
        
        Element::Title(title_parts.join(" "))
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

    fn eval_defun(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("defun requires exactly 2 arguments: (defun (<name> <params>...) <body>...)".to_string());
        }

        let (name, params) = match &args[0] {
            Expr::List(sig) if !sig.is_empty() => {
                let name = match &sig[0] {
                    Expr::Symbol(s) => s.clone(),
                    _ => return Err("Function name must be a symbol".to_string()),
                };
                
                let mut param_names = Vec::new();
                for param in &sig[1..] {
                    match param {
                        Expr::Symbol(s) => param_names.push(s.clone()),
                        _ => return Err("Function parameters must be symbols".to_string()),
                    }
                }
                
                (name, param_names)
            }
            _ => return Err("First argument to defun must be a list (name params...)".to_string()),
        };

        let body = match &args[1] {
            expr => vec![expr.clone()],
        };

        let function = Value::Function(params, body);
        self.env.define(name.clone(), function.clone());
        Ok(function)
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

    fn eval_if_element(&mut self, args: &[Expr]) -> Element {
        if args.len() != 3 {
            return Element::Error("if requires exactly 3 arguments: (if <cond> <then> <else>)".to_string());
        }

        let condition = match self.eval(&args[0]) {
            Ok(val) => val,
            Err(e) => return Element::Error(e),
        };
        
        if condition.is_truthy() {
            self.eval_to_element(&args[1])
        } else {
            self.eval_to_element(&args[2])
        }
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
        let func_val = self.eval(&elements[0])?;
        
        match func_val {
            Value::Function(params, body) => {
                let mut args = Vec::new();
                for arg_expr in &elements[1..] {
                    args.push(self.eval(arg_expr)?);
                }
                
                if args.len() != params.len() {
                    return Err(format!(
                        "Function expects {} arguments, got {}",
                        params.len(),
                        args.len()
                    ));
                }
                
                let mut new_env = Environment::with_parent(self.env.clone());
                for (param, arg) in params.iter().zip(args.iter()) {
                    new_env.define(param.clone(), arg.clone());
                }
                
                let old_env = std::mem::replace(&mut self.env, new_env);
                
                let mut result = Value::Nil;
                for expr in &body {
                    result = self.eval(expr)?;
                }
                
                self.env = old_env;
                Ok(result)
            }
            _ => Err(format!("Cannot apply non-function: {:?}", elements[0])),
        }
    }

    pub fn run(&mut self, exprs: Vec<Expr>) -> Vec<Element> {
        let mut elements = Vec::new();
        
        for expr in exprs {
            let element = self.eval_to_element(&expr);
            self.flatten_element(element, &mut elements);
        }
        
        elements
    }

    fn flatten_element(&self, element: Element, output: &mut Vec<Element>) {
        match element {
            Element::Group(children) => {
                for child in children {
                    self.flatten_element(child, output);
                }
            }
            other => output.push(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse_str(input: &str) -> Vec<Expr> {
    	let mut lexer = Lexer::from_str(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    fn eval_str(input: &str) -> Vec<Element> {
        let mut lexer = Lexer::from_str(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        let mut runtime = Runtime::new();
        runtime.run(exprs)
    }

    #[test]
    fn test_text() {
        let elements = eval_str(r#"(text "Hello World")"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Text(s) => assert_eq!(s, "Hello World"),
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_text_with_expression() {
        let elements = eval_str(r#"(let x 42) (text "The answer is" x)"#);
        assert_eq!(elements.len(), 2);
        match &elements[1] {
            Element::Text(s) => assert_eq!(s, "The answer is 42"),
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_heading() {
        let elements = eval_str(r#"(heading "Chapter 1" 1)"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Heading(text, level) => {
                assert_eq!(text, "Chapter 1");
                assert_eq!(*level, 1);
            }
            _ => panic!("Expected Heading element"),
        }
    }

    #[test]
    fn test_box() {
        let elements = eval_str(r#"(box (text "First") (text "Second"))"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Box(children) => {
                assert_eq!(children.len(), 2);
                match &children[0] {
                    Element::Text(s) => assert_eq!(s, "First"),
                    _ => panic!("Expected Text element"),
                }
                match &children[1] {
                    Element::Text(s) => assert_eq!(s, "Second"),
                    _ => panic!("Expected Text element"),
                }
            }
            _ => panic!("Expected Box element"),
        }
    }

    #[test]
    fn test_nested_box() {
        let elements = eval_str(r#"(box (box (text "Nested")) (text "Top"))"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Box(children) => {
                assert_eq!(children.len(), 2);
                match &children[0] {
                    Element::Box(nested) => {
                        assert_eq!(nested.len(), 1);
                        match &nested[0] {
                            Element::Text(s) => assert_eq!(s, "Nested"),
                            _ => panic!("Expected Text element"),
                        }
                    }
                    _ => panic!("Expected Box element"),
                }
            }
            _ => panic!("Expected Box element"),
        }
    }

    #[test]
    fn test_instruction() {
        let elements = eval_str(r#"(instruction "Stand for the Gospel")"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Instruction(s) => assert_eq!(s, "Stand for the Gospel"),
            _ => panic!("Expected Instruction element"),
        }
    }

    #[test]
    fn test_title() {
        let elements = eval_str(r#"(title "Mass of the Holy Spirit")"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Title(s) => assert_eq!(s, "Mass of the Holy Spirit"),
            _ => panic!("Expected Title element"),
        }
    }

    #[test]
    fn test_raw_gabc() {
        let elements = eval_str(r#"(raw-gabc "c4 (g) Al(g)le(f)lu(gh)ia")"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::RawGabc(s) => assert_eq!(s, "c4 (g) Al(g)le(f)lu(gh)ia"),
            _ => panic!("Expected RawGabc element"),
        }
    }

    #[test]
    fn test_conditional_element() {
        let elements = eval_str(r#"(let x 1) (if (eq x 1) (text "One") (text "Not One"))"#);
        assert_eq!(elements.len(), 2);
        match &elements[1] {
            Element::Text(s) => assert_eq!(s, "One"),
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_error_handling() {
        let elements = eval_str(r#"(heading "Test")"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Error(_) => {},
            _ => panic!("Expected Error element"),
        }
    }

    #[test]
    fn test_mixed_statements() {
        let elements = eval_str(r#"
            (let title-text "My Document")
            (title title-text)
            (heading "Section 1" 1)
            (text "This is some text")
            (box (text "Boxed text"))
        "#);
        
        match &elements[0] {
            Element::Empty => {},
            _ => panic!("Expected Empty element for let statement"),
        }
        
        match &elements[1] {
            Element::Title(s) => assert_eq!(s, "My Document"),
            _ => panic!("Expected Title element"),
        }
        
        match &elements[2] {
            Element::Heading(text, level) => {
                assert_eq!(text, "Section 1");
                assert_eq!(*level, 1);
            }
            _ => panic!("Expected Heading element"),
        }
    }

    #[test]
    fn test_computation_in_text() {
        let elements = eval_str(r#"(text "2 + 2 =" (+ 2 2))"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Text(s) => assert_eq!(s, "2 + 2 = 4"),
            _ => panic!("Expected Text element"),
        }
    }

    #[test]
    fn test_undefined_symbol_error() {
        let elements = eval_str(r#"(text undefined-var)"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Error(e) => assert!(e.contains("Undefined symbol")),
            _ => panic!("Expected Error element"),
        }
    }

    #[test]
    fn test_error_in_box_child() {
        let elements = eval_str(r#"(box (text "Good") (text undefined-var) (text "Also Good"))"#);
        assert_eq!(elements.len(), 1);
        match &elements[0] {
            Element::Box(children) => {
                assert_eq!(children.len(), 3);
                match &children[0] {
                    Element::Text(s) => assert_eq!(s, "Good"),
                    _ => panic!("Expected Text element"),
                }
                match &children[1] {
                    Element::Error(e) => assert!(e.contains("Undefined symbol")),
                    _ => panic!("Expected Error element"),
                }
                match &children[2] {
                    Element::Text(s) => assert_eq!(s, "Also Good"),
                    _ => panic!("Expected Text element"),
                }
            }
            _ => panic!("Expected Box element"),
        }
    }

    #[test]
    fn test_defined_function() {
    	let mut runtime = Runtime::new();
    	runtime.run(parse_str("(defun (square x) (* x x))"));
    	let out = runtime.run(parse_str("(text (square 2))"));
    	assert_eq!(out, vec![Element::Text(String::from("4"))]);
    }
}