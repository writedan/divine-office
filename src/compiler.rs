use crate::parser::Directive;
use crate::parser::ast::*;

use build_html::{HtmlContainer, Container, ContainerType};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
	static ref SmallPrint: Regex = Regex::new(r"\{([^{}]*)\}").unwrap();
	static ref Vowel: Regex = Regex::new(r"([aeiouAEIOU])").unwrap();
	static ref Y: Regex = Regex::new(r"([yY])").unwrap();
	static ref Intone: Regex = Regex::new(r"\(([^()]*)\)").unwrap();
	static ref Flex: Regex = Regex::new(r"\^([^^]*)\^").unwrap();
	static ref Mediant: Regex = Regex::new(r"\~([^~]*)\~").unwrap();
	static ref Accent: Regex = Regex::new(r"\`([^`]*)\`").unwrap();
}

fn style_first_vowel(text: &str, sym: &str, style: &str) -> String {
	if Vowel.is_match(text) {
		format!("<{}>{}</{}>", style, Vowel.replace(text, |caps: &regex::Captures| {
			format!("{}{}", caps[1].to_string(), sym.to_string())
		}).into_owned(), style)
	} else {
		format!("<{}>{}</{}>", style, Y.replace(text, |caps: &regex::Captures| {
			format!("{}{}", caps[1].to_string(), sym.to_string())
		}).into_owned(), style)
	}
}

pub fn compile_ast(tree: ASTree<Directive>) -> Vec<Container> {
	let mut res = Vec::new();

	for child in tree.children() {
		match child {
			ASTNode::Node(dir) => res.push(compile_node(dir)),
			ASTNode::Tree(tree) => {
				let mut tree = match tree.root {
					Some(ref root) => vec![compile_tree(tree)],
					None => compile_ast(tree)
				};

				res.append(&mut tree);
			}
		}
	}

	res
}

fn compile_dispatch(node: ASTNode<Directive>) -> Container {
	match node {
		ASTNode::Node(directive) => compile_node(directive),
		ASTNode::Tree(tree) => compile_tree(tree)
	}
}

fn compile_tree(tree: ASTree<Directive>) -> Container {
	match tree.root {
		Some(Directive::Box) => {
			let mut cont = Container::new(ContainerType::Div).with_attributes(vec![("class", "boxed")]);
			for node in tree.children() {
				cont.add_container(compile_dispatch(node));
			}

			cont
		},

		_ => compile_node(Directive::Error(format!("Unsupported tree root directive {:?}", tree.root)))
	}
}

fn compile_node(node: Directive) -> Container {
	match node {
		Directive::Text(text) => {
			let text = text.replace('*', "<span class='symbol'>*</span><br/>&nbsp;&nbsp;&nbsp;&nbsp;")
			.replace("+++", "<span class='symbol'>✠</span>")
			.replace('+', "<span class='symbol'>+</span><br/>");

			let text = SmallPrint.replace_all(&text, "<span class='instr'>$1</span>");

			let text = Intone.replace_all(&text, |caps: &regex::Captures| {
				style_first_vowel(&caps[1], "\u{030A}", "span")
			});

			let text = Flex.replace_all(&text, |caps: &regex::Captures| {
				style_first_vowel(&caps[1], "\u{0302}", "i")
			});

			let text = Mediant.replace_all(&text, |caps: &regex::Captures| {
				style_first_vowel(&caps[1], "\u{0303}", "u")
			});
			
			let text = Accent.replace_all(&text, |caps: &regex::Captures| {
				style_first_vowel(&caps[1], "\u{0301}", "b")
			});

			Container::new(ContainerType::Div).with_paragraph(text)
		},

		Directive::Heading(text, level) => Container::new(ContainerType::Div).with_header(level, text),

		Directive::Hymn(hymn) => {
			let mut buffer = format!("initial-style: 1;\nannotation: Hymn.;\ncentering-scheme: english;\n%%\n({})", hymn.clef);
			for stanza_idx in 0..(hymn.verses.len() / hymn.melody.len()) {
				for verse_idx in 0..hymn.melody.len() {
					let verse = &hymn.verses[hymn.verse_idx(stanza_idx, verse_idx)];

					if verse_idx == 0 && stanza_idx > 0 {
						buffer = format!("{} (::) {}. ", buffer, stanza_idx + 1);
					}

					for (idx, syllable) in verse.into_iter().enumerate() {
						let continuous = syllable.ends_with('-');
						let syllable = if continuous {syllable[0..syllable.len() - 1].to_string()} else {format!("{} ", syllable)};
						buffer = format!("{}{}({})", buffer, syllable, hymn.melody[verse_idx][idx]);
					}

					if verse_idx == hymn.melody.len() - 1 { continue; }

					buffer = format!("{} {}({})", buffer, if stanza_idx == 0 && verse_idx == 0 {"<sp>*</sp>"} else {""}, if verse_idx % 2  == 0 { "," } else { ";" }); 
				}
			}

			buffer = format!("{} (::) A({})men.({})", buffer, hymn.amen.0, hymn.amen.1);

			compile_node(Directive::RawGabc(buffer))
		},

		Directive::Instruction(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "instruction")]).with_paragraph(text),

		Directive::RawGabc(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "gabc-score")]).with_raw(text),

		Directive::Title(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "title")]).with_paragraph(if text.ends_with('.') { text } else { format!("{}.", text) }),

		Directive::Error(why) =>
			Container::new(ContainerType::Div).with_attributes(vec![("class", "error")]).with_paragraph(format!("Error: {}", why)),

		Directive::Empty => Container::new(ContainerType::Div).with_attributes(vec![("class", "empty")]).with_paragraph("empty"),

		_ => compile_node(Directive::Error(format!("Unsupported node {:?}", node)))
	}
}