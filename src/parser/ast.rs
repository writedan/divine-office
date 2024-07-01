use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct ASTree<T: Clone> {
	pub root: Option<T>,
	pub children: Vec<ASTNode<T>>
}

#[derive(Debug, Clone)]
pub enum ASTNode<T: Clone> {
	Tree(ASTree<T>),
	Node(T)
}

pub trait Tree<T: Clone> {
	fn add_child(&mut self, child: T);
	fn add_node(&mut self, child: ASTNode<T>);
	fn add_subtree(&mut self, child: ASTree<T>);
	fn children(&self) -> Vec<ASTNode<T>>;
}

impl<T: Clone> Tree<T> for ASTree<T> {
	fn add_child(&mut self, child: T) {
		self.children.push(ASTNode::Node(child));
	}

	fn add_node(&mut self, child: ASTNode<T>) {
		match child {
			ASTNode::Tree(tree) => self.add_subtree(tree),
			ASTNode::Node(node) => self.add_child(node)
		};
	}

	fn add_subtree(&mut self, child: ASTree<T>) {
		for child in child.children {
			self.children.push(child);
		}
	}

	fn children(&self) -> Vec<ASTNode<T>> {
		self.children.clone()
	}
}

impl<T: Clone> ASTree<T> {
	pub fn new() -> ASTree<T> {
		ASTree {
			root: None,
			children: Vec::new()
		}
	}
}