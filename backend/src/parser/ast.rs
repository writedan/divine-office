use std::clone::Clone;

#[derive(Clone)]
pub struct ASTree<T: Clone> {
    pub root: Option<T>,
    pub children: Vec<ASTNode<T>>,
}

#[derive(Debug, Clone)]
pub enum ASTNode<T: Clone> {
    Tree(ASTree<T>),
    Node(T),
}

pub trait Tree<T: Clone> {
    // fn add_child(&mut self, child: T);
    fn add_node(&mut self, child: ASTNode<T>);
    // fn add_subtree(&mut self, child: ASTree<T>);
    fn children(&self) -> Vec<ASTNode<T>>;
}

impl<T: Clone> Tree<T> for ASTree<T> {
    // fn add_child(&mut self, child: T) {
    //     self.children.push(ASTNode::Node(child));
    // }

    fn add_node(&mut self, child: ASTNode<T>) {
        self.children.push(child);
    }

    // fn add_subtree(&mut self, child: ASTree<T>) {
    //     for child in child.children {
    //         self.children.push(child);
    //     }
    // }

    fn children(&self) -> Vec<ASTNode<T>> {
        self.children.clone()
    }
}

impl<T: Clone> ASTree<T> {
    pub fn new() -> ASTree<T> {
        ASTree {
            root: None,
            children: Vec::new(),
        }
    }

    pub fn from_root(node: T) -> ASTree<T> {
        ASTree {
            root: Some(node),
            children: Vec::new(),
        }
    }
}

impl<T: std::fmt::Debug + Clone> std::fmt::Debug for ASTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_tree<T: std::fmt::Debug + Clone>(
            tree: &ASTree<T>,
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
        ) -> std::fmt::Result {
            let indent = "  ".repeat(depth * 2);

            for child in &tree.children {
                match child {
                    ASTNode::Node(value) => writeln!(f, "{}{:?}", indent, value)?,
                    ASTNode::Tree(subtree) => {
                        writeln!(f, "{}{:?}", indent, subtree.root)?;
                        fmt_tree(subtree, f, depth + 1)?;
                    }
                }
            }

            Ok(())
        }

        fmt_tree(self, f, 0)
    }
}
