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
    Error(String)
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(true) => "#t".into(),
            Value::Boolean(false) => "#f".into(),
            Value::Symbol(s) => s.clone(),
            Value::List(vals) => {
                let items: Vec<_> = vals.iter().map(|v| v.to_string()).collect();
                format!("({})", items.join(" "))
            }
            Value::Function(_, _) => "<function>".into(),
            Value::Nil => "()".into(),
            Value::Error(e) => format!("Error: {}", e)
        }
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Boolean(false) | Value::Nil)
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
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.bindings
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }
}

pub struct Runtime {
    env: Environment,
    pub output: Vec<Value>, 
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            output: Vec::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Nil => Ok(Value::Nil),

            Expr::Symbol(s) => self
                .env
                .get(s)
                .cloned()
                .ok_or_else(|| format!("Undefined symbol: {}", s)),

            Expr::Quote(inner) => self.quote_to_value(inner),

            Expr::List(list) => self.eval_list(list),

            _ => Err(format!("Cannot evaluate expression: {:?}", expr)),
        }
    }

    fn eval_list(&mut self, list: &[Expr]) -> Result<Value, String> {
        if list.is_empty() {
            return Ok(Value::Nil);
        }

        if let Expr::Symbol(op) = &list[0] {
            return match op.as_str() {
            	"return" => self.eval_return(&list[1..]),
                "import" => self.eval_import(&list[1..]),

                "let" => self.eval_let(&list[1..]),
                "defun" => self.eval_defun(&list[1..]),

                "eq" => self.eval_eq(&list[1..]),
                "if" => self.eval_if(&list[1..]),
                "quote" => self.eval_quote(&list[1..]),

                "+" => self.eval_arith(&list[1..], |a, b| a + b),
                "-" => self.eval_arith(&list[1..], |a, b| a - b),
                "*" => self.eval_arith(&list[1..], |a, b| a * b),
                "/" => self.eval_arith(&list[1..], |a, b| a / b),

                "<" => self.eval_cmp(&list[1..], |a, b| a < b),
                ">" => self.eval_cmp(&list[1..], |a, b| a > b),
                "<=" => self.eval_cmp(&list[1..], |a, b| a <= b),
                ">=" => self.eval_cmp(&list[1..], |a, b| a >= b),

                "and" => self.eval_and(&list[1..]),
                "or" => self.eval_or(&list[1..]),
                "not" => self.eval_not(&list[1..]),

                "cat" => self.eval_cat(&list[1..]),

                "raw-gabc" => self.eval_raw_gabc(&list[1..]),

                _ => self.eval_application(list),
            };
        }

        self.eval_application(list)
    }

    fn eval_return(&mut self, args: &[Expr]) -> Result<Value, String> {
	    let mut values = Vec::new();
	    for expr in args {
	        let val = self.eval(expr)?;
	        values.push(val.clone());
	        self.output.push(val); 
	    }

	    Ok(Value::List(values))
	}


    fn eval_cat(&mut self, args: &[Expr]) -> Result<Value, String> {
    	let mut out = String::new();

    	for arg in args {
    		let val = self.eval(arg)?;
    		out.push_str(&val.to_string());
    	}

    	Ok(Value::String(out))
    }

    fn eval_raw_gabc(&mut self, args: &[Expr]) -> Result<Value, String> {
    	if args.len() != 1 {
    		return Err("raw-gabc requires exactly one argument".into());
    	}

    	let path = self.eval(&args[0])?.to_string();
    	Ok(Value::String(path))
    }

    fn eval_import(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("import requires exactly one argument".into());
        }
        let path = self.eval(&args[0])?.to_string();
        Ok(Value::String(read_file(path)?))
    }

    fn eval_let(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("let requires (let <symbol> <value>)".into());
        }

        let name = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err("let name must be a symbol".into()),
        };

        let val = self.eval(&args[1])?;
        self.env.define(name.clone(), val.clone());
        Ok(val)
    }

    fn eval_defun(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() < 2 {
            return Err("defun requires (defun (<name> <params>...) <body>...)" .into());
        }

        // parse signature (name + params)
        let (fname, params) = match &args[0] {
            Expr::List(sig) if !sig.is_empty() => {
                let name = match &sig[0] {
                    Expr::Symbol(s) => s.clone(),
                    _ => return Err("Function name must be symbol".into()),
                };

                let mut ps = Vec::new();
                for p in &sig[1..] {
                    match p {
                        Expr::Symbol(s) => ps.push(s.clone()),
                        _ => return Err("Function parameters must be symbols".into()),
                    }
                }
                (name, ps)
            }
            _ => return Err("defun signature must be (name params...)".into()),
        };

        // allow multiple body expressions directly:
        let body: Vec<Expr> = args[1..].iter().cloned().collect();

        let fun = Value::Function(params, body);

        self.env.define(fname.clone(), fun.clone());
        Ok(fun)
    }

    fn eval_eq(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("eq requires 2 arguments".into());
        }
        let a = self.eval(&args[0])?;
        let b = self.eval(&args[1])?;
        Ok(Value::Boolean(a == b))
    }

    fn eval_if(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("if requires 3 arguments".into());
        }
        if self.eval(&args[0])?.is_truthy() {
            self.eval(&args[1])
        } else {
            self.eval(&args[2])
        }
    }

    fn eval_quote(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("quote requires 1 argument".into());
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

            Expr::List(xs) => {
                let mut vals = Vec::new();
                for x in xs {
                    vals.push(self.quote_to_value(x)?);
                }
                Ok(Value::List(vals))
            }

            _ => Err("Cannot quote this expression type".into()),
        }
    }

    fn eval_arith<F>(&mut self, args: &[Expr], op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        if args.is_empty() {
            return Err("Arithmetic requires at least 1 argument".into());
        }

        let mut result = match self.eval(&args[0])? {
            Value::Number(n) => n,
            _ => return Err("Arithmetic operands must be numbers".into()),
        };

        for a in &args[1..] {
            match self.eval(a)? {
                Value::Number(n) => result = op(result, n),
                _ => return Err("Arithmetic operands must be numbers".into()),
            }
        }

        Ok(Value::Number(result))
    }

    fn eval_cmp<F>(&mut self, args: &[Expr], op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        if args.len() != 2 {
            return Err("Comparison requires 2 arguments".into());
        }

        let a = self.eval(&args[0])?;
        let b = self.eval(&args[1])?;

        match (a, b) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(op(x, y))),
            _ => Err("Comparison requires numbers".into()),
        }
    }

    fn eval_and(&mut self, args: &[Expr]) -> Result<Value, String> {
        for a in args {
            if !self.eval(a)?.is_truthy() {
                return Ok(Value::Boolean(false));
            }
        }
        Ok(Value::Boolean(true))
    }

    fn eval_or(&mut self, args: &[Expr]) -> Result<Value, String> {
        for a in args {
            if self.eval(a)?.is_truthy() {
                return Ok(Value::Boolean(true));
            }
        }
        Ok(Value::Boolean(false))
    }

    fn eval_not(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("not requires 1 argument".into());
        }
        Ok(Value::Boolean(!self.eval(&args[0])?.is_truthy()))
    }

    fn eval_application(&mut self, list: &[Expr]) -> Result<Value, String> {
        let func = self.eval(&list[0])?;

        match func {
            Value::Function(params, body) => {
                if params.len() != list.len() - 1 {
                    return Err(format!(
                        "Function expects {} args, got {}",
                        params.len(),
                        list.len() - 1
                    ));
                }

                let mut new_env = Environment::with_parent(self.env.clone());
                for (param, arg) in params.iter().zip(list[1..].iter()) {
                    new_env.define(param.clone(), self.eval(arg)?);
                }

                let old_env = std::mem::replace(&mut self.env, new_env);

                let mut result = Value::Nil;
                for expr in &body {
                    result = self.eval(expr)?;
                }

                self.env = old_env;
                Ok(result)
            }

            _ => Err(format!("Attempted to call a non-function: {:?}", func)),
        }
    }
     /// Evaluates a sequence of expressions. If an error occurs, a Value::Error is pushed for that expression.
    pub fn run(&mut self, exprs: Vec<Expr>) -> Vec<Value> {
	    let mut results = Vec::new();

	    for expr in exprs {
	        match self.eval(&expr) {
	            Ok(v) => {
	                if let Value::List(vals) = &v {
	                    for val in vals {
	                        results.push(val.clone());
	                    }
	                }
	            }
	            Err(e) => {
	                results.push(Value::Error(e));
	            }
	        }
	    }

	    results
	}

}
