use crate::parser::Directive;
use crate::parser::ast::*;
use std::path::PathBuf;

use build_html::{HtmlContainer, Container, ContainerType};

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
			let text = text.replace("*", "*<br/>&nbsp;&nbsp;&nbsp;&nbsp;");
			Container::new(ContainerType::Div).with_paragraph(text)
		},

		Directive::Heading(text) => Container::new(ContainerType::Div).with_header(2, text),

		Directive::Instruction(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "instruction")]).with_paragraph(text),

		Directive::RawGabc(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "gabc")]).with_raw(text),

		Directive::Title(text) => Container::new(ContainerType::Div).with_attributes(vec![("class", "title")]).with_paragraph(text),

		Directive::Error(why) =>
			Container::new(ContainerType::Div).with_attributes(vec![("class", "error")]).with_paragraph(format!("Error: {}", why)),

		_ => compile_node(Directive::Error(format!("Unsupported node {:?}", node)))
	}
}