mod ast;

use crate::liturgy::Liturgy;
use crate::kalendar::Identifier;
use crate::kalendar::Kalendar;

use std::path::PathBuf;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;

use regex::Regex;

use lazy_static::lazy_static;

use crate::parser::ast::*;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r#"^#([\w-]+)(?:\s+"((?:[^"\\]|\\.)*)"(?:\s+"((?:[^"\\]|\\.)*)")?(?:\s+"((?:[^"\\]|\\.)*)")?)?"#
    ).unwrap();
}

#[derive(Debug, Clone)]
pub enum Directive {
	Text(String),
	Heading(String),
	Instruction(String),
	Gabc(String, bool, String), // whether english (true) or latin (false) -- default to english
	RawGabc(String),
	Import(PathBuf),
	Title(String),
	Error(String)
}

struct Parser {
	propers: HashMap<&'static str, PathBuf>
}

struct Preprocessor {
	parser: Parser,
	tree: ASTree<Directive>
}

impl Preprocessor {
	fn preprocess(&mut self) -> ASTree<Directive> {
		self.parser.propers.remove("internal:preprocess");
		let mut base = ASTree::<Directive>::new();
		for node in self.tree.children() {
			base.add_node(match node {
				ASTNode::Node(node) => self.preprocess_directive(node),
				ASTNode::Tree(tree) => self.preprocess_tree(tree)
			});
		}

		if self.parser.propers.contains_key("internal:preprocess") {
			self.tree = base;
			return self.preprocess();
		}

		base
	}

	fn preprocess_directive(&mut self, dir: Directive) -> ASTNode<Directive> {
		match dir {
			Directive::Gabc(music, is_english, style) => {
				let lang = if is_english { "english" } else { "latin" };
				ASTNode::Node(Directive::RawGabc(format!("initial-style: {};\ncentering-scheme: {};\n%%\n{}", style, lang, music)))
			},

			Directive::Import(path) => match self.parser.parse_file(path) {
				Ok(tree) => ASTNode::Tree(tree),
				Err(why) => ASTNode::Node(Directive::Error(why))
			},
			_ => ASTNode::Node(dir)
		}
	}

	fn preprocess_tree(&self, tree: ASTree<Directive>) -> ASTNode<Directive> {
		todo!()
	}
}

impl Parser {

	fn parse_line(&mut self, line: String) -> Result<Directive, String> {
		self.propers.insert("internal:preprocess", "true".into());
		if let Some(captures) = RE.captures(&line) {
			let command = captures.get(1).map_or("", |m| m.as_str());
			let arg1 = captures.get(2).map_or("", |m| m.as_str()).to_string();
			let arg2 = captures.get(3).map_or("", |m| m.as_str()).to_string();
			let arg3 = captures.get(4).map_or("", |m| m.as_str()).to_string();

			match command {
				"antiphon" => {
					let mut antiphon_path: PathBuf = ["antiphon", &arg1].iter().collect();
					antiphon_path.set_extension("gabc");
					self.propers.insert("internal:previous-antiphon", antiphon_path.clone());
					Ok(Directive::Import(antiphon_path))
				},
				"repeat-antiphon" => {
					match self.propers.get("internal:previous-antiphon") {
						Some(path) => Ok(Directive::Import(path.to_path_buf())),
						None => Err(format!("No antiphon was previously declared"))
					}
				},
				"repeat-tone" => {
					match self.propers.get("internal:previous-tone") {
						Some(path) => Ok(Directive::Import(path.to_path_buf())),
						None => Err(format!("No tone was previously declared"))
					}
				},
				"tone" => {
					let mut tone_path: PathBuf = ["tone", &arg1].iter().collect();
					tone_path.set_extension("gabc");
					self.propers.insert("internal:previous-tone", tone_path.clone());
					Ok(Directive::Import(tone_path))
				},
				"psalm" => {
					let mut psalm_path: PathBuf = ["psalter", &arg1, "text.lit"].iter().collect();
					Ok(Directive::Import(psalm_path))
				},
				"text" => Ok(Directive::Text(arg1)),
				"heading" => Ok(Directive::Heading(arg1)),
				"instruction" => Ok(Directive::Instruction(arg1)),
				"gabc" => Ok(Directive::Gabc(arg1, arg2 == "english" || arg2 == "", if arg3 == "" { "0".to_string() } else { arg3 })),
				"include" => Ok(Directive::Import(self.resolve_field(arg1)?)),
				"import" => Ok(Directive::Import(arg1.into())),
				"title" => Ok(Directive::Title(arg1)),
				_ => Err(format!("Unknown command \"{}\"", command))
			}
		} else {
			Err(format!("Malformed command: {}", line))
		}
	}

	fn resolve_field(&self, field: String) -> Result<PathBuf, String> {
		match self.propers.get(field.as_str()) {
			Some(path) => Ok(path.to_path_buf()),
			None => Err(format!("Field \"{}\" was not set.", field))
		}
	}

	fn parse_file(&mut self, path: PathBuf) -> Result<ASTree<Directive>, String> {
		let path = path.as_path();
		let mut file = match File::open(&path) {
			Err(why) => return Err(format!("Failed to open \"{}\": {}", path.display(), why)),
			Ok(file) => file
		};

		let mut base = ASTree::<Directive>::new();
		let lines = match Parser::read_lines(path) {
			Ok(lines) => lines,
			Err(why) => return Err(format!("Could not read \"{}\": {}", path.display(), why))
		};

		let mut command_lines = Vec::new();

		for line in lines {
			if !line.starts_with("#") {
				command_lines.push(format!("#text \"{}\"", line));
			} else {
				command_lines.push(line);
			}
		}

		for line in command_lines {
			base.add_child( match self.parse_line(line) {
				Ok(dir) => dir,
				Err(why) => Directive::Error(why)
			});
		}

		Ok(base)
	}

	fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>, {
	    let file = File::open(filename)?;
	    let lines = io::BufReader::new(file).lines();
	    let mut res = Vec::new();
	    for line in lines.flatten() {
	    	let line = line.trim();
	    	if line == "" { continue }
	    	res.push(line.to_string());
	    }

	    Ok(res)
	}

	fn parse_field(&mut self, query: &'static str) -> Result<ASTree<Directive>, String> {
		let field = match self.propers.get(query) {
			Some(val) => val,
			None => return Err(format!("Field \"{}\" was not set.", query))
		};

		match self.parse_file(field.to_path_buf()) {
			Ok(tree) => Ok(tree),
			Err(why) => Err(format!("Could not parse field \"{}\": {}", query, why))
		}
	}
}

pub fn parse_hour(propers: HashMap<&'static str, PathBuf>) -> ASTree<Directive> {
	let mut base = ASTree::<Directive>::new();

	let mut parser = Parser {
		propers
	};

	base.add_node(match parser.parse_field("order") {
		Ok(tree) => ASTNode::Tree(tree),
		Err(why) => ASTNode::Node(Directive::Error(why))
	});

	let mut preprocess = Preprocessor {
		parser,
		tree: base
	};

	preprocess.preprocess()
}