use crate::parser::Expr;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
enum TempExpr {
	One(Result<Expr>),
	Many(Vec<Result<Expr>>)
}

pub struct Preprocessor {
	env: HashMap<&'static str, TempExpr>
}

impl Preprocessor {
	pub fn new() -> Self {
		Preprocessor {
			env: HashMap::from([
				("test-var", TempExpr::One(Ok(Expr::String(String::from("This is a test variable.")))))
			])
		}
	}

	fn resolve_symbol(&self, sym: &str) -> TempExpr {
		match self.env.get(&sym) {
			Some(val) => val.clone(),
			None => TempExpr::One(Ok(Expr::Symbol(sym.to_string())))
		}
	}

	fn preprocess_expr(&self, expr: Expr) -> TempExpr {
	    match expr {
	        Expr::Symbol(sym) => self.resolve_symbol(&sym),

	        Expr::List(vals) => {
	            let mut processed_items = Vec::new();

	            for item in vals {
	                match self.preprocess_expr(item) {
	                    TempExpr::One(res) => processed_items.push(res),
	                    TempExpr::Many(many) => processed_items.extend(many),
	                }
	            }

	            if let Some(Err(e)) = processed_items.iter().find(|r| r.is_err()) {
	                return TempExpr::One(Err(e.clone()));
	            }

	            let exprs: Result<Vec<Expr>> = processed_items.into_iter().collect();

	            match exprs {
	                Ok(exprs) => TempExpr::One(Ok(Expr::List(exprs))),
	                Err(e) => TempExpr::One(Err(e)),
	            }
	        }

	        _ => TempExpr::One(Ok(expr)),
	    }
	}

	pub fn preprocess(&self, exprs: Vec<Expr>) -> Vec<Result<Expr>> {
		let mut out = Vec::new();

		for e in exprs {
			match self.preprocess_expr(e) {
				TempExpr::One(expr) => out.push(expr),
				TempExpr::Many(exprs) => out.extend(exprs)
			};
		}

		out
	}
}