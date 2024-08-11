pub mod ast;





use std::path::PathBuf;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;


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
	Heading(String, u8),
	Instruction(String),
	Gabc(String, bool, String), // musicm whether english (true) or latin (false) -- default to english, initial style
	RawGabc(String),
	Import(PathBuf),
	Title(String),
	Error(String),
}

struct Parser {
	propers: HashMap<&'static str, PathBuf>,
	reserve: HashMap<&'static str, String>
}

struct Preprocessor {
	parser: Parser,
	tree: ASTree<Directive>
}

fn resolve_tone(tone: &String) -> String {
	let parts = tone.split('-').collect::<Vec<&str>>();
	let median = parts[0];
	let ending = parts.get(1);

	let tone = match ending {
		Some(s) => format!("{}-{}", median, s),
		None => median.to_string()
	};

	match median {
		"1" => "8".to_string(),
		"6" => "8".to_string(),
		"2" => match ending {
			Some(&"i") => "2".to_string(),
			Some(&"ii") => "8".to_string(),
			_ => tone.to_string()
		},
		"3" => match ending {
			Some(&"i") | Some(&"ii") | Some(&"iii") => "3a".to_string(),
			Some(&"iv") | Some(&"v") => "3b".to_string(),
			Some(&"vi") => "3c".to_string(),
			_ => tone.to_string()
		},
		"4" => match ending {
			Some(&"i") | Some(&"ii") | Some(&"iii") | Some(&"iv") | Some(&"v") => "4a".to_string(),
			Some(&"vi") => "4b".to_string(),
			Some(&"vii") | Some(&"viii") | Some(&"ix") => "4c".to_string(),
			_ => tone.to_string()
		},
		"5" => match ending {
			Some(&"i") | Some(&"ii") => "5".to_string(),
			Some(&"iii") => "8".to_string(),
			_ => tone.to_string()
		},
		_ => median.to_string()
	}
}

impl Preprocessor {
	fn preprocess(&mut self) -> ASTree<Directive> {
		self.parser.reserve.remove("preprocess");
		let mut base = ASTree::<Directive>::new();
		for node in self.tree.children() {
			base.add_node(match node {
				ASTNode::Node(node) => self.preprocess_directive(node),
				ASTNode::Tree(tree) => self.preprocess_tree(tree)
			});
		}

		if self.parser.reserve.contains_key("preprocess") {
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

			Directive::Import(path) => {
				let ext = match path.as_path().extension() {
					Some(ext) => ext,
					None => return ASTNode::Node(Directive::Error(format!("Unable to resolve \"{}\" because of the missing extension.", path.display())))
				};

				if ext == "gabc" {
					let music = Parser::read_lines(path.clone());
					match music {
						Ok(music) => ASTNode::Node(Directive::RawGabc(music.join("\n"))),
						Err(why) => ASTNode::Node(Directive::Error(format!("Unable to load score \"{}\": {}", path.display(), why)))
					}
				} else {
					match self.parser.parse_file(path) {
						Ok(tree) => ASTNode::Tree(tree),
						Err(why) => ASTNode::Node(Directive::Error(why))
					}
				}
			},
			_ => ASTNode::Node(dir)
		}
	}

	fn preprocess_tree(&self, _tree: ASTree<Directive>) -> ASTNode<Directive> {
		todo!()
	}
}

impl Parser {

	fn parse_line(&mut self, line: String) -> Result<Vec<Directive>, String> {
		self.reserve.insert("preprocess", "true".into());

		if let Some(captures) = RE.captures(&line) {
			let command = captures.get(1).map_or("", |m| m.as_str());
			let arg1 = captures.get(2).map_or("", |m| m.as_str()).to_string();
			let arg2 = captures.get(3).map_or("", |m| m.as_str()).to_string();
			let arg3 = captures.get(4).map_or("", |m| m.as_str()).to_string();

			match command {
				"no-gloria" => {
					self.reserve.insert("no-gloria", "enabled".to_string());
					Ok(Vec::new())
				},
				"gloria" => {
					if self.reserve.contains_key("no-gloria") {
						self.reserve.remove("no-gloria");
						return Ok(Vec::new());
					}

					let tone = if arg1.is_empty() {
						match self.reserve.get("previous-tone") {
							Some(path) => path,
							None => return Err(format!("No tone was previously declared."))
						}
					} else {
						&arg1
					};

					Ok(vec![Directive::Import(PathBuf::from(format!("commons/gloria/{}.lit", resolve_tone(tone))))])
				},
				"antiphon" => {
					let mut antiphon_path: PathBuf = ["antiphon", &arg1].iter().collect();
					antiphon_path.set_extension("gabc");
					self.reserve.insert("previous-antiphon", antiphon_path.display().to_string());
					Ok(vec![Directive::Import(antiphon_path)])
				},
				"repeat-antiphon" => {
					match self.reserve.get("previous-antiphon") {
						Some(path) => Ok(vec![Directive::Import(path.into())]),
						None => Err("No antiphon was previously declared".to_string())
					}
				},
				"repeat-tone" => {
					match self.reserve.get("previous-tone") {
						Some(tone) => {
							let mut tone_path: PathBuf = ["tone", &tone].iter().collect();
							tone_path.set_extension("gabc");
							Ok(vec![Directive::Import(tone_path)])
						},
						None => Err("No tone was previously declared".to_string())
					}
				},
				"tone" => {
					let mut tone_path: PathBuf = ["tone", &arg1].iter().collect();
					tone_path.set_extension("gabc");
					self.reserve.insert("previous-tone", arg1);
					Ok(vec![Directive::Import(tone_path)])
				},
				"psalm" => {
					let tone = match self.reserve.get("previous-tone") {
						Some(tone) => resolve_tone(tone),
						None => return Err(format!("No tone was previously declared."))
					};
					let mut psalm_path: PathBuf = ["psalter", &arg1, &tone].iter().collect();
					psalm_path.set_extension("lit");
					let mut vec: Vec<Directive> = Vec::new();
					if arg1.parse::<u8>().is_ok() {
						vec.push(Directive::Title(format!("Psalm {}", arg1)));
					}
					vec.push(Directive::Import(psalm_path));
					Ok(vec)
				},
				"text" => Ok(vec![Directive::Text(arg1)]),
				"heading" => Ok(vec![Directive::Heading(arg1, 2)]),
				"subheading" => Ok(vec![Directive::Heading(arg1, 3)]),
				"instruction" => Ok(vec![Directive::Instruction(arg1)]),
				"gabc" => Ok(vec![Directive::Gabc(arg1, arg2 == "english" || arg2.is_empty(), if arg3.is_empty() { "0".to_string() } else { arg3 })]),
				"include" => Ok(vec![Directive::Import(self.resolve_field(arg1)?)]),
				"import" => Ok(vec![Directive::Import(arg1.into())]),
				"title" => Ok(vec![Directive::Title(arg1)]),
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

		let mut base = ASTree::<Directive>::new();
		let lines = match Parser::read_lines(path) {
			Ok(lines) => lines,
			Err(why) => return Err(format!("Could not read \"{}\": {}", path.display(), why))
		};

		for line in lines {
			if (!line.starts_with('#')) {
				base.add_node(ASTNode::Node(Directive::Text(line)));
			} else {
				base.add_node( match self.parse_line(line) {
					Ok(dirs) => {
						let mut base = ASTree::<Directive>::new();
						for d in dirs {
							base.add_child(d);
						}

						ASTNode::Tree(base)
					},
					Err(why) => ASTNode::Node(Directive::Error(why))
				});
			}
		}

		Ok(base)
	}

	fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>, {
	    let file = File::open(filename)?;
	    let lines = io::BufReader::new(file).lines();
	    let mut res = Vec::new();
	    for line in lines.flatten() {
	    	let line = line.trim();
	    	if line.is_empty() { continue }
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
		propers,
		reserve: HashMap::new()
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