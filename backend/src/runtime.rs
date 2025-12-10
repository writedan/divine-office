use crate::parser::Expr;
use crate::gabc::GabcFile;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Symbol(String),
    List(Vec<Value>),
    Function(Vec<String>, Vec<Expr>),
    Native(fn(Rc<RefCell<Runtime>>, Vec<Expr>) -> Result<Value, String>),
    Nil,

    // these are the compiler values which can be safely returned
    Error(String),
    Gabc(GabcFile),
    Title(String),
    Instruction(String),
    Box(Vec<Value>),
    Heading(String, u8)
}

impl Serialize for Value {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
		match self {
			Value::List(vals) => {
                fn flatten<'a>(v: &'a Value, out: &mut Vec<&'a Value>) {
                    match v {
                        Value::List(inner) => {
                            for x in inner {
                                flatten(x, out);
                            }
                        }
                        other => out.push(other)
                    }
                }

                let mut flat: Vec<&Value> = Vec::new();
                for v in vals { flatten(v, &mut flat); }

                let mut seq = serializer.serialize_seq(Some(flat.len()))?;
                for v in flat { seq.serialize_element(v)?; }
                seq.end()
            },
			Value::Gabc(g) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("RawGabc", &g.to_string())?;
                map.end()
			},
			Value::Error(e) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Error", &e)?;
                map.end()
			},
			Value::Title(s) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Title", &s)?;
                map.end()
			},
			Value::Instruction(s) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Instruction", &s)?;
                map.end()
			},
			Value::Box(vals) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Box", &vals)?;
                map.end()
			},
			Value::Heading(s, n) => {
				let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Heading", &(s, n))?;
                map.end()
			},
            Value::String(s) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("Text", &s)?;
                map.end()
            }
			_ => panic!("Serialization is not supported for {:?}", self)
		}

	}
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
            Value::Native(_) => "<native>".into(),
            Value::Nil => "()".into(),
            Value::Error(e) => format!("Error: {}", e),
            Value::Gabc(g) => g.to_string(),
            Value::Title(s) => s.clone(),
            Value::Instruction(s) => s.clone(),
            Value::Box(vals) => {
            	let items: Vec<_> = vals.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(" "))
            },
            Value::Heading(s, _) => s.clone()
        }
    }

    pub fn is_truthy(&self) -> bool {
    	match self {
    		Value::Boolean(b) => *b,
    		Value::List(vals) => vals.len() > 0,
    		Value::Nil => false,
    		_ => true
    	}
    }

    pub fn is_numeric(&self) -> bool {
    	match self {
    		Value::Number(_) => true,
    		_ => false
    	}
    }
}

pub struct Runtime {
    parent: Option<Rc<RefCell<Runtime>>>,
    bindings: HashMap<String, Value>,
    yields: Vec<Value>
}

impl Runtime {
    pub fn new() -> Rc<RefCell<Self>> {
        let rt = Rc::new(RefCell::new(Self {
            parent: None,
            bindings: HashMap::new(),
            yields: Vec::new()
        }));

        {
            rt.borrow_mut().define("fun".into(), Value::Native(|_, args| {
	    		if args.is_empty() {
			        return Err("fun requires (fun (<params>...) <body>...)".into());
			    }

			    let params = match &args[0] {
			        Expr::List(ps) => ps.iter()
			            .map(|p| match p {
			                Expr::Symbol(s) => Ok(s.clone()),
			                other => Err(format!("Lambda parameters must be symbols, got {:?}", other)),
			            })
			            .collect::<Result<Vec<_>, _>>()?,
			        other => return Err(format!("Lambda parameters must be a list, got {:?}", other)),
			    };

			    let body: Vec<Expr> = args[1..].to_vec();
			    Ok(Value::Function(params, body))
			}));

            rt.borrow_mut().define("let".into(), Value::Native(|env, args| {
                if args.len() != 2 {
                    return Err("let requires (let <name> <value>)".into());
                }
                let name = match &args[0] {
                    Expr::Symbol(s) => s.clone(),
                    other => return Err(format!("Variable name must be symbol, got {:?}", other)),
                };
                let val = Runtime::eval(Rc::clone(&env), &args[1])?;
                env.borrow_mut().define(name, val.clone());

                Ok(val)
            }));

            rt.borrow_mut().define("export".into(), Value::Native(|env, args| {
                for arg in args {
                    match arg {
                        Expr::Symbol(ref s) => {
                            let val = Runtime::eval(Rc::clone(&env), &arg)?;
                            Runtime::root(&env).borrow_mut().define(s.clone(), val);
                        }
                        other => return Err(format!("Arguments must be symbol, got {:?}", other)),
                    }
                }

                Ok(Value::Nil)
            }));
        }

        rt.borrow_mut().define("return".into(), Value::Native(|env, args| {
        	let mut out = Vec::new();
        	for arg in args {
        		let val = match Runtime::eval(Rc::clone(&env), &arg) {
        			Ok(val) => val,
        			Err(why) => Value::Error(format!("Error while evaluating {}: {}", arg, why))
        		};

        		match val {
        			// This code will prevent returns of lists, instead flattening out the structure entirely. It is commented out since this applies to any function return, not merely the highest-level. Rather, serialization will flatten lists at compile-time.

        			// Value::List(ref values) => {
        			// 	out.extend(values.clone());
        			// 	env.borrow_mut().yields.extend(values.clone());
        			// },

        			ref other => {
        				out.push(other.clone());
        				env.borrow_mut().yields.push(other.clone());
        			}
        		}
        	}

        	Ok(Value::List(out))
        }));

        rt.borrow_mut().define("if".into(), Value::Native(|env, args| {
        	if args.len() != 3 {
	            return Err("if requires 3 arguments: (if (cond) (then) (else))".into());
	        }

	        if Runtime::eval(Rc::clone(&env), &args[0])?.is_truthy() {
	            Runtime::eval(Rc::clone(&env), &args[1])
	        } else {
	            Runtime::eval(Rc::clone(&env), &args[2])
	        }
        }));

        rt.borrow_mut().define("val".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
	            return Err("val requires 1 argument: (val <variable>".into());
	        }


	        match Runtime::eval(Rc::clone(&env), &args[0])? {
	        	Value::Symbol(s) => {
	        		match env.borrow().get(&s) {
	        			Some(val) => Ok(val),
	        			None => Ok(Value::Nil)
	        		}
	        	},
	        	other => return Err(format!("val requires a symbol, got {:?}", other))
	        }
        }));

        rt.borrow_mut().define("is-num".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
	            return Err("is-num requires 1 argument: (is-num <value>".into());
	        }

	        Ok(Value::Boolean(Runtime::eval(Rc::clone(&env), &args[0])?.is_numeric()))
        }));

        rt.borrow_mut().define("import".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
	            return Err("import requires 1 argument: (import <file>".into());
	        }

	        let path = Runtime::eval(Rc::clone(&env), &args[0])?.to_string();
	        match crate::wasm::read_file(&path) {
	        	Ok(val) => Ok(Value::String(val)),
	        	Err(why) => Err(format!("Error reading file \"{}\": {}", path, why))
	        }
        }));

        rt.borrow_mut().define("cat".into(), Value::Native(|env, args| {
        	let mut out = String::new();
        	for arg in args {
        		out.push_str(Runtime::eval(Rc::clone(&env), &arg)?.to_string().as_str());
        	}

        	Ok(Value::String(out))
        }));

        rt.borrow_mut().define("title".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
	            return Err("title requires 1 argument.".into());
	        }

	        Ok(Value::Title(Runtime::eval(Rc::clone(&env), &args[0])?.to_string()))
        }));

        rt.borrow_mut().define("raw-gabc".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
	            return Err("raw-gabc requires 1 argument.".into());
	        }

	        Ok(Value::Gabc(GabcFile::new(&Runtime::eval(Rc::clone(&env), &args[0])?.to_string())?))
        }));

        rt.borrow_mut().define("gabc-attr".into(), Value::Native(|env, args| {
        	if args.len() != 2 {
        		return Err("gabc-attr requires two arguments: (gabc-attr <gabc> <attr>)".into());
        	}

        	let gabc = match Runtime::eval(Rc::clone(&env), &args[0])? {
        		Value::Gabc(g) => g,
        		other => return Err(format!("gabc-attr expects GABC, got {:?}", other))
        	};

        	let attr = Runtime::eval(Rc::clone(&env), &args[1])?.to_string();

        	Ok(
        		match gabc.get_header(&attr) {
        			Some(val) => Value::String(val.to_string()),
        			None => Value::Nil
        		}
        	)
        }));

        rt.borrow_mut().define("set-gabc-attr".into(), Value::Native(|env, args| {
        	if args.len() != 3 {
        		return Err("set-gabc-attr requires three arguments: (set-gabc-attr <gabc> <attr> <value>)".into());
        	}

        	let mut gabc = match Runtime::eval(Rc::clone(&env), &args[0])? {
        		Value::Gabc(g) => g,
        		other => return Err(format!("gabc-attr expects GABC, got {:?}", other.to_string()))
        	};

        	let attr = Runtime::eval(Rc::clone(&env), &args[1])?.to_string();
        	let value = Runtime::eval(Rc::clone(&env), &args[2])?.to_string();

        	gabc.set_header(attr, value);
        	Ok(Value::Gabc(gabc))
        }));

        rt.borrow_mut().define("not".into(), Value::Native(|env, args| {
        	if args.len() != 1 {
        		return Err("not requires one argument".into());
        	}

        	Ok(Value::Boolean(!Runtime::eval(Rc::clone(&env), &args[0])?.is_truthy()))
        }));

        rt.borrow_mut().define("load".into(), Value::Native(|env, args| {
            use crate::wasm;

            if args.len() != 1 {
                return Err("load requires one argument".into());
            }

            let file = Runtime::eval(Rc::clone(&env), &args[0])?.to_string();
            let file = wasm::read_file(file)?;
            let exprs = wasm::get_exprs(file)?;
            let new_env = Runtime::with_parent(Rc::clone(&env));
            Ok(Value::List(Runtime::run(Rc::clone(&new_env), exprs)))
        }));

        rt.borrow_mut().define(">=".into(), Value::Native(|env, args| {
            if args.len() != 2 {
                return Err(">= requires 2 arguments: (>= <a> <b>)".into());
            }
            
            let a = Runtime::eval(Rc::clone(&env), &args[0])?;
            let b = Runtime::eval(Rc::clone(&env), &args[1])?;
            
            match (a, b) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x >= y)),
                _ => Err(">= requires numeric arguments".into())
            }
        }));

        rt.borrow_mut().define("as-num".into(), Value::Native(|env, args| {
            if args.len() != 1 {
                return Err("as-num requires 1 argument: (as-num <value>)".into());
            }
            
            let val = Runtime::eval(Rc::clone(&env), &args[0])?;
            
            match val {
                Value::Number(n) => Ok(Value::Number(n)),
                Value::String(s) => {
                    s.parse::<f64>()
                        .map(Value::Number)
                        .map_err(|_| format!("Cannot convert '{}' to number", s))
                }
                other => Err(format!("Cannot convert {:?} to number", other))
            }
        }));

        rt.borrow_mut().define("match".into(), Value::Native(|env, args| {
            if args.len() < 2 {
                return Err("match requires at least 2 arguments: (match <value> <case>...)".into());
            }
            
            let match_val = Runtime::eval(Rc::clone(&env), &args[0])?;
            
            for case_expr in &args[1..] {
                match case_expr {
                    Expr::List(case_parts) if !case_parts.is_empty() => {
                        if let Expr::Symbol(s) = &case_parts[0] {
                            if s != "case" {
                                return Err(format!("Expected 'case', got '{}'", s));
                            }
                        } else {
                            return Err("Match body must contain case expressions".into());
                        }
                        
                        if case_parts.len() < 2 {
                            return Err("case requires at least a pattern: (case <pattern> <body>...)".into());
                        }
                        
                        let pattern = &case_parts[1];
                        let matched = match pattern {
                            Expr::Quote(inner) => {
                                if let Expr::Symbol(sym) = &**inner {
                                    // bind the symbol to match_val in the current environment
                                    env.borrow_mut().define(sym.clone(), match_val.clone());
                                    
                                    let mut result = Value::Nil;
                                    for expr in &case_parts[2..] {
                                        result = Runtime::eval(Rc::clone(&env), expr)?;
                                    }

                                    return Ok(result);
                                } else {
                                    return Err("Quoted pattern must be a symbol".into());
                                }
                            },

                            Expr::String(s) => {
                                if let Value::String(ref val_str) = match_val {
                                    val_str == s
                                } else {
                                    false
                                }
                            },

                            Expr::List(patterns) => {
                                let mut any_match = false;
                                for pat in patterns {
                                    if let Expr::String(s) = pat {
                                        if let Value::String(ref val_str) = match_val {
                                            if val_str == s {
                                                any_match = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                any_match
                            }
                            other => return Err(format!("Invalid case pattern: {:?}", other))
                        };
                        
                        if matched {
                            let mut result = Value::Nil;
                            for expr in &case_parts[2..] {
                                result = Runtime::eval(Rc::clone(&env), expr)?;
                            }
                            return Ok(result);
                        }
                    }
                    other => return Err(format!("Match body must be case expressions, got {:?}", other))
                }
            }
            
            Ok(Value::Nil)
        }));

        rt.borrow_mut().define("gabc".into(), Value::Native(|env, args| {
            let mut out = Vec::new();
            for arg in args {
                let val = Runtime::eval(Rc::clone(&env), &arg)?.to_string();
                out.push(Value::Gabc(GabcFile::new(format!("initial-style: 0;\ncentering-scheme: english;\n%%\n{}", val).as_str())?));
            }

            Ok(Value::List(out)) 
        }));

        rt.borrow_mut().define("instruction".into(), Value::Native(|env, args| {
            let mut out = Vec::new();
            for arg in args {
                let val = Runtime::eval(Rc::clone(&env), &arg)?.to_string();
                out.push(Value::Instruction(val));
            }

            Ok(Value::List(out))
        }));

        rt.borrow_mut().define("heading".into(), Value::Native(|env, args| {
            if args.len() < 1 || args.len() > 2 {
                return Err("heading requires at least one argument: (heading <text> [size])".into());
            }

            let text = Runtime::eval(Rc::clone(&env), &args[0])?.to_string();
            let level = if args.len() == 1 { 1.0 } else { 
                match Runtime::eval(Rc::clone(&env), &args[1])? {
                    Value::Number(n) => n,
                    other => return Err(format!("Second argunebt to heading must be numeric, got {:?}", other))
                }
            };

            Ok(Value::Heading(text, level as u8))
        }));

        rt
    }

    pub fn with_parent(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            bindings: HashMap::new(),
            yields: Vec::new()
        }))
    }

    pub fn root(env: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        match &env.borrow().parent {
            Some(parent) => Runtime::root(parent),
            None => Rc::clone(env),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.bindings.get(name) {
            return Some(v.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        None
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn eval(env: Rc<RefCell<Self>>, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Symbol(s) => env.borrow().get(s).ok_or_else(|| format!("Undefined symbol: {}", s)),
            Expr::Quote(inner) => env.borrow().quote_to_value(inner),
            Expr::List(list) => Self::eval_list(env, list),
            Expr::Nil => Ok(Value::Nil),
            _ => Err(format!("Cannot evaluate expression {}", expr)),
        }
    }

    fn eval_list(env: Rc<RefCell<Self>>, list: &[Expr]) -> Result<Value, String> {
        if list.is_empty() {
            return Ok(Value::Nil);
        }

        let func = Self::eval(Rc::clone(&env), &list[0])?;
        let args: Vec<Expr> = list[1..].to_vec();

        match func {
            Value::Native(f) => f(Rc::clone(&env), args),
            Value::Function(params, body) => Self::eval_function(env, list, params, body),
            other => {
                let mut evaluated = vec![other];
                for expr in &args {
                    evaluated.push(Self::eval(Rc::clone(&env), expr)?);
                }
                Ok(Value::List(evaluated))
            }
        }
    }

    fn eval_function(env: Rc<RefCell<Self>>, list: &[Expr], params: Vec<String>, body: Vec<Expr>) -> Result<Value, String> {
    	if params.len() != list.len() - 1 {
            return Err(format!(
                "Function expects {} args, got {}",
                params.len(),
                list.len() - 1
            ));
        }

    	let new_env = Runtime::with_parent(Rc::clone(&env));
    	{
    		let mut env_mut = new_env.borrow_mut();
        	for (param, arg) in params.iter().zip(list[1..].iter()) {
                env_mut.define(param.clone(), Runtime::eval(Rc::clone(&new_env), arg)?);
        	}
    	}

    	for expr in &body {
            Runtime::eval(Rc::clone(&new_env), expr)?;
        }

        let yields = new_env.borrow().yields.clone();
    	Ok(Value::List(yields))
    }

    fn quote_to_value(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Symbol(s) => Ok(Value::Symbol(s.clone())),
            Expr::Nil => Ok(Value::Nil),
            Expr::List(xs) => {
                let vals: Result<Vec<_>, _> = xs.iter().map(|x| self.quote_to_value(x)).collect();
                Ok(Value::List(vals?))
            }
            _ => Err(format!("Cannot quote this expression: {}", expr)),
        }
    }

    pub fn run(env: Rc<RefCell<Self>>, exprs: Vec<Expr>) -> Vec<Value> {
        for expr in exprs {
            match Self::eval(Rc::clone(&env), &expr) {
                Ok(_) => {},
                Err(why) => env.borrow_mut().yields.push(Value::Error(format!("Error evaluating {}: {}", expr, why)))
            };
        }

        env.borrow().yields.clone()
    }
}