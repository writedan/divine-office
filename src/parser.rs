pub mod ast;





use std::path::PathBuf;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;

use std::rc::Rc;
use std::cell::RefCell;


use regex::Regex;

use lazy_static::lazy_static;

use crate::parser::ast::*;

lazy_static! {
    static ref WHOLE_RE: Regex = Regex::new(
        r#"#([\w-]+)(?:\s+"([^"]*)")*"#
    ).unwrap();

    static ref COMMAND_RE: Regex = Regex::new(r#""([^"]*)""#).unwrap();
}

trait ErrGet<T> {
	fn get_err(&self, idx: usize) -> Result<&T, String>;
}

impl<T> ErrGet<T> for Vec<T> {
	fn get_err(&self, idx: usize) -> Result<&T, String> {
		match self.get(idx) {
			Some(r) => Ok(&r),
			None => Err(format!("attempted to get vec[{}] but len={}", idx, self.len()))
		}
	}
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
	Box,

	MakeHymn(String, (String, String)), // clef, (a, men)
	MakeVerse(Vec<String>, Vec<Vec<String>>), // melody, vec<verse>

	// parser internal use only

	Hymn,
	Clef(String),
	Melody(Vec<String>),
	Verse(Vec<String>),
	Amen(String, String),

	EndHymn,
	EndBox,
	Empty
}

struct Parser {
	propers: HashMap<&'static str, PathBuf>,
	reserve: HashMap<&'static str, String>,
	lines: Option<Rc<RefCell<dyn ExactSizeIterator<Item = String>>>>
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

	fn preprocess_tree(&mut self, tree: ASTree<Directive>) -> ASTNode<Directive> {
		match tree.root {
			Some(Directive::Hymn) => {
				#[derive(Debug)]
				struct Hymn {
					clef: String,
					melody: Vec<Vec<String>>, // Vec<Melody>
					verses: Vec<Vec<Vec<String>>>, // Vec<Vec<Verse>> (Vec<Vec> corresponds to Melody)
					amen: (String, String)
				}

				let mut hymn = Hymn {
					clef: "".to_string(),
					verses: Vec::new(),
					melody: Vec::new(),
					amen: ("".to_string(), "".to_string())
				};

				let mut iter = tree.children().into_iter();

				while let Some(node) = iter.next() {
					if let ASTNode::Node(directive) = node {
						use Directive::*;
						match directive {
							Clef(clef) => hymn.clef = clef,

							Melody(notes) => {
								hymn.melody.push(notes);
								hymn.verses.push(Vec::new());
							},

							Verse(syllables) => {
								match hymn.verses.last_mut() {
									Some(verses) => {
										verses.push(syllables);
									},
									None => return ASTNode::Node(Directive::Error(format!("No melody was declared but tried to provide verse.")))
								}
							},

							Amen(n1, n2) => {
								hymn.amen = (n1, n2);
							}

							_ => return ASTNode::Node(Directive::Error(format!("Unsupported directive {:?} in hymn compilation", directive)))
						}
					} else {
						return ASTNode::Node(Directive::Error(format!("Unsupported node {:?} in hymn compilation", node)))
					}
				}

				if hymn.melody.len() == 0 || hymn.verses.len() == 0 {
					return ASTNode::Node(Directive::Error(format!("Hymn has no melody or verses.")));
				}

				let mut base = ASTree::<Directive>::from_root(Directive::MakeHymn(hymn.clef, hymn.amen));

				let standard_len = hymn.verses[0].len();
				for (idx, melody) in hymn.melody.into_iter().enumerate() {
					if hymn.verses[idx].len() == 0 {
						return ASTNode::Node(Directive::Error(format!("Melody has no corresponding verses for stanza {}", idx + 1)))
					}

					for verse in hymn.verses[idx].iter() {
						if verse.len() != melody.len() {
							return ASTNode::Node(Directive::Error(format!("Melody and verse have differing syllable counts for stanza {} on verse {:?}", idx + 1, verse)));
						}
					}

					if hymn.verses[idx].len() != standard_len {
						return ASTNode::Node(Directive::Error(format!("Stanza {} has differing number of verses from first stanza.", idx + 1)));
					}

					base.add_node(ASTNode::Node(Directive::MakeVerse(melody, hymn.verses[idx].clone())));
				}

				ASTNode::Tree(base)
			},

			_ => {
				let mut newtree = ASTree::<Directive>::new();
				for node in tree.children() {
					newtree.add_node(match node {
						ASTNode::Node(directive) => self.preprocess_directive(directive),
						ASTNode::Tree(tree) => self.preprocess_tree(tree)
					});
				}
				
				newtree.root = tree.root;
				ASTNode::Tree(newtree)
			}
		}
	}
}

impl Parser {

	fn parse_line(&mut self, line: String) -> Result<ASTNode<Directive>, String> {
		self.reserve.insert("preprocess", "true".into());

		let mut args = Vec::new();
		if let Some(captures) = WHOLE_RE.captures(&line) {

	        // Find all arguments
	        for arg in COMMAND_RE.find_iter(&line) {
	            args.push((line[arg.start() + 1..arg.end() - 1]).to_string().clone());
	        }
	    }

		if let Some(captures) = WHOLE_RE.captures(&line) {
			let command = captures.get(1).map_or("", |m| m.as_str());

			match command {
				"begin-hymn" => {
					let mut hymnbox = ASTree::<Directive>::from_root(Directive::Hymn);
					loop {
						let next = self.parse_next_line()?;
						if let ASTNode::Node(Directive::EndHymn) = next {
							break;
						} else {
							hymnbox.add_node(next);
						}
					}

					Ok(ASTNode::Tree(hymnbox))
				},

				"end-hymn" => Ok(ASTNode::Node(Directive::EndHymn)),

				"clef" => Ok(ASTNode::Node(Directive::Clef(args.get_err(0)?.clone()))),
				"melody" => Ok(ASTNode::Node(Directive::Melody(args.clone()))),
				"verse" => Ok(ASTNode::Node(Directive::Verse(args.clone()))),
				"amen" => Ok(ASTNode::Node(Directive::Amen(args.get_err(0)?.clone(), args.get_err(1)?.clone()))),

				"begin-box" => {
					let mut boxbase = ASTree::<Directive>::from_root(Directive::Box);

					loop {
						let next = self.parse_next_line()?;
						if let ASTNode::Node(Directive::EndBox) = next {
							break;
						} else {
							boxbase.add_node(next);
						}
					}

					Ok(ASTNode::Tree(boxbase))
				},
				"end-box" => {
					Ok(ASTNode::Node(Directive::EndBox))
				},
				"no-gloria" => {
					self.reserve.insert("no-gloria", "enabled".to_string());
					Ok(ASTNode::Node(Directive::Empty))
				},
				"gloria" => {
					if self.reserve.contains_key("no-gloria") {
						self.reserve.remove("no-gloria");
						return Ok(ASTNode::Node(Directive::Empty));
					}

					let tone = if args.len() == 0 {
						match self.reserve.get("previous-tone") {
							Some(path) => path,
							None => return Err(format!("No tone was previously declared."))
						}
					} else {
						&args[0]
					};

					Ok(ASTNode::Node(Directive::Import(PathBuf::from(format!("commons/gloria/{}.lit", resolve_tone(tone))))))
				},
				"antiphon" => {
					let mut antiphon_path: PathBuf = ["antiphon", &args.get_err(0)?.clone()].iter().collect();
					antiphon_path.set_extension("gabc");
					self.reserve.insert("previous-antiphon", antiphon_path.display().to_string());

					let mut base = ASTree::<Directive>::new();
					base.add_node(ASTNode::Node(Directive::Title("Antiphon".to_string())));
					base.add_node(ASTNode::Node(Directive::Import(antiphon_path)));
					Ok(ASTNode::Tree(base))
				},
				"repeat-antiphon" => {
					match self.reserve.get("previous-antiphon") {
						Some(path) => Ok(ASTNode::Node(Directive::Import(path.into()))),
						None => Err("No antiphon was previously declared".to_string())
					}
				},
				"repeat-tone" => {
					match self.reserve.get("previous-tone") {
						Some(tone) => {
							let mut tone_path: PathBuf = ["tone", &tone].iter().collect();
							tone_path.set_extension("gabc");
							Ok(ASTNode::Node(Directive::Import(tone_path)))
						},
						None => Err("No tone was previously declared".to_string())
					}
				},
				"tone" => {
					let mut tone_path: PathBuf = ["tone", &args.get_err(0)?.clone()].iter().collect();
					tone_path.set_extension("gabc");
					self.reserve.insert("previous-tone", args.get_err(0)?.clone());
					Ok(ASTNode::Node(Directive::Import(tone_path)))
				},
				"psalm" => {
					let tone = match self.reserve.get("previous-tone") {
						Some(tone) => resolve_tone(tone),
						None => return Err(format!("No tone was previously declared."))
					};
					let mut psalm_path: PathBuf = ["psalter", &args.get_err(0)?.clone(), &tone].iter().collect();
					psalm_path.set_extension("lit");
					let mut vec: Vec<Directive> = Vec::new();
					if args[0].parse::<u8>().is_ok() {
						vec.push(Directive::Title(format!("Psalm {}", args.get_err(0)?.clone())));
					}
					vec.push(Directive::Import(psalm_path));
					
					let mut base = ASTree::<Directive>::new();
					for i in vec {
						base.add_node(ASTNode::Node(i));
					}

					Ok(ASTNode::Tree(base))
				},
				"text" => Ok(ASTNode::Node(Directive::Text(args.get_err(0)?.clone()))),
				"heading" => Ok(ASTNode::Node(Directive::Heading(args.get_err(0)?.clone(), 2))),
				"subheading" => Ok(ASTNode::Node(Directive::Heading(args.get_err(0)?.clone(), 3))),
				"instruction" => Ok(ASTNode::Node(Directive::Instruction(args.get_err(0)?.clone()))),
				"gabc" => Ok(ASTNode::Node(Directive::Gabc(args.get_err(0)?.clone(), args.len() < 2 || args[1] == "english", if args.len() < 3 { "0".to_string() } else { args[2].clone() }))),
				"include" => Ok(ASTNode::Node(Directive::Import(self.resolve_field(args.get_err(0)?.clone())?))),
				"import" => Ok(ASTNode::Node(Directive::Import(args.get_err(0)?.clone().into()))),
				"title" => Ok(ASTNode::Node(Directive::Title(args.get_err(0)?.clone()))),
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

	fn parse_next_line(&mut self) -> Result<ASTNode<Directive>, String> {
		let line = match &self.lines {
			Some(lines) => {
				Rc::clone(&lines).borrow_mut().next()
			},

			None => panic!("Parser.lines must be set by now")
		}.unwrap();

		let line = line.to_owned();
		if !line.starts_with('#') {
			Ok(ASTNode::Node(Directive::Text(line)))
		} else {
			self.parse_line(line)
		}
	}

	fn parse_file(&mut self, path: PathBuf) -> Result<ASTree<Directive>, String> {
		let path = path.as_path();

		let mut base = ASTree::<Directive>::new();
		let lines = match Parser::read_lines(path) {
			Ok(lines) => lines,
			Err(why) => return Err(format!("Could not read \"{}\": {}", path.display(), why))
		};

		let iter = lines.clone().into_iter(); // Create an iterator over the lines
		self.lines = Some(Rc::new(RefCell::new(iter)));

		loop {
			let tree = match self.parse_next_line() {
				Ok(tree) => tree,
				Err(why) => ASTNode::Node(Directive::Error(why))
			};

			base.add_node(tree);

			let cont = match &self.lines {
				Some(lines) => {
					let rc = Rc::clone(&lines);
					let iter = rc.borrow();
					iter.len() > 0
				},
				None => panic!("Parser.lines must be set by now.")
			};

			if !cont { break; }
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
		reserve: HashMap::new(),
		lines: None
	};

	base.add_node(match parser.parse_field("order") {
		Ok(tree) => ASTNode::Tree(tree),
		Err(why) => ASTNode::Node(Directive::Error(why))
	});

	let mut preprocess = Preprocessor {
		parser,
		tree: base,
	};

	preprocess.preprocess()
}