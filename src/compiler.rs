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
			ASTNode::Tree(tree) => res.append(&mut compile_ast(tree))
		};
	}

	res
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

		Directive::Instruction(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "instruction")]).with_paragraph(text),

		Directive::RawGabc(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "gabc-score")]).with_raw(text),

		Directive::Title(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "title")]).with_paragraph(text),

		Directive::Error(why) =>
			Container::new(ContainerType::Div).with_attributes(vec![("class", "error")]).with_paragraph(format!("Error: {}", why)),

		_ => compile_node(Directive::Error(format!("Unsupported node {:?}", node)))
	}
}